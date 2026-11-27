use super::*;

#[test]
fn test_format_hyperlane_attributes_is_idempotent() {
    let input: &str = "    #[methods( get ,  post ,  put )]\n    #[route( \"/api/v1/items\" )]\n    #[hyperlane::response_header( \"x-id\" , \"1\" )]\n    fn handle() {}\n";
    let (once, hits1): (String, usize) = format_hyperlane_attributes_in_source(input);
    assert_eq!(hits1, 3);
    assert!(once.contains("    #[methods(get, post, put)]\n"));
    assert!(once.contains("    #[route(\"/api/v1/items\")]\n"));
    assert!(once.contains("    #[hyperlane::response_header(\"x-id\", \"1\")]\n"));
    let (twice, hits2): (String, usize) = format_hyperlane_attributes_in_source(&once);
    assert_eq!(hits2, 0);
    assert_eq!(once, twice);
}

#[tokio::test]
async fn test_execute_fmt_runs_crate_cli_unified_then_hyperlane_attrs() {
    let dir: PathBuf = temp_dir().join("hyperlane_cli_unified_fmt_test");
    drop(fs::remove_dir_all(&dir));
    fs::create_dir_all(&dir).unwrap();
    let md_path: PathBuf = dir.join("README.md");
    let yml_path: PathBuf = dir.join("ci.yml");
    let rs_path: PathBuf = dir.join("handler.rs");
    fs::write(&md_path, "# Title\n\n| A | B |\n|---|---|\n| 1 | 2 |\n").unwrap();
    fs::write(&yml_path, "name: \"CI\"\non:\n- push\n").unwrap();
    fs::write(&rs_path, "#[methods( get , post )]\nfn handler() {}\n").unwrap();
    let report: HyperlaneFmtReport = execute_fmt(&dir, false).await.unwrap();
    assert_eq!(report.get_text_files_formatted(), 2);
    assert_eq!(report.get_rust_files_formatted(), 1);
    let md_after: String = fs::read_to_string(&md_path).unwrap();
    let yml_after: String = fs::read_to_string(&yml_path).unwrap();
    let rs_after: String = fs::read_to_string(&rs_path).unwrap();
    assert!(md_after.contains("| --- | --- |"));
    assert!(yml_after.contains("name: 'CI'"));
    assert!(yml_after.contains("  - push"));
    assert_eq!(rs_after, "#[methods(get, post)]\nfn handler() {}\n");
    let check_report: HyperlaneFmtReport = execute_fmt(&dir, true).await.unwrap();
    assert_eq!(check_report.get_text_files_formatted(), 0);
    assert_eq!(check_report.get_rust_files_formatted(), 0);
    drop(fs::remove_dir_all(&dir));
}
