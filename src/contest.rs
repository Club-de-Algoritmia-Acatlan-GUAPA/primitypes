use chrono::serde::ts_milliseconds;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::problem::{ContestId, ProblemId, SubmissionId};

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
    C
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
            "c" => Ok(Language::C),
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
            Language::C => "c",
        };
        write!(f, "{}", lang_str)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct Submission {
    pub problem_id: ProblemId,
    pub user_id: Uuid,
    pub contest_id: Option<ContestId>,
    pub language: Language,
    pub code: Vec<u8>,
    #[serde(
        deserialize_with = "deserialize_string_to_submission_id",
        serialize_with = "serialize_submission_id_as_string"
    )]
    pub id: SubmissionId,
}
fn serialize_submission_id_as_string<S>(
    submission_id: &SubmissionId,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&submission_id.as_u128().to_string())
}

fn deserialize_string_to_submission_id<'de, D>(deserializer: D) -> Result<SubmissionId, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let num: u128 = s.parse().map_err(serde::de::Error::custom)?;
    Ok(SubmissionId::from_u128(num))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct Contest {
    pub id: ContestId,
    pub author: Uuid,
    pub body: ContestBody,
    pub name: String,
    #[serde(with = "ts_milliseconds")]
    pub start_date: chrono::DateTime<chrono::Utc>,
    #[serde(with = "ts_milliseconds")]
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub contest_type: ContestType,
    pub problems: Vec<ProblemId>,
    pub is_frozen: bool,
    pub frozen_time: i32
}


pub enum ContestState {
    NotStarted,
    Running,
    Ended,
}
impl Contest {
    pub fn status(&self) -> ContestState {
        let now = chrono::Utc::now();
        if now < self.start_date {
            ContestState::NotStarted
        } else if now < self.end_date {
            ContestState::Running
        } else {
            ContestState::Ended
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct ContestBody {
    pub information: String,
    pub rules: String,
    pub sponsor: String,
}

impl From<sqlx::types::JsonValue> for ContestBody {
    fn from(value: sqlx::types::JsonValue) -> Self {
        let value = value.as_object().unwrap();
        Self {
            information: value["information"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            rules: value["rules"].as_str().unwrap_or_default().to_string(),
            sponsor: value["sponsor"].as_str().unwrap_or_default().to_string(),
        }
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize, TS, sqlx::Type)]
#[ts(export)]
#[sqlx(type_name = "contest_type", rename_all = "snake_case")]
pub enum ContestType {
    #[default]
    ICPC,
}
