use core::fmt;
use lalrpop_util::lalrpop_mod;
use parser::PrerequisiteParser;
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use std::collections::HashMap;

use super::{prerequisite_tree::PrerequisiteTree, term::Season};
lalrpop_mod!(pub(crate) parser);

// A model mapping the CSV row inputs
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CourseInput {
    subject: String,
    catalog: u32,
    name: String,
    prerequisites: Option<String>,
    #[serde(deserialize_with = "deserialize_bool")]
    winter: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    summer: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    fall: bool,
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(serde::de::Error::custom("expected true or false")),
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Course {
    /// The subject code of the course
    pub subject_code: String,

    /// The name of the course
    pub name: String,

    /// The 4-digit course code
    pub catalog_code: u32,

    /// The prerequisite tree for the course
    pub(crate) prerequisites: Option<PrerequisiteTree>,

    /// The terms during which the course is offered
    pub terms_offered: HashMap<Season, bool>,
}

impl Course {
    pub fn new(input: CourseInput) -> Result<Self, String> {
        let prerequisites = input
            .prerequisites
            .as_ref()
            .map(|prerequisites_str| PrerequisiteParser::new().parse(prerequisites_str).unwrap());

        Ok(Self {
            subject_code: input.subject,
            name: input.name,
            catalog_code: input.catalog,
            prerequisites,
            terms_offered: HashMap::from([
                (Season::Winter, input.winter),
                (Season::Summer, input.summer),
                (Season::Fall, input.fall),
            ]),
        })
    }

    pub fn info(&self) -> CourseInfo {
        CourseInfo {
            subject_code: self.subject_code.clone(),
            name: self.name.clone(),
            catalog_code: self.catalog_code.clone(),
        }
    }
}

impl Serialize for Course {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Course", 3)?;
        state.serialize_field("subject_code", &self.subject_code)?;
        state.serialize_field("catalog_code", &self.catalog_code)?;
        state.serialize_field("course_name", &self.name)?;

        state.end()
    }
}

impl fmt::Display for Course {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}: {}",
            self.subject_code, self.catalog_code, self.name
        )?;
        Ok(())
    }
}

/// A simplified course model
#[derive(Serialize)]
pub struct CourseInfo {
    pub subject_code: String,
    pub name: String,
    pub catalog_code: u32,
}

impl From<Course> for CourseInfo {
    fn from(course: Course) -> Self {
        CourseInfo {
            subject_code: course.subject_code,
            name: course.name,
            catalog_code: course.catalog_code,
        }
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use crate::{
        course::{Course, CourseInput},
        prerequisite_tree::{CourseNode, LogicNode, PrerequisiteTree},
        term::Season,
    };

    #[test]
    fn convert_input_to_course() {
        let input = CourseInput {
            subject: String::from("CSI"),
            name: String::from("Intro to computing"),
            catalog: 1111,
            prerequisites: Some(String::from("CSI 2110, CSI 2132.")),
            summer: true,
            fall: true,
            winter: true,
        };

        let expected = Course {
            subject_code: String::from("CSI"),
            name: String::from("Intro to computing"),
            catalog_code: 1111,
            prerequisites: Some(PrerequisiteTree::AndNode(LogicNode::new(
                PrerequisiteTree::CourseNode(CourseNode {
                    catalog_code: 2110,
                    subject_code: String::from("CSI"),
                }),
                PrerequisiteTree::CourseNode(CourseNode {
                    catalog_code: 2132,
                    subject_code: String::from("CSI"),
                }),
            ))),
            terms_offered: HashMap::from([
                (Season::Winter, true),
                (Season::Summer, true),
                (Season::Fall, true),
            ]),
        };

        assert_eq!(Course::new(input).unwrap(), expected);
    }

    #[test]
    fn convert_input_to_course_no_prerequisites() {
        let input = CourseInput {
            subject: String::from("CSI"),
            name: String::from("Intro to computing"),
            catalog: 1111,
            prerequisites: None,
            summer: true,
            fall: true,
            winter: true,
        };

        let expected = Course {
            subject_code: String::from("CSI"),
            name: String::from("Intro to computing"),
            catalog_code: 1111,
            prerequisites: None,
            terms_offered: HashMap::from([
                (Season::Winter, true),
                (Season::Summer, true),
                (Season::Fall, true),
            ]),
        };

        assert_eq!(Course::new(input).unwrap(), expected);
    }
}
