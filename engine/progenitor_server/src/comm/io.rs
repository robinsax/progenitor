use progenitor::SerialValue;

#[derive(Clone)]
pub struct Route {
    path: String,
}

impl Route {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

impl From<&'static str> for Route {
    fn from(path: &'static str) -> Self {
        Route::new(path.to_string())
    }
}

pub struct Request {
    route: Route,
    payload: SerialValue
}

impl Request {
    pub fn new(route: Route, payload: SerialValue) -> Self {
        Self { route, payload }
    }

    pub fn payload(&self) -> &SerialValue {
        &self.payload
    }

    pub fn route(&self) -> &Route {
        &self.route
    }
}

pub struct Response {
    payload: SerialValue
}

// TODO: No.
impl Clone for Response {
    fn clone(&self) -> Self {
        let payload = match &self.payload {
            SerialValue::Buffer(bytes) => SerialValue::Buffer(bytes.clone())
        };

        Self {
            payload
        }
    }
}

impl Response {
    pub fn new(payload: SerialValue) -> Self {
        Self { payload }
    }

    pub fn payload(self) -> SerialValue {
        self.payload
    }
}
