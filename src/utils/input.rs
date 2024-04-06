use super::{
    course::Course, prerequisites::validate_prerequisites, sequence::SequenceConfig, term::Season,
};

pub fn validate_input<'a>(courses: &'a Vec<Course>, config: &SequenceConfig) -> Result<(), String> {
    for course in courses {
        if !validate_prerequisites(&course.prerequisites, courses) {
            return Err(format!(
                "Prerequisite for course {} {} can not be satisfied with the courses given",
                course.subject_code, course.catalog_code
            ));
        }

        let course_only_offered_in_summer = course.terms_offered.iter().any(|(season, offered)| {
            if *offered {
                *season == Season::Summer
            } else {
                true
            }
        });

        if !config.include_summer && course_only_offered_in_summer {
            return Err(format!(
                "Course {} {} can only be taken in the summer",
                course.subject_code, course.catalog_code,
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::utils::prerequisite_tree::{CourseNode, PrerequisiteTree};

    use super::*;

    #[test]
    fn should_be_ok_if_input_is_valid() {
        let courses = vec![
            Course {
                subject_code: "CSI".to_string(),
                course_name: String::from("A computing course"),
                catalog_code: 1100,
                prerequisites: None,
                terms_offered: HashMap::from([(Season::Fall, true)]),
            },
            Course {
                subject_code: "CSI".to_string(),
                course_name: String::from("A computing course"),
                catalog_code: 1200,
                prerequisites: Some(PrerequisiteTree::CourseNode(CourseNode {
                    subject_code: "CSI".to_string(),
                    catalog_code: 1100,
                })),
                terms_offered: HashMap::from([(Season::Winter, true)]),
            },
        ];

        let config = SequenceConfig {
            include_summer: true,
            max_courses_per_term: 5,
            starting_semester: Season::Fall,
            starting_year: 2023,
        };

        let result = validate_input(&courses, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn should_return_error_if_prerequisites_cannot_be_satisfied() {
        let courses = vec![
            Course {
                subject_code: "CSI".to_string(),
                course_name: String::from("A computing course"),
                catalog_code: 1100,
                prerequisites: None,
                terms_offered: HashMap::from([(Season::Fall, true)]),
            },
            Course {
                subject_code: "CSI".to_string(),
                course_name: String::from("A computing course"),
                catalog_code: 1300,
                prerequisites: Some(PrerequisiteTree::CourseNode(CourseNode {
                    subject_code: "CSI".to_string(),
                    catalog_code: 1200,
                })),
                terms_offered: HashMap::from([(Season::Winter, true)]),
            },
        ];
        let config = SequenceConfig {
            include_summer: true,
            max_courses_per_term: 5,
            starting_semester: Season::Fall,
            starting_year: 2023,
        };

        let result = validate_input(&courses, &config);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Prerequisite for course CSI 1300 can not be satisfied with the courses given"
                .to_string()
        );
    }

    #[test]
    fn should_return_error_if_course_only_offered_in_summer_but_summer_not_included() {
        let courses = vec![
            Course {
                subject_code: "CSI".to_string(),
                course_name: String::from("A computing course"),
                catalog_code: 1100,
                prerequisites: None,
                terms_offered: HashMap::from([(Season::Summer, true)]),
            },
            Course {
                subject_code: "CSI".to_string(),
                course_name: String::from("A computing course"),
                catalog_code: 1300,
                prerequisites: Some(PrerequisiteTree::CourseNode(CourseNode {
                    subject_code: "CSI".to_string(),
                    catalog_code: 1200,
                })),
                terms_offered: HashMap::from([(Season::Winter, true)]),
            },
        ];
        let config = SequenceConfig {
            include_summer: false,
            max_courses_per_term: 5,
            starting_semester: Season::Fall,
            starting_year: 2023,
        };

        let result = validate_input(&courses, &config);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Course CSI 1100 can only be taken in the summer".to_string()
        );
    }
}
