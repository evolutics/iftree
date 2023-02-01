use std::env;
use std::fs;
use std::process;

#[test]
fn main() {
    let dependent = arrange();

    let status = process::Command::new("cargo")
        .current_dir(dependent.path())
        .arg("run")
        .status()
        .unwrap();

    assert!(status.success());
    dependent.close().unwrap();
}

fn arrange() -> tempfile::TempDir {
    let dependent = tempfile::tempdir().unwrap();
    arrange_manifest(&dependent);
    arrange_source(&dependent);
    dependent
}

fn arrange_manifest(dependent: &tempfile::TempDir) {
    let dependency = env::var("CARGO_MANIFEST_DIR").unwrap();
    let contents = format!(
        "[package]
name = 'dependent'
version = '0.1.0'

[dependencies]
iftree = {{ path = {dependency:?} }}",
    );
    fs::write(dependent.path().join("Cargo.toml"), contents).unwrap();
}

fn arrange_source(dependent: &tempfile::TempDir) {
    fs::create_dir(dependent.path().join("src")).unwrap();
    fs::write(
        dependent.path().join("src").join("main.rs"),
        "#[iftree::include_file_tree(\"paths = '/src/**'\")]
pub struct Asset {
    relative_path: &'static str,
    contents_str: &'static str,
}

fn main() {
    assert_eq!(ASSETS.len(), 1);
    assert_eq!(base::src::MAIN_RS.relative_path, \"src/main.rs\");
    assert_eq!(base::src::MAIN_RS.contents_str, include_str!(\"main.rs\"));
}",
    )
    .unwrap();
}
