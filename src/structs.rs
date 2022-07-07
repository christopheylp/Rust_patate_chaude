use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Message {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Challenge),
    EndOfGame(EndOfGame),
    RoundSummary(RoundSummary),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Welcome {
    version: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ReportedChallengeResult {
    name: String,
    value: String
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Challenge {
    MD5HashCash: ChallengeInput,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ChallengeResult {
    result: ChallengeAnswer,
    next_target: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ChallengeAnswer {
    ChallengeName(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct EndOfGame {
    leaderBoard: Vec<PublicPlayer>,
}


#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ChallengeInput {
    complexity: u32,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Subscribe {
    pub(crate) name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum SubscribeResult {
    Ok,
    Err(SubscribeError)
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) enum SubscribeError {
    AlreadyRegistered,
    InvalidName
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct PublicPlayer {
    pub(crate) name: String,
    pub(crate) stream_id: String,
    pub(crate) score: i32,
    pub(crate) steps: u32,
    pub(crate) is_active: bool,
    pub(crate) total_used_time: f64,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MD5HashCashInput {
    pub complexity: u32,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MD5HashCashOutput {
    pub(crate) seed: u64,
    pub(crate) hashcode: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct MD5HashCashChallenge {
    pub(crate) input: MD5HashCashInput
}