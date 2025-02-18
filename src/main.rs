pub mod app;
pub use app::App;

use std::time::Duration;

use anyhow::*;
use tokio::sync::mpsc;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let (tx, mut rx) = mpsc::channel(137);

    // Spawned blocks run immediately when U spawn them
    // U don't have to await them unless U need the return values via handles
    tokio::spawn(async move {
        let mut x = 0;
        loop {
            tx.send(x).await;
            x = x + 1;
            sleep(Duration::from_secs(3)).await;
        }
    });

    App::new().run(terminal, &mut rx).await?;

    ratatui::restore();
    color_eyre::Result::Ok(())
}
