use super::{course::Course, prerequisite_tree::PrerequisiteTree};

pub(crate) fn validate_prerequisites(
    prerequisites: &Option<PrerequisiteTree>,
    courses_taken: &Vec<Course>,
) -> bool {
    match prerequisites {
        Some(prerequisites) => evaluate_prerequisite_tree(prerequisites, courses_taken),
        None => true,
    }
}

fn evaluate_prerequisite_tree(tree: &PrerequisiteTree, courses_taken: &Vec<Course>) -> bool {
    match tree {
        PrerequisiteTree::CourseNode(course_node) => courses_taken.iter().any(|course| {
            course.subject_code == course_node.subject_code
                && course.catalog_code == course_node.catalog_code
        }),
        PrerequisiteTree::AndNode(logic_node) => {
            evaluate_prerequisite_tree(&logic_node.left, courses_taken)
                && evaluate_prerequisite_tree(&logic_node.right, courses_taken)
        }
        PrerequisiteTree::OrNode(logic_node) => {
            evaluate_prerequisite_tree(&logic_node.left, courses_taken)
                || evaluate_prerequisite_tree(&logic_node.right, courses_taken)
        }
        PrerequisiteTree::MinCreditNode(min_credit_node) => satisfies_min_credits(
            min_credit_node.credits,
            &min_credit_node.required_levels,
            &min_credit_node.required_subjects,
            courses_taken,
        ),
    }
}

fn satisfies_min_credits(
    credits_required: u32,
    required_levels: &Option<Vec<u32>>,
    required_subjects: &Option<Vec<String>>,
    courses_taken: &Vec<Course>,
) -> bool {
    let total_credits = courses_taken
        .iter()
        .filter(|course| {
            let course_level = ((course.catalog_code / 1000) % 10) * 1000;
            let subject_matches = required_subjects
                .as_ref()
                .map_or(true, |subjects| subjects.contains(&course.subject_code));
            let level_matches = required_levels
                .as_ref()
                .map_or(true, |levels| levels.contains(&course_level));
            subject_matches && level_matches
        })
        .count()
        * 3;

    total_credits >= credits_required as usize
}

#[cfg(test)]
mod tests {

    use crate::{
        course::Course,
        prerequisite_tree::{CourseNode, PrerequisiteTree},
        prerequisites::{
            evaluate_prerequisite_tree, satisfies_min_credits, validate_prerequisites,
        },
    };

    use std::collections::HashMap;

    #[test]
    fn test_satisfies_min_credits() {
        let binding = Course {
            subject_code: String::from("MAT"),
            name: String::from("A math course"),
            catalog_code: 2132,
            prerequisites: None,
            terms_offered: HashMap::new(),
        };
        let binding2 = Course {
            subject_code: String::from("CSI"),
            name: String::from("a computing course"),
            catalog_code: 3110,
            prerequisites: None,
            terms_offered: HashMap::new(),
        };
        let courses_taken = vec![&binding, &binding2];

        assert!(
            satisfies_min_credits(6, &None, &None, &courses_taken),
            "Should return true when total credits meet the required credits"
        );

        assert!(
            satisfies_min_credits(3, &None, &Some(vec![String::from("CSI")]), &courses_taken),
            "Should return true when total credits meet the required credits and subject matches"
        );

        assert!(
            !satisfies_min_credits(6, &None, &Some(vec![String::from("CSI")]), &courses_taken),
            "Should return false when total credits do not meet the required credits because subject does not match"
        );

        assert!(
            satisfies_min_credits(6, &Some(vec![3000, 2000]), &None, &courses_taken),
            "Should return true when total credits meet the required credits and level matches"
        );

        assert!(
            !satisfies_min_credits(6, &Some(vec![3000]), &None, &courses_taken),
            "Should return false when total credits do not meet the required credits because level does not match"
        );
    }

    #[test]
    fn test_evaluate_prerequisite_tree() {
        let binding = Course {
            subject_code: String::from("CSI"),
            name: String::from("A computing course"),
            catalog_code: 3110,
            prerequisites: None,
            terms_offered: HashMap::new(),
        };

        let binding2 = Course {
            subject_code: String::from("MAT"),
            name: String::from("A math course"),
            catalog_code: 2132,
            prerequisites: None,
            terms_offered: HashMap::new(),
        };

        let courses_taken = vec![binding, binding2];

        let tree = PrerequisiteTree::CourseNode(CourseNode {
            subject_code: String::from("CSI"),
            catalog_code: 3110,
        });

        assert!(
            evaluate_prerequisite_tree(&tree, &courses_taken),
            "Should return true when prerequisites are satisfied"
        );
    }

    #[test]
    fn test_validate_prerequisites() {
        let binding = Course {
            subject_code: String::from("CSI"),
            name: String::from("Advanced computing"),
            catalog_code: 3110,
            prerequisites: None,
            terms_offered: HashMap::new(),
        };

        let binding2 = Course {
            subject_code: String::from("MAT"),
            name: String::from("Intro to Math"),
            catalog_code: 2132,
            prerequisites: None,
            terms_offered: HashMap::new(),
        };

        let courses_taken = vec![binding, binding2];

        let tree = Some(PrerequisiteTree::CourseNode(CourseNode {
            subject_code: String::from("CSI"),
            catalog_code: 3110,
        }));

        assert!(
            validate_prerequisites(&tree, &courses_taken),
            "Should return true when prerequisites are satisfied"
        );
    }
}
