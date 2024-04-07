use csv;
use std::error::Error;

use super::course::{Course, CourseInput};

pub fn parse_csv_to_courses(input: &str) -> Result<Vec<Course>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(input.as_bytes());
    let mut output: Vec<Course> = vec![];
    for result in rdr.deserialize() {
        let course_input: CourseInput = result.expect("Failed to deserialize");

        output.push(Course::new(course_input))
    }

    Ok(output)
}

#[cfg(test)]
mod tests {

    use crate::{
        prerequisite_tree::{CourseNode, PrerequisiteTree},
        term::Season,
    };

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn parse_csv_test() {
        let expected = vec![Course {
            subject_code: "CSI".to_string(),
            name: String::from("A computing course"),
            catalog_code: 1111,
            prerequisites: Some(PrerequisiteTree::CourseNode(CourseNode {
                subject_code: "CSI".to_string(),
                catalog_code: 1112,
            })),
            terms_offered: HashMap::from([
                (Season::Winter, true),
                (Season::Summer, true),
                (Season::Fall, false),
            ]),
        }];

        let csv = "Subject,Catalog,Name,Prerequisites,Winter,Summer,Fall\nCSI,1111,A computing course,CSI 1112.,true,true,false";

        let result = parse_csv_to_courses(csv);

        assert_eq!(result.unwrap(), expected);
    }
}
