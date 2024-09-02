use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::problem::{ContestId, ProblemId, SubmissionId};
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "relation_type", rename_all = "snake_case")]
pub enum Relations {
    Participant,
    Owner,
    ProblemSetter,
    Member,
    Admin,
}

impl fmt::Display for Relations {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Relations::Participant => write!(f, "Participant"),
            Relations::Owner => write!(f, "Owner"),
            Relations::ProblemSetter => write!(f, "Problem Setter"),
            Relations::Member => write!(f, "Member"),
            Relations::Admin => write!(f, "Admin"),
        }
    }
}

pub enum Resource {
    Contest(ContestId),
    Problem(ProblemId),
    Submission(SubmissionId),
    User(Uuid),
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Resource::Contest(id) => write!(f,"problem:{}", id.as_u32()),
            Resource::Problem(id) => write!(f,"contest:{}", id.as_u32()),
            Resource::Submission(id) => write!(f,"submission:{}", id.as_u128()),
            Resource::User(id) => write!(f,"user:{}", id.to_string()),
        }
    }
}
