//! 定期実行されるタスク群
//!
//! 例：スクレイピング

use serde_json::json;

/// EDINETからすべての企業一覧を取得する
pub async fn all_company() -> anyhow::Result<()> {
    let client = reqwest::Client::builder().build()?;

    let response = client
        .get("https://edinet-proxy.tk/api/corporations/")
        .send()
        .await?
        .text()
        .await?;

    let response = json!(response);

    println!("{:?}", response);

    Ok(())
}
