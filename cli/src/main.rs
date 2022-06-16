use clap::Parser;
use std::convert::TryFrom;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use std::time::Duration;
use weather_underground as wu;

#[derive(Debug)]
struct Unit(wu::Unit);

impl FromStr for Unit {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "m" => Ok(Unit(wu::Unit::Metric)),
            "e" => Ok(Unit(wu::Unit::English)),
            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                "Invalid Unit value, options are 'm' for metric and 'e' for imperial.",
            )),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Options {
    /// Timeout in ms
    #[clap(short, long, default_value = "10000")]
    pub timeout: u64,
    /// Unit (m for metric, e for imperial)
    #[clap(short, long, default_value = "m")]
    pub unit: Unit,
    /// ID of the station you want the observations from
    #[clap()]
    pub station_id: String,
}

impl Options {
    pub fn get_timeout(&self) -> Duration {
        Duration::from_millis(self.timeout)
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let opts = Options::parse();

    let client = wu::create_client(opts.get_timeout()).expect("couldn't create client");
    let api_key = wu::fetch_api_key(&client)
        .await
        .expect("couldn't fetch api key");
    let result = wu::fetch_observation(
        &client,
        api_key.as_str(),
        opts.station_id.as_str(),
        &opts.unit.0,
    )
    .await
    .expect("couldn't fetch observation");
    if let Some(value) = result {
        let result = wu::ObservationResponse::try_from(value).expect("couldn't parse response");
        println!(
            "{}",
            result.to_json().expect("couldn't format observations")
        );
    } else {
        eprintln!("no result...");
    }
}
