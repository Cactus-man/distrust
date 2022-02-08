use serde::{Deserialize, Serialize};

pub struct UpdatePresence {
    since: Option<u64>,
    activities: Vec<Activity>,
    status: StatusType,
    afk: bool,
}

#[derive(Deserialize, Serialize)]
struct Activity {
    name: String,
    #[serde(rename = "type")]
    activity_type: u8,
    url: Option<String>,
    created_at: u64,
    timestamps: (),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum StatusType {
    Online,
    #[serde(rename = "dnd")]
    DoNotDisturb,
    Idle,
    Invisible,
    Offline,
}
