use super::Data;
use serialport::SerialPort;
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

    pub fn write_serial(&self, port: &mut dyn SerialPort) -> Result<(), Box<dyn std::error::Error>> {
        let back = self.back().ok_or("missing back point")?;
        let front = self.front().ok_or("missing front point")?;
        let dest = self.dest().ok_or("missing dest point")?;

        let data = format!("{:.2},{:.2};{:.2},{:.2};{:.2},{:.2}!",
            back.x, back.y,
            front.x, front.y,
            dest.x, dest.y
        ).into_bytes();

        port.write(&data)?;


        Ok(())
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