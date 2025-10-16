use anyhow::{Result};
use pcd_rs::{PcdDeserialize, PcdSerialize};
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PcdDeserialize, PcdSerialize, Serialize, Deserialize)]
pub struct PointXYZ {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PointCloudPacket {
    pub total_points: usize,
    pub points: Vec<PointXYZ>,
}

impl PointCloudPacket {
    pub fn new(
        total_points: usize,
        points: Vec<PointXYZ>
    ) -> Self {
        Self {
            total_points,
            points,
        }
    }

    pub fn to_json_bytes(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(self)
            .map_err(|e| anyhow::anyhow!("Failed to serialize PointCloudPacket to JSON bytes: {}", e))
    }
}