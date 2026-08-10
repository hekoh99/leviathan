use std::env;
use std::fs;
use std::path::PathBuf;

use leviathan_contracts::generated_schemas;

fn main() {
    let command = env::args().nth(1).unwrap_or_else(|| "help".to_string());
    match command.as_str() {
        "schema-gen" => schema_gen(),
        _ => print_help(),
    }
}

fn schema_gen() {
    let output_dir = repo_root().join("schemas/generated");
    fs::create_dir_all(&output_dir).expect("create schema output directory");

    for (file_name, schema) in generated_schemas() {
        let path = output_dir.join(file_name);
        let json = serde_json::to_string_pretty(&schema).expect("serialize schema");
        fs::write(path, json).expect("write schema file");
    }

    println!("Generated {} schema files into {}", generated_schemas().len(), output_dir.display());
}

fn print_help() {
    println!("xtask commands:");
    println!("- schema-gen");
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask has repo root parent")
        .to_path_buf()
}
