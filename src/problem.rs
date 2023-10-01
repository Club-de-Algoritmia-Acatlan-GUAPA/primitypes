use crate::consts::{
    CONTEST_ID_BITS, EMPTY_BITS, PROBLEM_ID_BITS, SUBMISSION_ID_BITS, TIMESTAMP_BITS,
    UUID_TIME_MID_BITS,
};
use crate::serde::external_struct;
use crate::status::Status;
use anyhow::{anyhow, Result};
use bit_vec::BitVec;
use serde::{Deserialize, Serialize};
use std::process::Output;
use ts_rs::TS;
use uuid::Uuid;
/// # Id concurso (20 bits):
///
///    Se auto serializa en la base de datos de uno en uno,se guarda en 20 bits
///    es decir podemos tener un maximo de 2^20 concursos con 32 problemas cada uno.
///
#[derive(TS)]
#[ts(export)]
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct ContestId(pub u32);
impl ContestId {
    pub fn as_u32(&self) -> u32 {
        self.0
    }
}
impl From<&ContestId> for u128 {
    fn from(value: &ContestId) -> Self {
        value.0 as u128
    }
}

impl From<ContestId> for u128 {
    fn from(value: ContestId) -> Self {
        value.0 as u128
    }
}
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Hash, TS)]
#[serde(transparent)]
pub struct ProblemID(pub u32);
impl Eq for ProblemID {}
impl From<&ProblemID> for u128 {
    fn from(value: &ProblemID) -> Self {
        value.0 as u128
    }
}
#[derive(TS)]
#[ts(export)]
pub enum ProblemType {
    Individual = 0,
    Contest = 1,
}

impl ProblemID {
    pub fn new(problem_type: ProblemType, id: u32) -> Self {
        let mut base = match problem_type {
            ProblemType::Individual => 0,
            ProblemType::Contest => 1 << (PROBLEM_ID_BITS - 1),
        };
        base |= id;
        Self(base)
    }
    pub fn as_u32(&self) -> u32 {
        self.0
    }
    pub fn is_contest_problem(value: &u32) -> bool {
        (value >> (PROBLEM_ID_BITS - 1)) & 1 == 1
    }
    pub fn is_individual_problem(value: &u32) -> bool {
        !ProblemID::is_contest_problem(value)
    }
    pub fn as_submission_id_bit_vec(&self) -> BitVec {
        let mask_of_ones: u128 = (1 << PROBLEM_ID_BITS) - 1;
        let mask_of_shifted_bits: u128 = mask_of_ones << (UUID_TIME_MID_BITS + EMPTY_BITS);
        let shifted_problem_id: u128 = (self.0 as u128) << (UUID_TIME_MID_BITS + EMPTY_BITS);
        println!("{:b}", mask_of_ones);
        println!("{:b}", mask_of_shifted_bits);
        println!("{:b}", shifted_problem_id);
        let problem_id =
            BitVec::from_bytes(&(shifted_problem_id & mask_of_shifted_bits).to_be_bytes());
        let mut base = BitVec::from_elem(128, false);
        base.or(&problem_id);
        base
    }
}
/// # Id submision (128 bits)
///
///    Se compone de concatenar :
///        - la hora desde unix_epoch en milisegundos, se guarda en 41 bits
///        - el id_concurso -> 20 bits
///        - el id_problema  -> 29 bits
///        - el [time_mid del uuid](https://es.wikipedia.org/wiki/Identificador_%C3%BAnico_universal#:~:text=Un%20n%C3%BAmero%20entero%20de%2016%20bits%20(4%20d%C3%ADgitos%20hexadecimales)%20%22time_mid%22%20con%20los%2016%20bits%20centrales%20del%20timestamp.) del usuario consta de 16 bits -> 16 bits
///
///    Total 100 bits
///

#[derive(Default, Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub struct SubmissionID(u128, #[serde(skip)] BitVec);

