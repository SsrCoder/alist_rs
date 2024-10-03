use alist_rs::client::fs::AddOfflineDownloadRequest;
use alist_rs::client::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = Client::new("https://alist.example.com".to_string());
    client.login("username", "password").await?;

    let tasks = client
        .add_offline_download(AddOfflineDownloadRequest::new(
            "/".into(),
            vec!["url".into()],
        ))
        .await?;
    let task = tasks.first().unwrap();
    loop {
        let undone = client.offline_download_undone().await?;
        let task_status = undone
            .iter()
            .filter(|t| t.id == task.id)
            .collect::<Vec<_>>();
        if task_status.is_empty() {
            break;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    println!("download finished");
    Ok(())
}
