use super::Positions;
use crate::prelude::*;

pub use serde::{Serialize,Deserialize};

///
/// Represents a set of points in 3D space, usually created with a scanner.
///
#[derive(Default, Clone)]
#[cfg_attr(feature = "serde-core", derive(Serialize, Deserialize))]
pub struct PointCloud {
    /// The positions of the points.
    pub positions: Positions,
    /// The colors of the points.
    pub colors: Option<Vec<Color>>,
}

impl std::fmt::Debug for PointCloud {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("PointCloud");
        d.field("positions", &self.positions.len());
        d.field("colors", &self.colors.as_ref().map(|v| v.len()));
        d.finish()
    }
}

impl PointCloud {
    ///
    /// Returns a point cloud whose points lie on the corners of an axis aligned unconnected cube with positions in the range `[-1..1]` in all axes.
    ///
    pub fn cube() -> Self {
        let positions = vec![
            vec3(-1.0, -1.0, -1.0),
            vec3(-1.0, -1.0, 1.0),
            vec3(-1.0, 1.0, -1.0),
            vec3(-1.0, 1.0, 1.0),
            vec3(1.0, -1.0, -1.0),
            vec3(1.0, -1.0, 1.0),
            vec3(1.0, 1.0, -1.0),
            vec3(1.0, 1.0, 1.0),
        ];
        Self {
            positions: Positions::F32(positions),
            ..Default::default()
        }
    }

    ///
    /// Computes the [AxisAlignedBoundingBox] for this point cloud.
    ///
    pub fn compute_aabb(&self) -> AxisAlignedBoundingBox {
        self.positions.compute_aabb()
    }
}
