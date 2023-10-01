use bit_vec::BitVec;
use serde::{de::IntoDeserializer, Deserialize, Serialize};
use ts_rs::TS;

use crate::{contest::Language, problem::ProblemID, status::StatusPG};

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
    pub problem_id: ProblemID,
    pub from: usize,
    pub to: usize,
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

//https://github.com/serde-rs/serde/issues/1425#issuecomment-462282398`
pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    let opt = opt.as_deref();
    match opt {
        None | Some("") => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}
#[derive(Deserialize, Serialize, Debug, TS)]
#[ts(export)]
pub struct SubmitResponse {
    pub submission_id: String,
}
