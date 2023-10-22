use crate::prelude::*;
use reqwest::{blocking::Response, StatusCode};

impl TryFrom<W<Response>> for IpInfo {
    type Error = RequestIpinfoError;

    fn try_from(value: W<Response>) -> Result<IpInfo> {
        let resp = value.0;
        match resp.status() {
            StatusCode::OK => resp
                .json::<IpInfo>()
                .map_err(RequestIpinfoError::JsonDecode),
            other => Err(RequestIpinfoError::Http(other)),
        }
    }
}
