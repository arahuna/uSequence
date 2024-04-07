use config::SequenceConfig;
use course::Course;
use input::validate_input;
use prerequisites::validate_prerequisites;
use term::{Season, Term};

/* TYPES */
pub mod course;
pub mod prerequisite_tree;
pub mod term;

/* UTILS */
pub mod config;
pub mod csv;
pub mod input;
pub mod prerequisites;

pub trait Sequence {
    fn sequence(&self, courses: Vec<Course>) -> Result<Vec<Term>, String>;
}

pub struct Sequencer {
    config: SequenceConfig,
}

impl Sequencer {
    pub fn new(
        include_summer: bool,
        starting_semester: Season,
        starting_year: u32,
        max_courses_per_term: u32,
    ) -> Self {
        let config = SequenceConfig {
            include_summer,
            starting_semester,
            starting_year,
            max_courses_per_term,
        };

        Sequencer { config }
    }
}

impl Sequence for Sequencer {
    fn sequence(&self, mut courses: Vec<Course>) -> Result<Vec<Term>, String> {
        // Make sure we can in fact sequence the courses given the config
        if let Err(err) = validate_input(&courses, &self.config) {
            return Err(err);
        }

        // Sort courses by year, this is done as a heuristic to improve sequencing
        courses.sort_by(|a, b| a.catalog_code.cmp(&b.catalog_code));

        // Instatiate some necessary variables
        let mut result: Vec<Term> = vec![];
        let mut courses_taken: Vec<Course> = vec![];

        // Set starting year and season
        let mut current_season = self.config.starting_semester;
        let mut current_year = self.config.starting_year;

        while !courses.is_empty() {
            let mut current_term = Term::new(current_season, current_year, vec![]);

            while current_term.courses.len() < self.config.max_courses_per_term as usize {
                // If there is a course we can take, add it to the courses in the term
                if let Some(next_course_index) = courses.iter().position(|c| {
                    validate_prerequisites(&c.prerequisites, &courses_taken)
                        && *c.terms_offered.get(&current_season).unwrap()
                }) {
                    let next_course = courses.remove(next_course_index);
                    current_term.courses.push(next_course.clone());
                } else {
                    break;
                }
            }

            courses_taken.extend(current_term.courses.clone());

            result.push(current_term);
            current_season = current_season.next(self.config.include_summer);

            match current_season {
                Season::Winter => current_year += 1,
                _ => (),
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{
        course::Course,
        prerequisite_tree::{CourseNode, LogicNode, PrerequisiteTree},
        term::Season,
        Sequence, Sequencer,
    };

    #[test]
    fn sequence_courses_test() {
        let courses = vec![
            Course {
                subject_code: "CSI".to_string(),
                name: String::from("A computing course"),
                catalog_code: 1111,
                prerequisites: None,
                terms_offered: HashMap::from([
                    (Season::Winter, false),
                    (Season::Summer, true),
                    (Season::Fall, true),
                ]),
            },
            Course {
                subject_code: "CSI".to_string(),
                name: String::from("A computing course"),
                catalog_code: 1112,
                prerequisites: Some(PrerequisiteTree::CourseNode(CourseNode {
                    subject_code: "CSI".to_string(),
                    catalog_code: 1111,
                })),
                terms_offered: HashMap::from([
                    (Season::Winter, true),
                    (Season::Summer, true),
                    (Season::Fall, true),
                ]),
            },
            Course {
                subject_code: "CSI".to_string(),
                name: String::from("A computing course"),
                catalog_code: 1113,
                prerequisites: Some(PrerequisiteTree::CourseNode(CourseNode {
                    subject_code: "MAT".to_string(),
                    catalog_code: 1111,
                })),
                terms_offered: HashMap::from([
                    (Season::Winter, true),
                    (Season::Summer, true),
                    (Season::Fall, true),
                ]),
            },
            Course {
                subject_code: "MAT".to_string(),
                name: String::from("A math course"),
                catalog_code: 1111,
                prerequisites: None,
                terms_offered: HashMap::from([
                    (Season::Winter, false),
                    (Season::Summer, true),
                    (Season::Fall, true),
                ]),
            },
            Course {
                subject_code: "CSI".to_string(),
                name: String::from("A computing course"),
                catalog_code: 2111,
                prerequisites: Some(PrerequisiteTree::AndNode(LogicNode::new(
                    PrerequisiteTree::CourseNode(CourseNode {
                        subject_code: "CSI".to_string(),
                        catalog_code: 1112,
                    }),
                    PrerequisiteTree::CourseNode(CourseNode {
                        subject_code: "CSI".to_string(),
                        catalog_code: 1113,
                    }),
                ))),
                terms_offered: HashMap::from([
                    (Season::Winter, false),
                    (Season::Summer, true),
                    (Season::Fall, true),
                ]),
            },
            Course {
                subject_code: "PHY".to_string(),
                name: String::from("A physics course"),
                catalog_code: 1111,
                prerequisites: None,
                terms_offered: HashMap::from([
                    (Season::Winter, true),
                    (Season::Summer, false),
                    (Season::Fall, true),
                ]),
            },
        ];

        let result = Sequencer::new(false, Season::Fall, 2023, 3)
            .sequence(courses)
            .unwrap();

        assert_eq!(result.len(), 3);
    }
}
