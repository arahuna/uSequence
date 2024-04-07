use core::fmt;
use serde::{Deserialize, Serialize};

use super::course::Course;

/// An Enum for the seasons during which a Term can take place
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub enum Season {
    Summer,
    Fall,
    Winter,
}

impl fmt::Display for Season {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Season::Summer => write!(f, "Summer"),
            Season::Fall => write!(f, "Fall"),
            Season::Winter => write!(f, "Winter"),
        }
    }
}

impl Season {
    pub(crate) fn next(&self, include_summer: bool) -> Season {
        match self {
            Season::Summer => Season::Fall,
            Season::Fall => Season::Winter,
            Season::Winter => {
                if include_summer {
                    Season::Summer
                } else {
                    Season::Fall
                }
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Term {
    /// The season during which the term takes place
    season: Season,

    /// The year during which the term takes place
    year: u32,

    /// The courses taken during the term
    pub courses: Vec<Course>,
}

impl Term {
    pub fn new(season: Season, year: u32, courses: Vec<Course>) -> Self {
        Self {
            season,
            year,
            courses,
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Term: {} {}", self.season, self.year)?;
        if &self.courses.len() > &0 {
            for course in &self.courses {
                writeln!(f, "{}", course)?;
            }
        } else {
            writeln!(f, "<No courses in this term>")?;
        }

        Ok(())
    }
}
