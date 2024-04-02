use core::fmt;
use rocket::serde::Deserialize;
use serde::Serialize;

use super::course::Course;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Deserialize)]
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
    pub fn next(&self, include_summer: bool) -> Season {
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

#[derive(Debug)]
pub struct Term {
    season: Season,
    year: u32,
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
