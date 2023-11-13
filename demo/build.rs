fn main() {
    println!("cargo:rerun-if-changed=translate");
    println!("cargo:rerun-if-changed=src");
}
