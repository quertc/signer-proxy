mod app_types;
mod cli;
mod jsonrpc;
mod shutdown_signal;
mod signers;

use cli::{Command, Opt};
use signers::{aws_kms::handle_aws_kms, yubihsm::handle_yubihsm};
use structopt::StructOpt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "info,yubihsm_signer_proxy=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let opt = Opt::from_args();

    match opt.cmd {
        Command::Yubihsm(yubi_opt) => {
            handle_yubihsm(yubi_opt).await;
        }
        Command::AwsKms(aws_opt) => {
            handle_aws_kms(aws_opt).await;
        }
    }
}
