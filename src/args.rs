use std::str::FromStr;

use argh::FromArgs;
use never::Never;

#[derive(FromArgs, PartialEq, Debug)]
/// Arguments (should I describe the whole app here?)
pub struct Args {
    #[argh(option, long = "view", short = 'v', default = "View::Time")]
    /// which informatiout you want to see.
    /// One of: "time", "t" (current time, default), "date", "d" (current date), "datetime", "dt" (current date and time), "day" (current day of the week).
    pub view: View,
}

#[derive(PartialEq, Debug)]
pub enum View {
    Time,
    Date,
    DateTime,
    Day,
    Custom(String),
}

impl View {
    pub const SUPPORTED: &'static [&'static str] =
        &["time", "t", "date", "d", "datetime", "dt", "day"];
}

impl FromStr for View {
    type Err = Never;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "time" | "t" => View::Time,
            "date" | "d" => View::Date,
            "datetime" | "dt" => View::DateTime,
            "day" => View::Day,
            custom => View::Custom(custom.to_owned()),
        })
    }
}
