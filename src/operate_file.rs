use pcd_rs::{Reader, WriterInit};
use anyhow::{Context, Result};
use crate::types::PointXYZ;


pub fn load_pcd_xyz(file_path: &str) -> Result<Vec<PointXYZ>> {
    let reader = Reader::open(file_path)
        .context("Failed to open PCD file")?;

    let points: Vec<PointXYZ> = reader
        .collect::<Result<Vec<PointXYZ>, _>>()
        .context("Failed to read PCD file")?;

    Ok(points)
}

pub fn save_pcd_xyz(
    points: &[PointXYZ],
    file_path: &str
) -> Result<()> {
    let mut writer = WriterInit {
        width: 1,
        height: points.len() as u64,
        viewpoint: Default::default(),
        data_kind: pcd_rs::DataKind::Ascii,
        schema: None,
    }
    .create(file_path)
    .context("Failed to create PCD file")?;

    for point in points {
        writer.push(point).context("Failed to write point to PCD file")?;
    }
    writer.finish().context("Failed to finish writing PCD file")?;

    Ok(())
}
