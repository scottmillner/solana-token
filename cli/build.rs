use std::fs;

// Import code generation logic from the library
mod codegen {
    include!("src/codegen.rs");
}

use codegen::Idl;

fn main() {
    let idl_path = "../target/idl/solana_token.json";
    let out_path = "src/generated.rs";

    // Tell cargo to rerun if the IDL changes
    println!("cargo:rerun-if-changed={}", idl_path);

    // Read and parse IDL
    let idl_content = fs::read_to_string(idl_path)
        .expect("Failed to read IDL file. Run 'anchor build' first.");
    let idl: Idl = serde_json::from_str(&idl_content)
        .expect("Failed to parse IDL JSON");

    // Generate Rust code using the codegen module
    let output = codegen::generate_code(&idl);

    // Write generated code
    fs::write(out_path, output)
        .expect("Failed to write generated.rs");

    println!("cargo:warning=Generated {} instructions from IDL", idl.instructions.len());
}
