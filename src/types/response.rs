use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ResponseValue {
    Int(i32),
    String(String),
    Bool(bool),
    List(Vec<ResponseValue>),
    Map(HashMap<String, ResponseValue>),
}

pub type JSONResponse = HashMap<String, ResponseValue>;