impl SubmissionID {
    pub fn from_u128(base: u128) -> Self {
        let mut bv = BitVec::from_elem(128, false);
        let base_vec = BitVec::from_bytes(&base.to_be_bytes());
        bv.or(&base_vec);
        Self(base, bv)
    }
    pub fn from_bitvec(bitvec: BitVec) -> Result<Self> {
        let bytes: [u8; 16] = bitvec
            .to_bytes()
            .try_into()
            .map_err(|_| anyhow!("unable to parse bitvec"))?;
        let base = u128::from_be_bytes(bytes);

        Ok(Self(base, bitvec))
    }
    pub fn new(
        current_time: u64,
        problem_id: &ProblemID,
        _contest_id: Option<&ContestId>,
        user_id: &Uuid,
    ) -> Self {
        let contest_id = _contest_id.unwrap_or(&ContestId(0));
        let mut base: u128 = 0;
        // offset de 41 bits para el timestamp
        base |= (current_time as u128) << (SUBMISSION_ID_BITS - TIMESTAMP_BITS);

        // offset de 41 bits  + 20 bits bits para el contest_id
        base |= u128::from(contest_id) << (SUBMISSION_ID_BITS - (TIMESTAMP_BITS + CONTEST_ID_BITS));

        // offset de 41 bits  + 29 bits  + 20 bits para el problem_id
        base |= u128::from(problem_id)
            << (SUBMISSION_ID_BITS - (TIMESTAMP_BITS + CONTEST_ID_BITS + PROBLEM_ID_BITS));

        let (_, time_mid, _, _) = user_id.as_fields();
        base |= (time_mid as u128)
            << (SUBMISSION_ID_BITS
                - (TIMESTAMP_BITS + CONTEST_ID_BITS + PROBLEM_ID_BITS + UUID_TIME_MID_BITS));

        let mut bv = BitVec::from_elem(128, false);
        let base_vec = BitVec::from_bytes(&base.to_be_bytes());
        bv.or(&base_vec);
        Self(base, bv)
    }
    pub fn as_u128(&self) -> u128 {
        self.0
    }
    pub fn get_contest_id(&self) -> Result<ContestId> {
        let mask_of_ones = (1 << CONTEST_ID_BITS) - 1;
        let mask_of_shifted_bits = self.0 >> (PROBLEM_ID_BITS + UUID_TIME_MID_BITS + EMPTY_BITS);
        Ok(ContestId((mask_of_ones & mask_of_shifted_bits).try_into()?))
    }

    pub fn get_contest_id_as_bit_vec(&self) -> BitVec {
        let mask_of_ones: u128 = (1 << CONTEST_ID_BITS) - 1;
        let mask_of_shifted_bits: u128 =
            mask_of_ones << (PROBLEM_ID_BITS + UUID_TIME_MID_BITS + EMPTY_BITS);
        //Ok(ContestId((mask_of_ones & mask_of_shifted_bits).try_into()?))
        let contest_id = BitVec::from_bytes(&(self.0 & mask_of_shifted_bits).to_be_bytes());
        let mut base = BitVec::from_elem(128, false);
        base.or(&contest_id);
        base
    }

    pub fn get_problem_id(&self) -> Result<ProblemID> {
        let mask_of_ones = (1 << PROBLEM_ID_BITS) - 1;
        let mask_of_shifted_bits = self.0 >> (UUID_TIME_MID_BITS + EMPTY_BITS);
        Ok(ProblemID((mask_of_ones & mask_of_shifted_bits).try_into()?))
    }
    pub fn get_problem_id_as_bit_vec(&self) -> BitVec {
        let mask_of_ones: u128 = (1 << PROBLEM_ID_BITS) - 1;
        let mask_of_shifted_bits: u128 = mask_of_ones << (UUID_TIME_MID_BITS + EMPTY_BITS);
        let problem_id = BitVec::from_bytes(&(self.0 & mask_of_shifted_bits).to_be_bytes());
        let mut base = BitVec::from_elem(128, false);
        base.or(&problem_id);
        base
    }

