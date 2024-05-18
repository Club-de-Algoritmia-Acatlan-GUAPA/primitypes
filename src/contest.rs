use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::problem::{ContestId, ProblemID, SubmissionId};

#[derive(Clone, Debug, Serialize, Deserialize, TS, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub enum Language {
    Python3,
    Java,
    Cpp11,
    Cpp17,
    Cpp14,
    Cmp,
}
impl TryFrom<String> for Language {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "python3" => Ok(Language::Python3),
            "cpp14" => Ok(Language::Cpp14),
            "cpp11" => Ok(Language::Cpp11),
            "cpp17" => Ok(Language::Cpp17),
            "java" => Ok(Language::Java),
            "cmp" => Ok(Language::Cmp),
            _ => Err("Invalid option"),
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lang_str = match self {
            Language::Java => "java",
            Language::Python3 => "python3",
            Language::Cpp14 => "cpp14",
            Language::Cpp11 => "cpp11",
            Language::Cpp17 => "cpp17",
            Language::Cmp => "cmp",
        };
        write!(f, "{}", lang_str)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct Submission {
    pub problem_id: ProblemID,
    pub user_id: Uuid,
    pub contest_id: Option<ContestId>,
    pub language: Language,
    pub code: Vec<u8>,
    pub id: SubmissionId,
}
