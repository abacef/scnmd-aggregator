#[macro_use] extern crate rocket;
use serde::{Deserialize, Serialize};
use rocket::serde::{json::Json};

#[derive(Deserialize, Debug)]
struct Tang<'r> {
    description: &'r str,
    complete: bool
}

#[post("/metric_data", data = "<task>")]
fn new_data(task: Json<Tang<'_>>) {
    println!("{:?}", task);
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![new_data])
}
