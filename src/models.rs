use rocket::{form::FromForm, fs::TempFile};

use super::utils::term::Season;

#[derive(FromForm)]
pub struct RequestBody<'f> {
    pub include_summer: bool,
    pub starting_semester: Season,
    pub starting_year: u32,
    pub max_courses_per_term: u32,
    pub courses: TempFile<'f>,
}
