mod utils;

use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use utils::sequence::sequence_courses;
use utils::sequence::SequenceConfig;

use crate::utils::csv::parse_csv;

#[macro_use]
extern crate rocket;

#[get("/heartbeat")]
fn healthcheck() -> &'static str {
    "OK"
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct RequestBody {
    config: SequenceConfig,
    courses: String,
}

#[post("/sequence", data = "<body>")]
fn sequence(body: Json<RequestBody>) -> String {
    let courses = parse_csv(&body.courses).unwrap();
    let config = body.config;

    let sequence = sequence_courses(courses, config);

    format!("{:?}", sequence)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![healthcheck, sequence])
}
