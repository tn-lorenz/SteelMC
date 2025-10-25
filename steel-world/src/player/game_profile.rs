use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameProfileProperty {
    pub name: String,
    // base 64
    pub value: String,
    // base 64
    pub signature: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GameProfileAction {
    ForcedNameChange,
    UsingBannedSkin,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GameProfile {
    pub id: Uuid,
    pub name: String,
    pub properties: Vec<GameProfileProperty>,
    #[serde(rename = "profileActions")]
    pub profile_actions: Option<Vec<GameProfileAction>>,
}
