use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::tokio::io::AsyncReadExt;

use models::RequestBody;
use usequence_server::csv::parse_csv_to_courses;
use usequence_server::term::Term;
use usequence_server::Sequence;
use usequence_server::Sequencer;

mod models;

#[macro_use]
extern crate rocket;

#[get("/heartbeat")]
fn healthcheck() -> &'static str {
    "OK"
}

#[post("/sequence", data = "<body>")]
async fn sequence(
    body: Form<RequestBody<'_>>,
) -> Result<status::Custom<Json<Vec<Term>>>, status::Custom<String>> {
    let mut courses_input = body.courses.open().await.unwrap();
    let mut buf = String::new();

    courses_input.read_to_string(&mut buf).await.unwrap();

    let sequencer = Sequencer::new(
        body.include_summer,
        body.starting_semester.into(),
        body.starting_year,
        body.max_courses_per_term,
    );

    let courses_to_sequence = parse_csv_to_courses(&buf).unwrap();

    let sequence = sequencer
        .sequence(courses_to_sequence)
        .map_err(|e| status::Custom(Status::BadRequest, e))?;

    Ok(status::Custom(Status::Ok, Json(sequence)))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![healthcheck, sequence])
}
