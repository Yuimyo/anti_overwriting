mod fs;
mod sys;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    fs::rename();

    Ok(())
}
