use rocket::{form::FromForm, fs::TempFile, FromFormField};
use serde::{Deserialize, Serialize};

use usequence_server::term::Season;

#[derive(FromForm)]
pub struct RequestBody<'f> {
    pub include_summer: bool,
    pub starting_semester: SeasonForm,
    pub starting_year: u32,
    pub max_courses_per_term: u32,
    pub courses: TempFile<'f>,
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, Serialize, Deserialize, FromFormField)]
pub enum SeasonForm {
    Summer,
    Fall,
    Winter,
}

impl From<SeasonForm> for Season {
    fn from(season: SeasonForm) -> Self {
        match season {
            SeasonForm::Summer => Season::Summer,
            SeasonForm::Fall => Season::Fall,
            SeasonForm::Winter => Season::Winter,
        }
    }
}
