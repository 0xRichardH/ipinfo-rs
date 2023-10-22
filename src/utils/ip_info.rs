use serde::Deserialize;
use tabled::{Table, Tabled};

#[derive(Debug, Deserialize, Tabled)]
pub struct IpInfo {
    ip: String,
    city: String,
    region: String,
    country: String,
    loc: String,
    org: String,
    timezone: String,
}

impl IpInfo {
    pub fn pretty_print(&self) {
        let ipinfos = vec![self];
        let table = Table::new(ipinfos).to_string();
        println!("{}", table);
    }
}
