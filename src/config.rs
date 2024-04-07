use crate::term::Season;
use serde::{Deserialize, Serialize};

/// Config used when sequencing courses
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct SequenceConfig {
    /// Whether we should include the summer term
    pub include_summer: bool,

    /// The year the course sequence should start in
    pub starting_year: u32,

    /// The semester the sequence should start in
    pub starting_semester: Season,

    /// The maximum number of courses that can be in a term
    pub max_courses_per_term: u32,
}
