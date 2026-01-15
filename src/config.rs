use clap::{CommandFactory, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(disable_help_flag = true)]
pub struct Config {
    #[arg(short = 'h', long, env = "APP_HOST", default_value = "0.0.0.0")]
    pub host: String,

    #[arg(short = 'p', long, env = "APP_PORT", default_value = "3000")]
    pub port: u16,
}

impl Config {
    pub fn print_help() {
        Config::command().print_help().unwrap();
    }
}
