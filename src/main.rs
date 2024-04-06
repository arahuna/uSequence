use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::tokio::io::AsyncReadExt;

use usequence_server::models::RequestBody;
use usequence_server::utils::csv::parse_csv;
use usequence_server::utils::input::validate_input;
use usequence_server::utils::sequence::sequence_courses;
use usequence_server::utils::sequence::SequenceConfig;
use usequence_server::utils::term::Term;

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

    let courses_to_sequence = parse_csv(&buf).unwrap();
    let config = SequenceConfig {
        include_summer: body.include_summer,
        starting_semester: body.starting_semester,
        starting_year: body.starting_year,
        max_courses_per_term: body.max_courses_per_term,
    };

    validate_input(&courses_to_sequence, &config)
        .map_err(|e| status::Custom(Status::BadRequest, e))?;

    let sequence = sequence_courses(courses_to_sequence, config);

    Ok(status::Custom(Status::Ok, Json(sequence)))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![healthcheck, sequence])
}
