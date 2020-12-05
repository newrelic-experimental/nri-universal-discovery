mod decorator;
mod discovery;
mod processor;
mod request;
#[macro_use]
extern crate log;

use clap::Clap;
use env_logger::{Builder, Target};
use std::env;

macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

#[derive(Clap)]
#[clap(version= crate_version!(), author = "Kavashen Pather")]
pub struct Opts {
    /// New Relic Account ID
    #[clap(short = 'a', env = "NR_ACCOUNT_ID")]
    account_id: Option<String>,
    /// New Relic One API Key
    #[clap(short = 'k', env = "NR_API_KEY")]
    api_key: String,
    /// Query
    #[clap(short = 'q', env = "NR_QUERY")]
    query: String,
    /// Query mode NRQL or NerdGraph
    #[clap(short = 'm', env = "NR_MODE")]
    mode: Option<String>,
    /// Serve a custom discovery file
    #[clap(short = 'f', env = "NR_DISCOVERY_FILE")]
    discovery_file: Option<String>,
    /// New Relic Account ID
    #[clap(
        short = 'u',
        env = "NR_NERDGRAPH_URL",
        default_value = "https://api.newrelic.com/graphql"
    )]
    nerdgraph_url: String,
    /// Decorate
    #[clap(short = 'd', env = "NR_DECORATOR_FILE")]
    decorator_file: Option<String>,
    /// Verbose logging
    #[clap(short, env = "VERBOSE")]
    verbose: bool,
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();
    if opts.verbose {
        env::set_var("RUST_LOG", "debug")
    }

    let mut builder = Builder::from_default_env();
    builder.target(Target::Stderr); // use stderr so infra agent doesn't complain
    builder.init();

    discovery::start(opts).await;
}
