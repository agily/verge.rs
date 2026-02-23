use anyhow::{Result, anyhow};
use openapiv3::OpenAPI;

use std::collections::BTreeMap;

// Progenitor requires Operation ID to be specified for every operation
// on every path. This is "holier than thou" compared to the OpenAPI spec
// The code below creates operation IDs from paths, methods and optionally
// path parameters
// The bulk of this code is stolen from
// https://github.com/upachler/progenitor/blob/1105-progenitor-requires-operationid-to-be-set/progenitor-impl/src/opid.rs
//
//
/// newtype for encapsulating the combination of path and method,
/// which can uniquely identify a HTTP endpoint. The struct is
/// designed to be used as a key for map implementations
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PathMethod {
    path: String,
    method: String,
    params: Option<Vec<String>>,
}

impl PathMethod {
    /// Create new PathMethod. This may fail if path or method
    /// are empty.
    pub fn new(path: &str, method: &str, params: Option<Vec<&str>>) -> Result<Self> {
        // disallow empty path/method
        if path.is_empty() || method.is_empty() {
            return Err(anyhow!("path and method may not be empty",));
        }

        // NOTE: In the future, we may consider checking for the proper URL path
        // format in the the future according to the RFC:
        // https://datatracker.ietf.org/doc/html/rfc3986#section-3.3

        Ok(Self {
            path: path.to_string(),
            method: method.to_string(),
            params: params.map(|v| v.iter().map(|s| s.to_string()).collect()),
        })
    }
}

/// Store for a one to one mapping between OAS operation IDs and
/// path/method pairs. The store
/// supports lookup in each direction.
#[derive(Default, Debug)]
pub struct OperationIds {
    opid_to_path_method: BTreeMap<String, PathMethod>,
    path_method_to_opid: BTreeMap<PathMethod, String>,
}

/// Extract path parameters (like {id}) from a path string
/// and remove them from the path string. If parameters were found,
/// return a vector of them and the modified path. Otherwise, [`None`]
fn extract_params(path: &str) -> Option<(Vec<&str>, String)> {
    let mut params = Vec::new();
    let mut clean_path = String::with_capacity(path.len());
    let mut last_end = 0;
    let mut in_param = false;
    let mut param_start = 0;

    for (i, c) in path.char_indices() {
        if c == '{' && !in_param {
            in_param = true;
            clean_path.push_str(&path[last_end..i]);
            clean_path.push_str("{}");
            param_start = i + 1;
        } else if c == '}' && in_param {
            in_param = false;
            if i > param_start {
                params.push(&path[param_start..i]);
            }
            last_end = i + 1;
        }
    }

    // Add any remaining part after the last parameter
    if last_end < path.len() {
        clean_path.push_str(&path[last_end..]);
    }

    if params.is_empty() {
        None
    } else {
        Some((params, clean_path))
    }
}

impl OperationIds {
    /// Find operation ID for given path and method. Returns [`None`] if
    /// no operation ID was found
    pub fn opid_for_path_method(&self, path: &str, method: &str) -> Option<&str> {
        let key = match extract_params(path) {
            Some((params, normalized_path)) => {
                match PathMethod::new(&normalized_path, method, Some(params)) {
                    Ok(path_method) => path_method,
                    Err(_) => return None,
                }
            }
            None => match PathMethod::new(path, method, None) {
                Ok(path_method) => path_method,
                Err(_) => return None,
            },
        };

        self.path_method_to_opid.get(&key).map(|s| s.as_str())
    }

    /// Find path and method for a given operation ID. Returns [`None`] if
    /// no path/method combination was found for the given operation ID
    pub fn path_method_for_opid(&self, operation_id: &str) -> Option<(&str, &str)> {
        self.opid_to_path_method
            .get(operation_id)
            .map(|path_method| (path_method.path.as_str(), path_method.method.as_str()))
    }

