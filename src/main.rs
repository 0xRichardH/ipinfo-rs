mod error;
mod prelude;

use crate::prelude::*;
use reqwest::blocking::get;
use reqwest::StatusCode;
use serde::Deserialize;
use tabled::{Table, Tabled};

#[derive(Debug, Deserialize, Tabled)]
struct IpInfo {
    ip: String,
    city: String,
    region: String,
    country: String,
    loc: String,
    org: String,
    timezone: String,
}

const IPINFO_URL: &str = "https://ipinfo.io";

fn main() -> Result<()> {
    let info = request_ipinfo(IPINFO_URL)?;
    pretty_print(info);

    Ok(())
}

fn request_ipinfo(url: &str) -> Result<IpInfo> {
    let resp = get(url).map_err(RequestIpinfoError::Request)?;
    match resp.status() {
        StatusCode::OK => resp
            .json::<IpInfo>()
            .map_err(RequestIpinfoError::JsonDecode),
        other => Err(RequestIpinfoError::Http(other)),
    }
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
    fn it_requests_ipinfo_success() {
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

    #[test]
    fn it_requests_ipinfo_failed_when_invalid_status_code() {
        let mut s = mockito::Server::new();
        let url = s.url();
        s.mock("GET", "/").with_status(500).create();

        let result = request_ipinfo(url.as_str());
        assert!(result.is_err());
        assert_eq!(
            format!("{}", result.unwrap_err()),
            "Something unexpected happened: the server responded with 500 Internal Server Error"
        );
    }

    #[test]
    fn it_requests_ipinfo_failed_when_invalid_response_body() {
        let mut s = mockito::Server::new();
        let url = s.url();
        s.mock("GET", "/").with_status(200).create();

        let result = request_ipinfo(url.as_str());
        assert!(result.is_err());
        assert_eq!(
            format!("{}", result.unwrap_err()),
            "The response JSON is not valid"
        );
    }
}
