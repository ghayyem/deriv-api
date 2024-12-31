use std::env;
use std::process::Command;

fn main() {
    // Only run code generation when explicitly requested
    if env::var("DERIV_API_GENERATE").is_ok() {
        println!("cargo:rerun-if-changed=bin/generate_calls.rs");
        println!("cargo:rerun-if-changed=bin/schema_generator.rs");
        println!("cargo:rerun-if-changed=deriv-api-docs/config/v3/");
    }
}
