use anyhow::{Context, Result};
use indexmap::IndexMap;
use quote::quote;
use std::fs;
use std::path::{Path, PathBuf};
use syn::{File, Item, ItemMod, Type};

fn split_file(input_path: &Path, output_dir: &Path) -> Result<()> {
    println!("Reading {}...", input_path.display());
    let content = fs::read_to_string(input_path)
        .with_context(|| format!("Failed to read {}", input_path.display()))?;
    
    println!("Parsing file with syn (this may take a moment)...");
    let ast = syn::parse_file(&content)
        .with_context(|| "Failed to parse Rust file")?;
    
    println!("Successfully parsed {} items", ast.items.len());
    
    // Categorize items
    let mut modules: IndexMap<String, Vec<Item>> = IndexMap::new();
    let mut types_module: Option<ItemMod> = None;
    let mut builder_module: Option<ItemMod> = None;
    let mut client_items = Vec::new();
    let mut common_items = Vec::new();
    
    println!("Categorizing items...");
    for item in ast.items {
        match &item {
            Item::Mod(m) if m.ident == "types" => {
                println!("  Found types module");
                types_module = Some(m.clone());
            }
            Item::Mod(m) if m.ident == "builder" => {
                println!("  Found builder module");
                builder_module = Some(m.clone());
            }
            Item::Struct(s) => {
                let name = s.ident.to_string();
                if name == "Client" {
                    client_items.push(item);
                } else if let Some(module) = extract_module_name(&name) {
                    modules.entry(module).or_default().push(item);
                } else {
                    common_items.push(item);
                }
            }
            Item::Enum(e) => {
                let name = e.ident.to_string();
                if let Some(module) = extract_module_name(&name) {
                    modules.entry(module).or_default().push(item);
                } else {
                    common_items.push(item);
                }
            }
            Item::Type(t) => {
                let name = t.ident.to_string();
                if let Some(module) = extract_module_name(&name) {
                    modules.entry(module).or_default().push(item);
                } else {
                    common_items.push(item);
                }
            }
            Item::Impl(i) => {
                if let Type::Path(type_path) = &*i.self_ty {
                    if let Some(segment) = type_path.path.segments.last() {
                        let name = segment.ident.to_string();
                        if name == "Client" {
                            client_items.push(item);
                        } else if let Some(module) = extract_module_name(&name) {
                            modules.entry(module).or_default().push(item);
                        } else {
                            common_items.push(item);
                        }
                    } else {
                        common_items.push(item);
                    }
                } else {
                    common_items.push(item);
                }
            }
            Item::Use(_) | Item::ExternCrate(_) => {
                common_items.push(item);
            }
            _ => {
                common_items.push(item);
            }
        }
    }
    
    println!("Found {} distinct modules", modules.len());
    
    // Create output directory
    fs::create_dir_all(output_dir)?;
    
    println!("Writing module files...");
    
    // Write types module
    if let Some(types_mod) = &types_module {
        println!("  Writing types.rs...");
        let types_path = output_dir.join("types.rs");
        
        // Extract the content from the module
        if let Some((_, items)) = &types_mod.content {
            let tokens = quote! { #(#items)* };
            let file = syn::parse2::<File>(tokens)?;
            let formatted = prettyplease::unparse(&file);
            fs::write(types_path, formatted)?;
        }
    }
    
    // Write builder module
    if let Some(builder_mod) = &builder_module {
        println!("  Writing builder.rs...");
        let builder_path = output_dir.join("builder.rs");
        
        // Extract the content from the module
        if let Some((_, items)) = &builder_mod.content {
            let tokens = quote! { #(#items)* };
            let file = syn::parse2::<File>(tokens)?;
            let formatted = prettyplease::unparse(&file);
            fs::write(builder_path, formatted)?;
        }
    }
    
    // Write common module
    if !common_items.is_empty() {
        println!("  Writing common.rs ({} items)...", common_items.len());
        let common_path = output_dir.join("common.rs");
        write_items_to_file(&common_path, &common_items)?;
    }
    
    // Write client module
    if !client_items.is_empty() {
        println!("  Writing client.rs ({} items)...", client_items.len());
        let client_path = output_dir.join("client.rs");
        write_items_to_file(&client_path, &client_items)?;
    }
    
    // Write individual modules
    for (name, items) in &modules {
        if !items.is_empty() {
            println!("  Writing {}.rs ({} items)...", name, items.len());
            let module_path = output_dir.join(format!("{}.rs", name));
            write_items_to_file(&module_path, &items)?;
        }
    }
    
    // Generate new lib.rs
    println!("Writing lib.rs...");
    write_lib_file(
        output_dir, 
        &modules, 
        types_module.is_some(),
        builder_module.is_some(),
        !common_items.is_empty(),
        !client_items.is_empty()
    )?;
    
    Ok(())
}

fn extract_module_name(ident: &str) -> Option<String> {
    // Skip Client-related items
    if ident == "Client" || ident.starts_with("Client") {
        return None;
    }
    
    // Extract first word from CamelCase
    let mut result = String::new();
    let mut chars = ident.chars();
    
    if let Some(first) = chars.next() {
        if !first.is_uppercase() {
            return None;
        }
        result.push(first.to_lowercase().next().unwrap());
        
        for ch in chars {
            if ch.is_uppercase() {
                break;
            }
            result.push(ch.to_lowercase().next().unwrap());
        }
    }
    
    if result.is_empty() || result == "client" {
        None
    } else {
        Some(result)
    }
}

fn write_items_to_file(path: &Path, items: &[Item]) -> Result<()> {
    let tokens = quote! {
        #(#items)*
    };
    
    let file = syn::parse2::<File>(tokens)?;
    let formatted = prettyplease::unparse(&file);
    fs::write(path, formatted)?;
    
    Ok(())
}

fn write_lib_file(
    output_dir: &Path,
    modules: &IndexMap<String, Vec<Item>>,
    has_types: bool,
    has_builder: bool,
    has_common: bool,
    has_client: bool,
) -> Result<()> {
    let lib_path = output_dir.join("lib.rs");
    
    let mut module_names: Vec<String> = modules.keys()
        .filter(|k| !modules[*k].is_empty())
        .cloned()
        .collect();
    module_names.sort();
    
    let mut lib_content = String::new();
    lib_content.push_str("#![allow(unused_imports)]\n");
    lib_content.push_str("#![allow(clippy::all)]\n\n");
    
    // Module declarations
    if has_common {
        lib_content.push_str("pub mod common;\n");
    }
    if has_types {
        lib_content.push_str("pub mod types;\n");
    }
    if has_builder {
        lib_content.push_str("pub mod builder;\n");
    }
    if has_client {
        lib_content.push_str("pub mod client;\n");
    }
    
    for name in &module_names {
        lib_content.push_str(&format!("pub mod {};\n", name));
    }
    
    lib_content.push('\n');
    
    // Re-exports
    if has_common {
        lib_content.push_str("pub use common::*;\n");
    }
    if has_types {
        lib_content.push_str("pub use types::*;\n");
    }
    if has_builder {
        lib_content.push_str("pub use builder::*;\n");
    }
    if has_client {
        lib_content.push_str("pub use client::*;\n");
    }
    
    for name in &module_names {
        lib_content.push_str(&format!("pub use {}::*;\n", name));
    }
    
    fs::write(lib_path, lib_content)?;
    
    Ok(())
}

fn main() -> Result<()> {
    let input_path = PathBuf::from("sdk/src/lib.rs");
    let output_dir = PathBuf::from("sdk/src");
    
    if !input_path.exists() {
        eprintln!("Input file {} does not exist", input_path.display());
        eprintln!("Make sure to run the generator first");
        return Ok(());
    }
    
    // Backup original
    let backup_path = output_dir.join("lib.rs.original");
    if !backup_path.exists() {
        println!("Backing up original lib.rs...");
        fs::copy(&input_path, &backup_path)?;
    }
    
    println!("Starting full AST-based module splitter using syn and prettyplease...");
    println!("This will parse the entire file properly and split it into modules.\n");
    
    let start = std::time::Instant::now();
    split_file(&input_path, &output_dir)?;
    let duration = start.elapsed();
    
    println!("\nModule splitting complete in {:.2}s!", duration.as_secs_f64());
    println!("Original backed up as lib.rs.original");
    
    Ok(())
}