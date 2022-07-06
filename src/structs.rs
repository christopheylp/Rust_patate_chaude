use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Message {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderboard(PublicLeaderboard),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Welcome {
    version: u8,
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

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PublicPlayer {
    name: String,
    score: u32,
}


#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PublicLeaderboard {
    players: Vec<PublicPlayer>
}
