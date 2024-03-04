#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Tang {
    cpu_used: f64,
}

#[post("/cpu_usage", data = "<task>")]
fn new_data(task: Json<Tang>) {
    println!("{:?}", task);
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/metric_data", routes![new_data])
}
