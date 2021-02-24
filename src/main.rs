use std::error::Error;
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let output = Command::new("aws")
        .args(&["sts", "get-caller-identity"])
        .output()
        .await?;
    println!("{:?}", output);

    Ok(())
}
