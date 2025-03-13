use futures_util::future::pending;
mod access;
mod screenshot;

use access::Access;
use screenshot::Screenshot;

include!(concat!(env!("CARGO_TARGET_DIR"), "/config.rs"));

#[tokio::main]
async fn main() -> ashpd::Result<()> {
    let _ = systemd_journal_logger::init();
    log::set_max_level(log::LevelFilter::Info);

    if !std::env::var("XDG_CURRENT_DESKTOP").is_ok_and(|v| v == "gamescope") {
        log::warn!("Not running under a gamescope session");
    }

    if !std::process::Command::new("gamescopectl")
        .arg("version")
        .status()
        .is_ok_and(|s| s.success())
    {
        log::error!("Failed to run gamescopectl, expect degraded functionality");
    }

    ashpd::backend::Builder::new(BUSNAME)?
        // A default implementation of the Access interface is required for
        // the frontend to conditionally discover the Screenshot interface
        // (see https://github.com/flatpak/xdg-desktop-portal/blob/2fb76ffb/src/xdg-desktop-portal.c#L321-L358).
        .access(Access)
        .screenshot(Screenshot)
        .build()
        .await?;

    loop {
        pending::<()>().await;
    }
}
