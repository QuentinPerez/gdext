fn main() {
    println!("cargo:rerun-if-changed=src/extension_api.json");
    println!("cargo:rerun-if-changed=src/gdextension_interface.h");
}
