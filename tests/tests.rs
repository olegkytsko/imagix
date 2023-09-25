use std::path::Path;
use assert_cmd::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const SRC_FOLDER_ARG: &str = "--srcfolder=";
const INPUT_DIR: &str = "tmp/images/";
const OUTPUT_DIR: &str = "tmp/images/tmp/";
const FILE_NAME: &str = "image";
const JPG: &str = "jpg";
const PNG: &str = "png";
const SIZE_SMALL: &str = "--size=small";
const MODE_SINGLE: &str = "--mode=single";
const MODE_ALL: &str = "--mode=all";

#[test]
fn dies_no_arg() -> TestResult {
    let mut cmd = Command::cargo_bin("imagecli")?;
    cmd.assert().failure().stderr(predicates::str::contains("Usage"));
    Ok(())
}

#[test]
fn not_all_args() -> TestResult {
    let mut cmd = Command::cargo_bin("imagecli")?;
    cmd.args(&["resize", SIZE_SMALL, MODE_SINGLE])
        .assert()
        .failure()
        .stderr(predicates::str::contains("required arguments"));
    Ok(())
}

#[test]
fn single_image_resize() -> TestResult {
    let mut cmd = Command::cargo_bin("imagecli")?;
    cmd.args(&["resize", 
                SIZE_SMALL, 
                MODE_SINGLE, 
                format!("{}{}{}1.{}", 
                    SRC_FOLDER_ARG,
                    INPUT_DIR, 
                    FILE_NAME, 
                    JPG)
                    .as_str()])
        .assert()
        .success()
        .stdout(predicates::str::contains("Image resized successfully"));
    assert_eq!(true, Path::new(OUTPUT_DIR).is_dir());
    assert_eq!(true, Path::new("tmp/images/tmp/image1.png").is_file());
    Ok(())
}

#[test]
fn multiple_images_resize() -> TestResult {
    let mut cmd = Command::cargo_bin("imagecli")?;
    cmd.args(&["resize",
                SIZE_SMALL,
                MODE_ALL,
                format!("{}{}",
                    SRC_FOLDER_ARG,
                    INPUT_DIR,
                    )
                    .as_str()])
        .assert()
        .success()
        .stdout(predicates::str::contains("Image resized successfully"));
    assert_eq!(true, Path::new(OUTPUT_DIR).is_dir());
    Ok(())
}

#[test]
fn invalid_src_dir() -> TestResult {
    let mut cmd = Command::cargo_bin("imagecli")?;
    cmd.args(&["resize",
                SIZE_SMALL,
                MODE_SINGLE,
                format!("{}temp",
                    SRC_FOLDER_ARG
                ).as_str()])
        .assert()
        .failure()
        .stderr(predicates::str::contains("No such file or directory"));
    Ok(())
}

#[test]
fn no_file() -> TestResult {
    let mut cmd = Command::cargo_bin("imagecli")?;
    cmd.args(&["resize",
                SIZE_SMALL,
                MODE_SINGLE,
                format!("{}/temp/image4.jpg",
                    SRC_FOLDER_ARG
                ).as_str()])
        .assert()
        .failure()
        .stderr(predicates::str::contains("No such file or directory"));
    Ok(())
}

#[test]
fn incorrect_size() -> TestResult {
    let mut cmd = Command::cargo_bin("imagecli")?;
    cmd.args(&["resize",
                "--size=smell",
                MODE_SINGLE,
                format!("{}{}{}1.{}", 
                    SRC_FOLDER_ARG,
                    INPUT_DIR, 
                    FILE_NAME, 
                    JPG)
                    .as_str()])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Wrong value for size"));
    Ok(())
}

#[test]
fn incorrect_mode() -> TestResult {
    let mut cmd = Command::cargo_bin("imagecli")?;
    cmd.args(&["resize",
                SIZE_SMALL,
                "--mode=any",
                format!("{}{}{}1.{}", 
                    SRC_FOLDER_ARG,
                    INPUT_DIR, 
                    FILE_NAME, 
                    JPG)
                    .as_str()])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Wrong value for mode"));
    Ok(())
}

#[test]
fn wrong_file_format() -> TestResult {
    let mut cmd = Command::cargo_bin("imagecli")?;
    cmd.args(&["resize",
                SIZE_SMALL,
                MODE_SINGLE,
                format!("{}{}{}.{}", 
                    SRC_FOLDER_ARG,
                    INPUT_DIR, 
                    "some_pdf", 
                    "pdf")
                    .as_str()])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Invalid file format"));
    Ok(())
}