    /// Generate a new operation ID candidate for the given PathMethod, considering
    /// the number of attempts that have already been made. The number of attempts
    /// is included in the candiate name (unless it is 0), to help resolve name
    /// collisions.
    /// For generated operation IDs that would start with a number,
    /// the character 'n' is prepended.
    ///
    /// The operation_id names are created with the pattern
    /// `converted_path [attempt] converted_method`.
    ///
    /// For `GET /foo/bar`, this will yield an operation ID of `foo_bar_get`.
    /// It was deliberately chosen to have the method part at the end of the
    /// generated operation ID string, so that the main point of destinction
    /// is the path.
    /// This is useful when the operation ID is used to
    /// generate client method names: `foo_bar_get` and `foo_bar_post` will
    /// be listed next to each other in a method name list.
    fn gen_operation_id(path_method: &PathMethod, attempt: u32) -> String {
        let mut opid: String = path_method
            .path
            .replace(|c: char| !c.is_alphanumeric(), "_")
            .trim_matches('_')
            .to_lowercase();
        if opid.starts_with(char::is_numeric) {
            opid.insert(0, 'n');
        }

        let m = path_method.method.to_lowercase();
        if attempt == 0 {
            opid += &format!("_{m}");
        } else {
            opid += &format!("{attempt}_{m}");
        };

        if let Some(params) = &path_method.params {
            params.iter().for_each(|p| {
                opid += &format!("_by_{}", p.to_lowercase());
            });
        }
        opid
    }

    /// Insert a new operation ID with with it's path and method attached.
    /// The method will fail if the operation ID, or the path and method
    /// combination already exist in this [`OperationIds`] instance.
    pub fn insert_opid_with_path_method(
        &mut self,
        operation_id: &str,
        path: &str,
        method: &str,
    ) -> Result<()> {
        let key = match extract_params(path) {
            Some((params, normalized_path)) => {
                PathMethod::new(&normalized_path, method, Some(params))?
            }
            None => PathMethod::new(path, method, None)?,
        };

        if self.opid_to_path_method.contains_key(operation_id) {
            return Err(anyhow!("operation id is already present: {operation_id:?}"));
        }
        if self.path_method_to_opid.contains_key(&key) {
            return Err(anyhow!(
                "the combination of path {} and method {} is already present",
                key.path,
                key.method
            ));
        }

        self.opid_to_path_method
            .insert(operation_id.to_string(), key.clone());
        self.path_method_to_opid
            .insert(key, operation_id.to_string());
        Ok(())
    }

    /// Insert a generated opid for the given path and method combination.
    /// The method will choose an operation ID that does not collide
    /// with pre existing operation IDs in this [`OperationIds`] instance.
    /// The method will fail if the given path and methoc combination already
    /// exists.
    /// Returns synthetic operation ID
    pub fn insert_synthetic_opid_for_path_method(
        &mut self,
        path: &str,
        method: &str,
    ) -> Result<String> {
        let key = match extract_params(path) {
            Some((params, normalized_path)) => {
                PathMethod::new(&normalized_path, method, Some(params))?
            }
            None => PathMethod::new(path, method, None)?,
        };

        if self.path_method_to_opid.contains_key(&key) {
            return Err(anyhow!("operation id is already present: {key:?}"));
        }

        let mut candidate;
        let mut attempt = 0;

        loop {
            candidate = Self::gen_operation_id(&key, attempt);
            attempt += 1;
            if !self.opid_to_path_method.contains_key(&candidate) {
                break;
            }
        }

        self.path_method_to_opid
            .insert(key.clone(), candidate.clone());
        self.opid_to_path_method.insert(candidate.clone(), key);
        Ok(candidate)
    }
}

#[cfg(test)]
fn mk_pm(path: &str, method: &str) -> PathMethod {
    PathMethod::new(path, method, None).unwrap()
}

#[test]
fn test_extract_params() {
    assert_eq!(extract_params("/foo/bar"), None);
    assert_eq!(
        extract_params("/foo/{bar}/baz"),
        Some((vec!["bar"], String::from("/foo/{}/baz")))
    );
    assert_eq!(
        extract_params("/foo/{bar}/baz/{quux}"),
        Some((vec!["bar", "quux"], String::from("/foo/{}/baz/{}")))
    );
    assert_eq!(
        extract_params("/{foo}/{bar}"),
        Some((vec!["foo", "bar"], String::from("/{}/{}")))
    );
}

