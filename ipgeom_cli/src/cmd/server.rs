use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Result;
use clap::Args;

/// Run the HTTP server.
#[derive(Args)]
pub struct ServerCmd {
    /// Path to the SQLite database file
    #[arg(short, long, default_value = "ipgeom.db", env = "IPGEOMANCER_DB")]
    db: PathBuf,

    /// Address to listen on
    #[arg(
        short,
        long,
        default_value = "127.0.0.1:3000",
        env = "IPGEOMANCER_LISTEN"
    )]
    listen: SocketAddr,

    /// Open the web interface in a browser
    #[arg(long)]
    open: bool,
}

pub fn handle(args: ServerCmd) -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;

    if args.open {
        let url = format!("http://{}", args.listen);
        if let Err(e) = open_in_browser(&url) {
            eprintln!("Failed to open browser: {}", e);
        }
    }

    rt.block_on(ipgeom_server::run(args.listen, &args.db))
}

fn open_in_browser(url: &str) -> std::io::Result<()> {
    #[cfg(target_os = "linux")]
    let mut cmd = Command::new("xdg-open");

    #[cfg(target_os = "macos")]
    let mut cmd = Command::new("open");

    #[cfg(target_os = "windows")]
    let mut cmd = {
        let mut c = Command::new("cmd");
        c.arg("/C").arg("start");
        c
    };

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    compile_error!("opening a browser is not supported on this platform");

    cmd.arg(url);
    cmd.status().map(|_| ())
}
