#![forbid(unsafe_code)]

mod args;

use args::View;
use chrono::Datelike;

fn main() {
    let args: args::Args = argh::from_env();

    let date_time = chrono::offset::Local::now();

    match args.view {
        args::View::Time => {
            println!("{}", date_time.time());
        }
        args::View::Date => {
            println!("{}", date_time.date());
        }
        args::View::DateTime => {
            println!("{}", date_time);
        }
        args::View::Day => {
            println!("{}", date_time.day());
        }
        args::View::Custom(_) => {
            eprintln!(
                "Custom views are not supported :(\n\nSupported views: {:?}",
                View::SUPPORTED
            );
        }
    }
}
