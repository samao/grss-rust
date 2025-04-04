use assert_cmd::Command;
use assert_fs::prelude::FileWriteStr;

#[test]
fn file_doesnt_exit() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foobar").arg("some_not_existing_file");

    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("读取文件失败"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;

    file.write_str("A test\nActual content\nMore hello world\nAnother line")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("test"));

    Ok(())
}
