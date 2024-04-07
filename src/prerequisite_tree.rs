#[derive(Debug, PartialEq, Clone)]
pub(crate) struct CourseNode {
    pub subject_code: String,
    pub catalog_code: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct LogicNode {
    pub left: Box<PrerequisiteTree>,
    pub right: Box<PrerequisiteTree>,
}

impl LogicNode {
    pub fn new(left: PrerequisiteTree, right: PrerequisiteTree) -> Self {
        Self {
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct MinCreditNode {
    pub credits: u32,
    pub required_subjects: Option<Vec<String>>,
    pub required_levels: Option<Vec<u32>>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum PrerequisiteTree {
    CourseNode(CourseNode),
    OrNode(LogicNode),
    AndNode(LogicNode),
    MinCreditNode(MinCreditNode),
}

#[cfg(test)]
mod tests {
    use crate::{
        course::parser::PrerequisiteParser,
        prerequisite_tree::{CourseNode, LogicNode, MinCreditNode, PrerequisiteTree},
    };

    #[test]
    fn basic_or_test() {
        let input = String::from("ITI 1120 or GNG 1106.");
        let expected = PrerequisiteTree::OrNode(LogicNode::new(
            PrerequisiteTree::CourseNode(CourseNode {
                subject_code: String::from("ITI"),
                catalog_code: 1120,
            }),
            PrerequisiteTree::CourseNode(CourseNode {
                subject_code: String::from("GNG"),
                catalog_code: 1106,
            }),
        ));

        let result = PrerequisiteParser::new().parse(&input).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn basic_and_test() {
        let input = String::from("ITI 1120, GNG1106.");
        let expected = PrerequisiteTree::AndNode(LogicNode::new(
            PrerequisiteTree::CourseNode(CourseNode {
                subject_code: String::from("ITI"),
                catalog_code: 1120,
            }),
            PrerequisiteTree::CourseNode(CourseNode {
                subject_code: String::from("GNG"),
                catalog_code: 1106,
            }),
        ));

        let result = PrerequisiteParser::new().parse(&input).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn nested_expr_test() {
        let input = String::from("MAT1341, (MAT2371 or MAT 2377).");

        let expected = PrerequisiteTree::AndNode(LogicNode::new(
            PrerequisiteTree::CourseNode(CourseNode {
                subject_code: String::from("MAT"),
                catalog_code: 1341,
            }),
            PrerequisiteTree::OrNode(LogicNode::new(
                PrerequisiteTree::CourseNode(CourseNode {
                    subject_code: String::from("MAT"),
                    catalog_code: 2371,
                }),
                PrerequisiteTree::CourseNode(CourseNode {
                    subject_code: String::from("MAT"),
                    catalog_code: 2377,
                }),
            )),
        ));

        assert_eq!(expected, PrerequisiteParser::new().parse(&input).unwrap());
    }

    #[test]
    fn double_nested_expr_test() {
        let input = String::from("MAT 1341, ((MAT 2371, MAT 2375) or MAT 2377).");

        let expected = PrerequisiteTree::AndNode(LogicNode::new(
            PrerequisiteTree::CourseNode(CourseNode {
                subject_code: String::from("MAT"),
                catalog_code: 1341,
            }),
            PrerequisiteTree::OrNode(LogicNode::new(
                PrerequisiteTree::AndNode(LogicNode::new(
                    PrerequisiteTree::CourseNode(CourseNode {
                        subject_code: String::from("MAT"),
                        catalog_code: 2371,
                    }),
                    PrerequisiteTree::CourseNode(CourseNode {
                        subject_code: String::from("MAT"),
                        catalog_code: 2375,
                    }),
                )),
                PrerequisiteTree::CourseNode(CourseNode {
                    subject_code: String::from("MAT"),
                    catalog_code: 2377,
                }),
            )),
        ));

        assert_eq!(expected, PrerequisiteParser::new().parse(&input).unwrap());
    }

    #[test]
    fn min_course_units_test() {
        let input = String::from("18 university units in CSI.");

        let expected = PrerequisiteTree::MinCreditNode(MinCreditNode {
            credits: 18,
            required_subjects: Some(vec!["CSI".to_string()]),
            required_levels: None,
        });

        assert_eq!(expected, PrerequisiteParser::new().parse(&input).unwrap());
    }
    #[test]
    fn min_course_units_test2() {
        let input = String::from("18 university units in CSI or SEG at the 3000 or 4000 level.");

        let expected = PrerequisiteTree::MinCreditNode(MinCreditNode {
            credits: 18,
            required_subjects: Some(vec!["CSI".to_string(), "SEG".to_string()]),
            required_levels: Some(vec![3000, 4000]),
        });

        assert_eq!(expected, PrerequisiteParser::new().parse(&input).unwrap());
    }

    #[test]
    fn course_and_min_credit_test() {
        let input = String::from("CSI 1111 and 18 course units in CSI or SEG at the 3000 level.");

        let expected = PrerequisiteTree::AndNode(LogicNode::new(
            PrerequisiteTree::CourseNode(CourseNode {
                catalog_code: 1111,
                subject_code: "CSI".to_string(),
            }),
            PrerequisiteTree::MinCreditNode(MinCreditNode {
                credits: 18,
                required_levels: Some(vec![3000]),
                required_subjects: Some(vec!["CSI".to_string(), "SEG".to_string()]),
            }),
        ));

        assert_eq!(PrerequisiteParser::new().parse(&input).unwrap(), expected)
    }

    #[test]
    fn min_credit_expanded() {
        let input = String::from("18 course units in Computer Science (CSI) or Software Engineering (SEG) at the 3000 level.");

        let expected = PrerequisiteTree::MinCreditNode(MinCreditNode {
            credits: 18,
            required_levels: Some(vec![3000]),
            required_subjects: Some(vec!["CSI".to_string(), "SEG".to_string()]),
        });

        assert_eq!(PrerequisiteParser::new().parse(&input).unwrap(), expected);
    }

    #[test]
    fn test() {
        let input = String::from("Prerequisite: ITI 1120.");

        let expected = PrerequisiteTree::CourseNode(CourseNode {
            catalog_code: 1120,
            subject_code: "ITI".to_string(),
        });

        assert_eq!(PrerequisiteParser::new().parse(&input).unwrap(), expected);
    }
}
