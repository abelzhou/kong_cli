use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetList {
    pub data: Vec<Target>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Target{
    pub created_at: f32,
    pub id: String,
    pub target: String,
    pub weight: u32
}