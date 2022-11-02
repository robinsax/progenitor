use crate::serial::SerialValue;

#[derive(Clone)]
pub struct Route {
    pub path: String,
}

impl Route {
    pub fn new(path: String) -> Self {
        Self {
            path
        }
    }
}

impl From<&'static str> for Route {
    fn from(path: &'static str) -> Self {
        Route::new(path.to_string())
    }
}

pub struct Request {
    pub route: Route,
    pub payload: SerialValue
}

pub struct Response {
    pub payload: SerialValue
}

impl From<()> for Response {
    fn from(_: ()) -> Self {
        Response { payload: SerialValue::empty() }
    }
}
