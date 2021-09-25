use std::time::Duration;

use anyhow::Result;
use structopt::StructOpt;
use tokio::net::{TcpListener, TcpStream};

#[derive(StructOpt, Debug)]
#[structopt(
  name = "jftp",
  about = "Just Forward The Port. A simple port forwarder."
)]
struct Opt {
  /// The address to listen on.
  #[structopt(short, long)]
  listen: String,

  /// The address to connect to.
  #[structopt(short, long)]
  connect: String,

  /// Connect timeout in milliseconds.
  #[structopt(short, long, default_value = "30000")]
  timeout_ms: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
  let opt = Opt::from_args();
  pretty_env_logger::init();

  let sock = TcpListener::bind(&opt.listen).await?;
  log::info!("Listening on {}.", sock.local_addr()?);

  loop {
    let (mut incoming, peer) = sock.accept().await?;
    log::info!("Accepted connection from {}.", peer);
    let connect = opt.connect.clone();
    let timeout = Duration::from_millis(opt.timeout_ms);
    tokio::spawn(async move {
      tokio::select! {
        res = TcpStream::connect(&connect) => {
          match res {
            Ok(mut backend) => {
              let _ = tokio::io::copy_bidirectional(&mut incoming, &mut backend).await;
            }
            Err(e) => {
              log::error!("Unable to connect to backend {}: {:?}", connect, e);
            }
          }
        }
        _ = tokio::time::sleep(timeout) => {
          log::error!("Timeout connecting to backend {} after {:?}.", connect, timeout);
        }
      }
    });
  }
}
