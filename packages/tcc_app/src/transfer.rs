//! Transfer module

use std::path::PathBuf;

pub async fn download_file(url: &str, dest: PathBuf) -> anyhow::Result<()> {
    let client = tcc_net::RequestClient::new();
    let bytes = client
        .http()
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;
    std::fs::write(dest, bytes)?;
    Ok(())
}
