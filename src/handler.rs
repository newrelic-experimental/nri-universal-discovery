use super::Opts;
use bytes::Bytes;
use hyper::{Body, Client, Method, Request};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{env, fs};
extern crate base64;

pub fn nrql(json_string: String) -> String {
    //
    return String::from("a");
}
