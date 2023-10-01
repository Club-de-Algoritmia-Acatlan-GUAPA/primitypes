use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::os::unix::process::ExitStatusExt;
use std::process::ExitStatus;
use std::process::Output;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(remote = "Output")]
pub struct RemoteOutput {
    #[serde(
        serialize_with = "serialize_exit_status",
        deserialize_with = "deserialize_exit_status"
    )]
    pub status: std::process::ExitStatus,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

pub fn serialize_exit_status<S>(
    exit_status: &std::process::ExitStatus,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let code = exit_status.code();
    code.serialize(serializer)
}

pub fn deserialize_exit_status<'de, D>(deserializer: D) -> Result<ExitStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let code: Option<i32> = Deserialize::deserialize(deserializer)?;
    let exit_status = match code {
        Some(code) => ExitStatus::from_raw(code),
        None => {
            return Err(serde::de::Error::custom("Exit code is missing"));
        }
    };

    Ok(exit_status)
}

pub mod external_struct {
    use super::RemoteOutput;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::process::Output;

    pub fn serialize<S>(value: &Option<Output>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "RemoteOutput")] &'a Output);

        value.as_ref().map(Helper).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Output>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(#[serde(with = "RemoteOutput")] Output);

        let helper = Option::deserialize(deserializer)?;
        Ok(helper.map(|Helper(external)| external))
    }
}
