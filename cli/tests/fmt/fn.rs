use super::*;

#[tokio::test]
async fn test_format_path_integration() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_fmt");
    let _: Result<(), Error> = create_dir_all(&tmp_dir).await;
    let test_file: PathBuf = tmp_dir.join("test.rs");
    write(&test_file, "fn main() {\n    println!(\"hello\");\n}\n")
        .await
        .unwrap();
    let result: Result<(), io::Error> = format_path(&tmp_dir).await;
    assert!(result.is_ok());
}
