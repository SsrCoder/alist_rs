use alist_rs::client::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = Client::new("https://alist.example.com".to_string());
    client.login("username", "password").await?;
    let info = client.me().await?;
    println!("{:?}", info);
    Ok(())
}
