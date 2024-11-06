#![allow(dead_code)]

use std::error::Error;

use reqwest::header::{
    HeaderMap, HeaderValue, InvalidHeaderValue, ACCEPT, ACCEPT_LANGUAGE, COOKIE,
};
use serde::{Deserialize, Serialize};

const ACT_ID: &str = "e202102251931481";
const BASE_URL: &str = "https://sg-hk4e-api.hoyolab.com";

#[derive(Deserialize)]
pub struct CheckResponse {
    retcode: i32,
    message: String,
    data: CheckData,
}

#[derive(Deserialize)]
pub struct CheckData {
    total_sign_day: u32,
    today: String,
    is_sign: bool,
    first_bind: bool,
    is_sub: bool,
    region: String,
    month_last_day: bool,
}

#[derive(Serialize)]
pub struct ClaimRequestBody {
    act_id: String,
}

#[derive(Deserialize)]
pub struct ClaimResponse {
    retcode: i32,
    message: String,
    data: ClaimData,
}

#[derive(Deserialize)]
pub struct ClaimData {
    code: String,
    first_bind: bool,
    gt_result: GtResult,
}

#[derive(Deserialize)]
pub struct GtResult {
    risk_code: i32,
    gt: String,
    challenge: String,
    success: i32,
    is_risk: bool,
}

fn make_headers(cookie: &str) -> Result<HeaderMap, InvalidHeaderValue> {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/json, text/plain, */*"),
    );
    headers.insert(COOKIE, HeaderValue::from_str(cookie)?);
    Ok(headers)
}

pub async fn is_claimed(client: &reqwest::Client, cookie: &str) -> Result<bool, Box<dyn Error>> {
    let params = [("lang", "en-us"), ("act_id", ACT_ID)];
    let headers = make_headers(cookie)?;

    let response = client
        .get(format!("{BASE_URL}/event/sol/info"))
        .headers(headers)
        .query(&params)
        .send()
        .await?;

    let data = response.json::<CheckResponse>().await?;
    Ok(data.data.is_sign)
}

pub async fn claim(
    client: &reqwest::Client,
    cookie: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let params = [("lang", "en-us")];
    let headers = make_headers(cookie)?;
    let json = ClaimRequestBody {
        act_id: ACT_ID.to_string(),
    };

    let response = client
        .post(format!("{BASE_URL}/event/sol/sign"))
        .headers(headers)
        .query(&params)
        .json(&json)
        .send()
        .await?;

    let data = response.json::<ClaimResponse>().await?;
    Ok(data.data.code == "ok")
}
