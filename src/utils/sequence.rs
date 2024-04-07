use serde::Deserialize;

use super::{
    course::Course,
    prerequisites::validate_prerequisites,
    term::{Season, Term},
};

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct SequenceConfig {
    pub include_summer: bool,
    pub starting_year: u32,
    pub starting_semester: Season,
    pub max_courses_per_term: u32,
}

pub fn sequence_courses(mut courses: Vec<Course>, config: SequenceConfig) -> Vec<Term> {
    // Sort courses by year, this is done as a heuristic to improve sequencing
    courses.sort_by(|a, b| a.catalog_code.cmp(&b.catalog_code));

    // Instatiate some necessary variables
    let mut result: Vec<Term> = vec![];
    let mut courses_taken: Vec<Course> = vec![];

    // Set starting year and season
    let mut current_season = config.starting_semester;
    let mut current_year = config.starting_year;

    while !courses.is_empty() {
        let mut current_term = Term::new(current_season, current_year, vec![]);

        while current_term.courses.len() < config.max_courses_per_term as usize {
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
        current_season = current_season.next(config.include_summer);

        match current_season {
            Season::Winter => current_year += 1,
            _ => (),
        }
    }

    result
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::utils::{
        course::Course,
        prerequisite_tree::{CourseNode, LogicNode, PrerequisiteTree},
        term::Season,
    };

    use super::{sequence_courses, SequenceConfig};

    #[test]
    fn sequence_courses_test() {
        let courses = vec![
            Course {
                subject_code: "CSI".to_string(),
                course_name: String::from("A computing course"),
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
                course_name: String::from("A computing course"),
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
                course_name: String::from("A computing course"),
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
                course_name: String::from("A math course"),
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
                course_name: String::from("A computing course"),
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
                course_name: String::from("A physics course"),
                catalog_code: 1111,
                prerequisites: None,
                terms_offered: HashMap::from([
                    (Season::Winter, true),
                    (Season::Summer, false),
                    (Season::Fall, true),
                ]),
            },
        ];

        let result = sequence_courses(
            courses,
            SequenceConfig {
                include_summer: false,
                max_courses_per_term: 3,
                starting_semester: Season::Fall,
                starting_year: 2023,
            },
        );

        assert_eq!(result.len(), 3);
    }
}
