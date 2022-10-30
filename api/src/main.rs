mod fairings;
mod foo;

use fairings::scene_stage;

#[macro_use] extern crate rocket;

#[launch] // TODO dump rocket, need to experiment with raw http / rpc / configured tokio runtimes for internal svcs
fn rocket() -> _ {
    rocket::build().attach(scene_stage()).mount("/", routes![foo::get_foos])
}
