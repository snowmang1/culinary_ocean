use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use gloo_console::log;
use web_sys::{Request, RequestInit, RequestMode, Response};
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}
impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}
impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}

/// The possible states a fetch request can be in.
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

pub async fn fetch_rec(url: String) -> Result<Vec<HashMap<String, String>>, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    // use serde to parse json into a struct
    // https://rustwasm.github.io/wasm-bindgen/examples/fetch.html#the-fetch-api
    let text = JsFuture::from(resp.json()?).await?;

    let v: Vec<User> = text.into_serde().unwrap(); // deserialized json
    log!(format!("user name {}", v[0].user_email));

    let mut vec_map: Vec<HashMap<String, String>> = Default::default();
    for u in v {
        let mut map: HashMap<String, String> = Default::default();
        map.insert("instructions".to_string(), u.instructions.clone());
        map.insert("ingredients".to_string(), u.ingredients.clone());
        vec_map.push(map);
    }
    Ok(vec_map)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub user_email: String,
    pub password: String,
    pub instructions: String,
    pub ingredients: String,
}

// pub fn vechash_to_vec(map: &HashMap<String, String>) -> Vec<String>{
//     let mut res: Vec<String> = Default::default();
//         for (_i, j) in map {
//             res.push(j.clone());
//         }
//     res
// }
