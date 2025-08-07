use anyhow::{Context, Result};
use quote::quote;
use std::fs;
use std::path::{Path, PathBuf};
use syn::{File, Item, ItemMod};

/// Simple and safe module extraction that maintains exact compatibility
fn extract_modules(input_path: &Path, output_dir: &Path) -> Result<()> {
    println!("Reading {}...", input_path.display());
    let content = fs::read_to_string(input_path)
        .with_context(|| format!("Failed to read {}", input_path.display()))?;
    
    println!("Parsing file with syn (this may take a moment)...");
    let mut ast = syn::parse_file(&content)
        .with_context(|| "Failed to parse Rust file")?;
    
    println!("Successfully parsed {} items", ast.items.len());
    
    // Find and extract inline modules
    let mut types_module: Option<ItemMod> = None;
    let mut builder_module: Option<ItemMod> = None;
    let mut extracted_count = 0;
    
    println!("Extracting inline modules...");
    
    // First pass: identify modules to extract
    for item in &ast.items {
        if let Item::Mod(m) = item {
            if m.ident == "types" && m.content.is_some() {
                println!("  Found inline types module");
                types_module = Some(m.clone());
                extracted_count += 1;
            } else if m.ident == "builder" && m.content.is_some() {
                println!("  Found inline builder module");
                builder_module = Some(m.clone());
                extracted_count += 1;
            }
        }
    }
    
    if extracted_count == 0 {
        println!("No inline modules found to extract");
        return Ok(());
    }
    
    // Create output directory
    fs::create_dir_all(output_dir)?;
    
    // Write extracted modules as separate files
    if let Some(types_mod) = &types_module {
        println!("Writing types.rs...");
        let types_path = output_dir.join("types.rs");
        
        if let Some((_, items)) = &types_mod.content {
            // Write the inner content of the module
            let tokens = quote! { #(#items)* };
            let file = syn::parse2::<File>(tokens)?;
            let formatted = prettyplease::unparse(&file);
            fs::write(types_path, formatted)?;
        }
    }
    
    if let Some(builder_mod) = &builder_module {
        println!("Writing builder.rs...");
        let builder_path = output_dir.join("builder.rs");
        
        if let Some((_, items)) = &builder_mod.content {
            // Write the inner content of the module
            let tokens = quote! { #(#items)* };
            let file = syn::parse2::<File>(tokens)?;
            let formatted = prettyplease::unparse(&file);
            fs::write(builder_path, formatted)?;
        }
    }
    
    // Second pass: replace inline modules with module declarations
    let mut new_items = Vec::new();
    for item in ast.items {
        match &item {
            Item::Mod(m) if m.ident == "types" && types_module.is_some() => {
                // Replace inline module with declaration
                let mod_decl: Item = syn::parse_quote! {
                    pub mod types;
                };
                new_items.push(mod_decl);
            }
            Item::Mod(m) if m.ident == "builder" && builder_module.is_some() => {
                // Replace inline module with declaration
                let mod_decl: Item = syn::parse_quote! {
                    pub mod builder;
                };
                new_items.push(mod_decl);
            }
            _ => {
                // Keep all other items as-is
                new_items.push(item);
            }
        }
    }
    
    // Write the modified lib.rs
    println!("Writing updated lib.rs...");
    let lib_path = output_dir.join("lib.rs");
    
    ast.items = new_items;
    let tokens = quote! { #ast };
    let formatted = prettyplease::unparse(&syn::parse2(tokens)?);
    fs::write(lib_path, formatted)?;
    
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
    
    // Backup original if not already done
    let backup_path = output_dir.join("lib.rs.original");
    if !backup_path.exists() {
        println!("Backing up original lib.rs...");
        fs::copy(&input_path, &backup_path)?;
    } else {
        // Restore from backup to ensure clean state
        println!("Restoring from backup for clean extraction...");
        fs::copy(&backup_path, &input_path)?;
    }
    
    println!("Starting safe module extraction using syn and prettyplease...");
    println!("This will extract inline modules without changing any namespaces.\n");
    
    let start = std::time::Instant::now();
    extract_modules(&input_path, &output_dir)?;
    let duration = start.elapsed();
    
    println!("\nModule extraction complete in {:.2}s!", duration.as_secs_f64());
    println!("The code structure is preserved exactly - no namespace changes.");
    println!("Original backed up as lib.rs.original");
    
    Ok(())
}