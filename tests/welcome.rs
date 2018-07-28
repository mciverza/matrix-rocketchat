extern crate matrix_rocketchat;
extern crate matrix_rocketchat_test;
extern crate reqwest;

use std::collections::HashMap;

use matrix_rocketchat::api::{RequestData, RestApi};
use matrix_rocketchat_test::Test;
use reqwest::{Method, StatusCode};

#[test]
fn root_url_returns_a_welcome_message() {
    let test = Test::new().run();
    let url = test.config.as_url.clone();
    let params = HashMap::new();

    let (body, status) = RestApi::call(&Method::Get, &url, RequestData::Body("".to_string()), &params, None).unwrap();
    assert_eq!(body, "Your Rocket.Chat <-> Matrix application service is running\n");
    assert_eq!(status, StatusCode::Ok);
}
