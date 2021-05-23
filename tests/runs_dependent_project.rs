use std::env;
use std::fs;
use std::process;

#[test]
fn handles() {
    let dependent = arrange().unwrap();

    let status = process::Command::new("cargo")
        .current_dir(dependent.path())
        .arg("run")
        .status()
        .unwrap();

    assert!(status.success());
    dependent.close().unwrap();
}

fn arrange() -> Option<tempfile::TempDir> {
    let dependent = tempfile::tempdir().ok()?;
    arrange_manifest(&dependent)?;
    arrange_source(&dependent)?;
    Some(dependent)
}

fn arrange_manifest(dependent: &tempfile::TempDir) -> Option<()> {
    let dependency = env::var("CARGO_MANIFEST_DIR").ok()?;
    let dependency = toml::to_string(&dependency).ok()?;
    let contents = format!(
        "[package]
name = 'dependent'
version = '0.1.0'

[dependencies]
iftree = {{ path = {} }}",
        dependency,
    );
    fs::write(dependent.path().join("Cargo.toml"), contents).ok()
}

fn arrange_source(dependent: &tempfile::TempDir) -> Option<()> {
    fs::create_dir(dependent.path().join("src")).ok()?;
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
    .ok()
}
