use std::path::PathBuf;

#[test]
fn readme_starts_with_crate_md() {
    let root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let crate_md_path = root_path.join("src/crate.md");
    let readme_path = root_path.join("README.md");

    let crate_md = std::fs::read_to_string(crate_md_path).unwrap();
    let readme_md = std::fs::read_to_string(readme_path).unwrap();

    assert!(readme_md.starts_with(&crate_md));
}

#[test]
fn readme_ends_with_examples_md() {
    let root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let examples_path = root_path.join("src/examples.md");
    let readme_path = root_path.join("README.md");

    let examples_md = std::fs::read_to_string(examples_path).unwrap();
    let readme_md = std::fs::read_to_string(readme_path).unwrap();

    assert!(readme_md.ends_with(&examples_md));
}
