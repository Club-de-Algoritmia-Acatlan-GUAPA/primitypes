use crate::problem::{ContestId, ProblemID, SubmissionID};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;


#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub enum Language {
    Python3,
    Java,
    Cpp11,
    Cpp17,
    Cpp14,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct Submission {
    pub problem_id: ProblemID,
    pub user_id: Uuid,
    pub contest_id: Option<ContestId>,
    pub language: Language,
    pub code: String,
    pub id: SubmissionID,
}
