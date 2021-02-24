use std::error::Error;
use tokio::process::Command;

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
    println!("{:?}", output);

    let contents = tokio::fs::read("output.json").await?;
    println!("{:?}", String::from_utf8(contents));

    Ok(())
}
