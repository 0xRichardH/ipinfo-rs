use reqwest::blocking::get;
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
    let url = "https://ipinfo.io";
    request_ipinfo(url)
}

fn request_ipinfo(url: &str) -> Result<(), reqwest::Error> {
    let resp = get(url)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_ipinfo() {
        let mut s = mockito::Server::new();
        let url = s.url();
        let body = r#"
        {
            "ip": "1.2.33.115",
            "hostname": "ec2-13-212-33-105.ap-southeast-1.compute.amazonaws.com",
            "city": "Singapore",
            "region": "Singapore",
            "country": "SG",
            "loc": "1.2795,103.8682",
            "org": "AS16509 Amazon.com, Inc.",
            "postal": "019396",
            "timezone": "Asia/Singapore",
            "readme": "https://ipinfo.io/missingauth"
        }
    "#;
        s.mock("GET", "/").with_status(200).with_body(body).create();

        let result = request_ipinfo(url.as_str());
        assert!(result.is_ok());
    }
}
