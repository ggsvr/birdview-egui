use super::Data;
pub use crate::math::Point;

pub type Pt = Option<Point>;

#[derive(Debug, Clone)]
pub struct PointData {
    pub points: [Pt; 3],
}

impl PointData {
    pub fn new() -> Self {
        PointData {
            points: [None; 3]
        }
    }

    pub fn with_pts(back: Pt, front: Pt, dest: Pt) -> Self {
        Self {
            points: [back, front, dest]
        }
    }
}

impl Data for PointData {
    type Inner = Pt;

    fn list(&self) -> &[Self::Inner; 3] {
        &self.points
    }
    fn list_mut(&mut self) -> &mut [Self::Inner; 3] {
        &mut self.points
    }
}