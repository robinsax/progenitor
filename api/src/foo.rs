use std::sync::{Arc, Mutex};

use rocket::State;
use serde::Serialize;
use rocket::http::Status;
use rocket::request::Request;
use rocket::serde::json::Json;
use rocket::response::{Responder, Result as ResponderResult};

use super::fairings::{SceneState, Foo};

use progenitor::apps::StorageOpError;

#[derive(Serialize, Clone)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: &'static str
}

#[derive(Serialize, Clone)]
#[serde(untagged)]
pub enum ResponseCase<T: Serialize + Clone>  { // TODO fromresidual when moved to core
    Error(ErrorResponse),
    Success(T)
}

impl<T: Serialize + Clone> From<StorageOpError> for ResponseCase<T> {
    fn from(_: StorageOpError) -> Self {
        Self::Error(ErrorResponse{
            code: 500,
            message: "database erro"
        })
    }
}

impl<'r, T: Serialize + Clone> Responder<'r, 'r> for ResponseCase<T> {
    fn respond_to(self, req: &Request) -> ResponderResult<'r> {
        match self {
            ResponseCase::Success(payload) => Json(payload.clone()).respond_to(req),
            ResponseCase::Error(err) => match Status::from_code(err.code) {
                Some(status) => {
                    let mut resp = Json(err.clone()).respond_to(req)?;
                    resp.set_status(status);
                    return Ok(resp);
                },
                _ => Err(Status::InternalServerError)
            }
        }
    }
}

#[get("/foos")]
pub async fn get_foos(
    scene_state: &State<Arc<Mutex<SceneState>>>
) -> ResponseCase<Foo> {
    let scene = scene_state.lock().unwrap();

    let x = scene.foo_store.query().one().await;

    match x {
        Ok(Some(f)) => ResponseCase::Success(f),
        Ok(None) => ResponseCase::Error(ErrorResponse{
            code: 404,
            message: "nah"
        }),
        Err(err) => err.into()
    }
}
