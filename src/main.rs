mod config;
mod error;
mod plugins;
mod protocol;

use std::process::ExitCode;

use crate::config::Config;
use crate::error::Error;
use crate::plugins::Plugins;
use crate::protocol::Protocol;

fn main() -> ExitCode {
    match run() {
        Some(err) => {
            print_error(&err);
            ExitCode::FAILURE
        }
        None => ExitCode::SUCCESS,
    }
}

/// Run handler
fn run() -> Option<Error> {
    let args: Vec<String> = std::env::args().collect();
    let arg: &str = match args.len() {
        2 => &args[1],
        1 => {
            return {
                print_usage();
                None
            }
        }
        _ => return Some(Error::TooManyArgs),
    };

    match arg {
        "-v" | "--version" => {
            print_usage();
            None
        }
        _ => {
            let proto = Protocol::parse(arg).ok()?;
            let config = Config::load().ok()?;

            match proto.plugin {
                Plugins::Play => crate::plugins::play::exec(&proto, &config),
            }
        }
    }
}

fn print_usage() {
    let version: &str = option_env!("MPV_HANDLER_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));

    println!("mpv-handler {}\n", version);
    println!("Usage:\n  {}\n", "mpv-handler [options] <url>",);
    println!("OPTIONS:\n  {}    {}", "-v, --version", "show version");
}

fn print_error(err: &Error) {
    eprint!("ERROR: {err}");
    std::io::Read::read(&mut std::io::stdin(), &mut []).unwrap();
}
