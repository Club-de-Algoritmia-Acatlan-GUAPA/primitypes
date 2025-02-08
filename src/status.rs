use std::{collections::HashMap, fmt};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::back_to_enum;
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, Deserialize, Serialize, TS)]
#[ts(export)]
pub enum Status {
    #[default]
    Pending,
    Accepted,
    WrongAnswer,
    TimeLimitExceeded,
    RuntimeError,
    PartialPoints,
    CompilationError,
    UnknownError(String),
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize, TS, sqlx::Type)]
#[ts(export)]
#[sqlx(type_name = "submission_status", rename_all = "snake_case")]
pub enum StatusPG {
    #[default]
    Pending,
    Accepted,
    WrongAnswer,
    RuntimeError,
    TimeLimitExceeded,
    CompilationError,
    PartialPoints,
    UnknownError,
}
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Accepted => write!(f, "Accepted"),
            Status::WrongAnswer => write!(f, "Wrong Answer"),
            Status::TimeLimitExceeded => write!(f, "Time Limit Exceeded"),
            Status::PartialPoints => write!(f, "Partial Execution"),
            Status::RuntimeError => write!(f, "Runtime Error"),
            Status::UnknownError(e) => write!(f, "Unknown Error:({})", e),
            Status::CompilationError => write!(f, "Compilation Error"),
            Status::Pending => write!(f, "Pending"),
        }
    }
}

impl TryFrom<String> for Status {
    type Error = std::convert::Infallible;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Accepted" | "accepted" => Ok(Status::Accepted),
            "Wrong Answer" | "wrong_answer" => Ok(Status::WrongAnswer),
            "Time Limit Exceeded" | "time_limit_exceeded" => Ok(Status::TimeLimitExceeded),
            "Runtime Error" | "runtime_error" => Ok(Status::RuntimeError),
            "Pending" | "pending" => Ok(Status::Pending),
            "Compilation Error" | "compilation_error" => Ok(Status::CompilationError),
            "Unknown Error" | "unknown_error" => Ok(Status::UnknownError("".to_string())),
            _ => Ok(Status::UnknownError(value)),
        }
    }
}
impl fmt::Display for StatusPG {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StatusPG::Accepted => write!(f, "Accepted"),
            StatusPG::WrongAnswer => write!(f, "Wrong Answer"),
            StatusPG::TimeLimitExceeded => write!(f, "Time Limit Exceeded"),
            StatusPG::PartialPoints => write!(f, "Partial Execution"),
            StatusPG::RuntimeError => write!(f, "Runtime Error"),
            StatusPG::UnknownError => write!(f, "UnknownError"),
            StatusPG::CompilationError => write!(f, "Compilation Error"),
            StatusPG::Pending => write!(f, "Pending"),
        }
    }
}

lazy_static! {
    pub static ref STATUS_PRECEDENCE: HashMap<Status, i32> = HashMap::from([
        (Status::Accepted, 0),
        (Status::PartialPoints, 1),
        (Status::WrongAnswer, 2),
        (Status::TimeLimitExceeded, 3),
        (Status::RuntimeError, 4),
    ]);
}

back_to_enum! {
    #[derive(Debug)]
    #[repr(i32)]
    pub enum TestLibExitCodes {
        Accepted = 0,
        WrongAnswer = 1,
        FormatError = 2,
        PartialExecution = 7,
    }
}

back_to_enum! {
    #[derive(Debug)]
    #[repr(i32)]
    pub enum CmpExitCodes {
        Equal = 0,
        Different = 1,
        Problem = 2,
    }
}
