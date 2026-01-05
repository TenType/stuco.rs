use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/syllabus.typ");

    // Create public dir if it doesn't exist
    std::fs::create_dir_all("public").ok();

    let status = Command::new("typst")
        .args(["compile", "src/syllabus.typ", "public/syllabus.pdf"])
        .status()
        .expect("failed to run typst");

    if !status.success() {
        panic!("typst compilation failed");
    }
}
