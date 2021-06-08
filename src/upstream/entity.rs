use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetList {
    pub data: Vec<Target>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Target {
    pub created_at: f32,
    pub id: String,
    pub target: String,
    pub weight: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpstreamList {
    pub next: Option<String>,
    pub data: Vec<Upstream>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Upstream {
    pub created_at: f32,
    pub id: String,
    pub hash_on: String,
    pub name: String,
    pub slots: u32,
}
