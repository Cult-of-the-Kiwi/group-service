#[tokio::main]
async fn main() -> anyhow::Result<()> {
    group_service::app::run().await
}
