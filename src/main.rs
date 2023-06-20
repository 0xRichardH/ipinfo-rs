use reqwest::blocking::{get, Response};
use reqwest::StatusCode;
use serde::Deserialize;
use tabled::{Table, Tabled};

#[derive(Deserialize, Tabled)]
struct IpInfo {
    ip: String,
    city: String,
    region: String,
    country: String,
    loc: String,
    org: String,
    postal: String,
    timezone: String,
}

fn main() -> Result<(), reqwest::Error> {
    let url: &str = "https://ipinfo.io";
    let resp: Response = get(url)?;
    match resp.status() {
        StatusCode::OK => match resp.json::<IpInfo>() {
            Ok(info) => pretty_print(info),
            Err(_) => panic!("The response JSON is not valid"),
        },
        other => {
            panic!("Something unexpected happened: {:?}", other);
        }
    }

    Ok(())
}

fn pretty_print(info: IpInfo) {
    let ipinfos = vec![info];
    let table = Table::new(ipinfos).to_string();
    println!("{}", table);
}