#[test]
fn test_operation_id_generation() {
    assert_eq!(
        OperationIds::gen_operation_id(&mk_pm("/foo/bar", "get"), 0),
        "foo_bar_get"
    );
    assert_eq!(
        OperationIds::gen_operation_id(&mk_pm("/foo/bar", "get"), 1),
        "foo_bar1_get"
    );
    assert_eq!(
        OperationIds::gen_operation_id(&mk_pm("/some.json", "get"), 0),
        "some_json_get"
    );
}

#[test]
fn test_operation_ids() {
    let mut opids = OperationIds::default();

    // insert
    opids
        .insert_opid_with_path_method("foo_get", "/foo", "get")
        .unwrap();
    assert_eq!(opids.opid_for_path_method("/foo", "get"), Some("foo_get"));
    assert_eq!(opids.path_method_for_opid("foo_get"), Some(("/foo", "get")));
    opids
        .insert_opid_with_path_method("foo_post", "/foo", "post")
        .unwrap();
    assert_eq!(opids.opid_for_path_method("/foo", "post"), Some("foo_post"));
    assert_eq!(
        opids.path_method_for_opid("foo_post"),
        Some(("/foo", "post"))
    );

    // insert must fail because of collision with operation id
    assert!(
        opids
            .insert_opid_with_path_method("foo_get", "/bar", "get")
            .is_err()
    );

    // insert must fail because of collision with path and method
    assert!(
        opids
            .insert_opid_with_path_method("bar_get", "/foo", "get")
            .is_err()
    );

    // now check we can create synthetic operation ids:
    assert!(
        opids
            .insert_synthetic_opid_for_path_method("/bar", "get")
            .is_ok()
    );
    assert_eq!(opids.opid_for_path_method("/bar", "get"), Some("bar_get"));
    assert_eq!(opids.path_method_for_opid("bar_get"), Some(("/bar", "get")));
    assert!(
        opids
            .insert_synthetic_opid_for_path_method("/bar", "post")
            .is_ok()
    );
    assert_eq!(opids.opid_for_path_method("/bar", "post"), Some("bar_post"));
    assert_eq!(
        opids.path_method_for_opid("bar_post"),
        Some(("/bar", "post"))
    );

    // test collisions.
    // we're going to collide with foo_bar_get
    opids
        .insert_opid_with_path_method("foo_bar_get", "/foobar", "get")
        .unwrap();
    opids
        .insert_synthetic_opid_for_path_method("/foo/bar", "get")
        .unwrap();
    assert_eq!(
        opids.opid_for_path_method("/foo/bar", "get"),
        Some("foo_bar1_get")
    );
    assert_eq!(
        opids.path_method_for_opid("foo_bar1_get"),
        Some(("/foo/bar", "get"))
    );
}

fn gen_operation_ids(spec: &mut OpenAPI) -> Result<()> {
    let mut opids = OperationIds::default();

    spec.paths
        .paths
        .iter_mut()
        .try_for_each(|(path, item)| -> Result<()> {
            if let Some(item) = item.as_item_mut() {
                item.iter_mut().try_for_each(|(method, op)| -> Result<()> {
                    if let Some(opid) = op.operation_id.as_ref() {
                        opids.insert_opid_with_path_method(opid, path, method)?;
                    } else {
                        let opid = opids.insert_synthetic_opid_for_path_method(path, method)?;
                        op.operation_id = Some(opid);
                    }
                    Ok(())
                })?;
            }
            Ok(())
        })
}

fn main() -> Result<()> {
    let src = "./generator/swagger/v4.json";
    let dst = "./generator/swagger/generated-opids.json";
    let in_file = std::fs::File::open(src)?;
    let out_file = std::fs::File::create_new(dst)?;
    let mut spec = serde_json::from_reader(in_file)?;
    gen_operation_ids(&mut spec)?;
    serde_json::to_writer_pretty(out_file, &spec)?;
    Ok(())
}
