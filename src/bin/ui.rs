use my_idle::{server, ui};
use tokio::sync::mpsc::unbounded_channel;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let (sender, receiver) = unbounded_channel();
    tokio::spawn(async move {
        server::run_game_loop(sender).await;
    });
    ui::run_ui_app(receiver).await.unwrap();
    Ok(())
}
