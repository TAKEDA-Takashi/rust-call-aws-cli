use serde::Deserialize;
use std::error::Error;
use tokio::process::Command;

#[derive(Debug, Deserialize)]
struct TestData {
    name: String,
    code: u32,
    tags: Option<String>,
    lang: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let output = Command::new("aws")
        .args(&[
            "s3api",
            "select-object-content",
            "--bucket=testdata-xxxx",
            "--key=test_data.json",
            "--input-serialization",
            r#"{"JSON":{"Type":"LINES"}}"#,
            "--output-serialization",
            r#"{"JSON":{"RecordDelimiter":"\n"}}"#,
            "--expression",
            "SELECT * FROM s3object s LIMIT 5",
            "--expression-type=SQL",
            "output.json",
        ])
        .output()
        .await?;

    if Some(0) != output.status.code() {
        panic!("{:?}", output);
    }

    let contents = tokio::fs::read("output.json").await?;
    for line in String::from_utf8(contents)?.lines() {
        let d: TestData = serde_json::from_str(line)?;
        println!("{:?}", d);
    }

    Ok(())
}
