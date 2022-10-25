// This build script does nothing. It only triggers a rebuild if any
// locale files are changed.
fn main() {
    println!("cargo:rerun-if-changed=../../locales/**");
}
