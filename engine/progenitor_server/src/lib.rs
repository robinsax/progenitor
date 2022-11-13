#[macro_use]
extern crate macro_rules_attribute;

mod comm;
mod effects;

pub use self::comm::{Request, Response, CommError, Server};

pub mod effect {
    pub use super::effects::{read_req, write_resp};
}

pub mod ext {
    pub use super::comm::ext::CommDriver;

    // TODO: Extension.
    pub use super::comm::ext::Http1Comm;
}
