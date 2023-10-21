use std::str::FromStr;

use cornucopia::{CodegenSettings, Error};
use postgres::{Client, NoTls};

fn main() -> () {
    let queries_path = "queries";
    let destination = "src/cornucopia.rs";
    let settings = CodegenSettings {
        is_async: true,
        derive_ser: false,
    };

    let url = std::env::var("MS_DATABASE_URL")
        .unwrap_or_else(|e| panic!("could not find env var MS_DATABASE_URL: {e}"));
    let config = postgres::Config::from_str(&url).unwrap_or_else(|e| {
        panic!("could not construct config from database url {url}: {e}")
    });
    let mut client = config
        .connect(NoTls)
        .unwrap_or_else(|e| panic!("could not connect: {e}"));

    println!("cargo:rerun-if-changed={queries_path}");
    cornucopia::generate_live(
        &mut client,
        queries_path,
        Some(destination),
        settings,
    )
    .unwrap_or_else(|e| panic!("{}", e.report()));
}
