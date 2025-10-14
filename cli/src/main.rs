use anyhow::{anyhow, Context, Result};
use clap::{ArgAction, Parser};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, AUTHORIZATION};
use std::str::FromStr;
use verge_rs_sdk as sdk;

#[derive(Parser, Debug)]
#[command(name = "verge-cli", about = "Simple VergeIO CLI")] 
struct Args {
    /// Base URL of the VergeIO API, e.g. https://host/api
    #[arg(long, env = "VERGE_BASE_URL")]
    base_url: String,

    /// Tenant name
    #[arg(long, env = "VERGE_TENANT_NAME")]
    name: String,

    /// Optional description
    #[arg(long)]
    description: Option<String>,

    /// Optional owner (row/id or identifier string)
    #[arg(long)]
    owner: Option<String>,

    /// Optional initial password
    #[arg(long)]
    password: Option<String>,

    /// Optional help URL override
    #[arg(long)]
    help_url: Option<String>,

    /// Set tenant isolation on
    #[arg(long, action = ArgAction::SetTrue)]
    isolate: bool,

    /// Explicitly set expose_cloud_snapshots true
    #[arg(long, action = ArgAction::SetTrue)]
    expose_cloud_snapshots: bool,

    /// Add arbitrary header, format: Name:Value (repeatable)
    #[arg(long, action = ArgAction::Append)]
    header: Vec<String>,

    /// Bearer token; adds Authorization header
    #[arg(long, env = "VERGE_TOKEN")]
    token: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Build default headers
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    if let Some(token) = &args.token {
        let value = format!("Bearer {}", token);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&value).context("invalid token for Authorization header")?,
        );
    }
    for h in &args.header {
        let (name, value) = split_header(h).with_context(|| format!("invalid --header '{}': expected Name:Value", h))?;
        headers.insert(name, value);
    }

    // Build reqwest client and wrap in SDK client
    let http = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .context("building HTTP client")?;
    let client = sdk::Client::new_with_client(&args.base_url, http);

    // Compose tenants post body via builder
    let name = sdk::types::TenantsPostName::try_from(args.name.as_str())
        .context("invalid tenant name")?;
    let mut body = sdk::types::builder::TenantsPost::default().name(name);

    if let Some(desc) = args.description.as_deref() {
        let d = sdk::types::TenantsPostDescription::try_from(desc)
            .context("invalid description")?;
        body = body.description(Some(d));
    }
    if let Some(owner) = args.owner.as_deref() {
        body = body.owner(Some(owner.to_string()));
    }
    if let Some(pw) = args.password.as_deref() {
        let p = sdk::types::TenantsPostPassword::try_from(pw)
            .context("invalid password")?;
        body = body.password(Some(p));
    }
    if let Some(url) = args.help_url.as_deref() {
        body = body.help_url(url.to_string());
    }
    if args.isolate {
        body = body.isolate(true);
    }
    if args.expose_cloud_snapshots {
        body = body.expose_cloud_snapshots(true);
    }

    // Send create request
    let resp = client
        .tenants_post()
        .body_map(move |_b| body)
        .send()
        .await
        .map_err(|e| anyhow!("API error: {}", format_sdk_error(e)))?;

    let created: sdk::types::PostResponse = resp.into_inner();
    println!(
        "{}",
        serde_json::to_string_pretty(&created).unwrap_or_else(|_| "{}".to_string())
    );
    Ok(())
}

fn split_header(input: &str) -> Result<(HeaderName, HeaderValue)> {
    let (name, value) = input
        .split_once(':')
        .ok_or_else(|| anyhow!("missing colon"))?;
    let name = HeaderName::from_str(name.trim()).context("invalid header name")?;
    let value = HeaderValue::from_str(value.trim()).context("invalid header value")?;
    Ok((name, value))
}

fn format_sdk_error<E: std::fmt::Display>(e: E) -> String {
    e.to_string()
}