    pub fn get_timestamp(&self) -> Result<u64> {
        let mask_of_ones = (1 << TIMESTAMP_BITS) - 1;
        let mask_of_shifted_bits =
            self.0 >> (CONTEST_ID_BITS + PROBLEM_ID_BITS + UUID_TIME_MID_BITS + EMPTY_BITS);
        Ok((mask_of_ones & mask_of_shifted_bits).try_into()?)
    }

    pub fn as_bit_vec(&self) -> BitVec {
        BitVec::from_bytes(&self.0.to_be_bytes())
    }
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ProblemBodyMetadata {
    time_limit: usize,
    memory_limit: usize,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ProblemBody {
    pub metadata: ProblemBodyMetadata,
    pub identifier: Option<String>,
    pub name: String,
    pub input: String,
    pub output: String,
    pub problem: String,
    pub note: Option<String>,
}

#[derive(Debug, Clone, TS)]
#[ts(export)]
pub struct Problem {
    pub problem_id: ProblemID,
    pub name: Option<String>,
    pub policy_execution: PolicyExecution,
    // todo default implementation for system policy
    pub system_policy: Option<SystemPolicy>,
    pub test_cases: Vec<TestCase>,
    pub checker: Option<Checker>,
    pub validation_type: ValidatorType,
}
#[derive(Deserialize, Serialize, Debug, TS)]
#[ts(export)]
pub struct ProblemGetResponse {
    pub problem_id: u32,
    pub body: ProblemBody,
}
#[derive(Debug, Clone, TS)]
#[ts(export)]
pub struct Checker {
    pub checker: String,
}

#[derive(Debug, Clone, TS)]
#[ts(export)]
pub enum PolicyExecution {
    Checker,
    Interactive,
    AnswerFile,
}

#[derive(Debug, Clone, Default, TS)]
#[ts(export)]
pub struct TestCase {
    pub input_case: String,
    pub output_case: String,
    pub id: i32,
}

pub struct STestCase {
    pub input_case: String,
    pub output_case: String,
    pub id: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct TestCaseResult {
    pub status: Status,
    pub id: i32,
    #[serde(with = "external_struct")]
    #[ts(type = "{ stdout: any , stderr : any , status: any } | null")]
    pub output: Option<Output>,
}

#[derive(Debug, Clone, Default, TS)]
#[ts(export)]
pub struct SystemPolicy {
    pub memory_limit: usize,
    pub time_limit: usize,
}

#[derive(Clone, Debug, TS)]
#[ts(export)]
pub enum ValidatorType {
    TestLibChecker,
    LiteralChecker,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, TS)]
#[ts(export)]
pub struct ProblemExecutorResult {
    pub overall_result: Status,
    pub test_cases_results: Vec<TestCaseResult>,
    #[serde(with = "external_struct")]
    #[ts(type = "{ stdout: any , stderr : any , status: any } | null")]
    pub prepare_output: Option<Output>,
}
#[cfg(test)]
mod tests {
    use crate::consts::{CONTEST_ID_BITS, PROBLEM_ID_BITS};
    use crate::problem::{ContestId, ProblemID, SubmissionID};
    use rand::Rng;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn submission_id_is_being_generated_correctly() {
        let time: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .try_into()
            .unwrap();

        let user_id = uuid::Uuid::new_v4();
        let mut rng = rand::thread_rng();

        let max_limit: u32 = ((1u64 << PROBLEM_ID_BITS as u64) - 1).try_into().unwrap();
        let problem_id = ProblemID(rng.gen_range(1..max_limit));
        let contest_id = ContestId(rng.gen_range(1..(1 << CONTEST_ID_BITS)));
        let id = SubmissionID::new(time, &problem_id, Some(&contest_id), &user_id);

        assert_eq!(problem_id.as_u32(), id.get_problem_id().unwrap().as_u32());
        assert_eq!(contest_id.as_u32(), id.get_contest_id().unwrap().as_u32());
        assert_eq!(time, id.get_timestamp().unwrap());
        let bytes: [u8; 16] = id.as_bit_vec().to_owned().to_bytes().try_into().unwrap();
        assert_eq!(id.as_u128(), u128::from_be_bytes(bytes));
    }
}
