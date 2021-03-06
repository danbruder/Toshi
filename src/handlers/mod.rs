macro_rules! new_handler {
    ($N:ident) => {
        impl NewHandler for $N {
            type Instance = Self;
            fn new_handler(&self) -> IOResult<Self::Instance> { Ok(self.clone()) }
        }
    };
}

pub mod bulk;
pub mod index;
pub mod root;
pub mod search;

use index::*;
use settings::{Settings, SETTINGS};

use gotham::handler::*;
use gotham::http::response::create_response;
use gotham::state::*;
use hyper::{Body, StatusCode};

use futures::future;
use futures::future::FutureResult;
use hyper::Response;
use mime;
use serde_json;
use std::error::Error;
use std::sync::Arc;

#[derive(Deserialize, StateData, StaticResponseExtender, Debug)]
pub struct IndexPath {
    index: String,
}

#[derive(Deserialize, StateData, StaticResponseExtender, Debug)]
pub struct QueryOptions {
    #[serde(default = "Settings::default_pretty")]
    pretty: bool,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    reason: String,
}

impl ErrorResponse {
    pub fn new(reason: &str) -> Self {
        ErrorResponse {
            reason: reason.to_string(),
        }
    }
}

type FutureError = FutureResult<(State, Response<Body>), (State, HandlerError)>;

fn handle_error<T>(state: State, err: &T) -> FutureError
where T: Error + Sized {
    let err = serde_json::to_vec(&ErrorResponse::new(&format!("{}", err))).unwrap();
    let resp = create_response(&state, StatusCode::BadRequest, Some((err, mime::APPLICATION_JSON)));
    future::ok((state, resp))
}
