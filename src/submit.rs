use bit_vec::BitVec;
use serde::{
    de::{self},
    Deserialize, Deserializer, Serialize,
};
use ts_rs::TS;

use crate::{
    contest::Language,
    problem::{ProblemId, SubmissionId},
    status::StatusPG,
    utils::empty_string_as_none,
};

#[derive(Deserialize, Serialize, Debug, TS)]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub struct SubmitForm {
    pub language: Language,
    pub code: String,
    pub problem_id: u32,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub contest_id: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, TS)]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub struct GetSubmissionsForm {
    pub problem_id: ProblemId,
    pub from: usize,
    pub to: usize,
}

#[derive(Deserialize, Serialize, Debug, TS)]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub struct GetSubmissionId {
    #[serde(deserialize_with = "string_into_submission_id")]
    pub submission_id: SubmissionId,
}

fn string_into_submission_id<'de, D>(de: D) -> Result<SubmissionId, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = String::deserialize(de)?;
    let submission_id = SubmissionId::from_string(&opt).map_err(de::Error::custom)?;
    Ok(submission_id)
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct GetSubmissionsSqlx {
    pub output: Option<sqlx::types::JsonValue>,
    pub submission_id: BitVec,
    pub status: StatusPG,
    pub language: String,
}
#[derive(Deserialize, Serialize, Debug, TS)]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub struct GetSubmissionsJson {
    #[ts(type = "ProblemExecutorResult")]
    pub output: Option<sqlx::types::JsonValue>,
    pub submission_id: String,
    pub status: String,
    pub language: String,
    pub submitted_at: u64,
}

#[derive(Deserialize, Serialize, Debug, TS)]
#[ts(export)]
pub struct SubmitResponse {
    pub submission_id: String,
}
