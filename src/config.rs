use std::sync::LazyLock;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(short, long)]
    pub pid: Option<u32>,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::parse);
