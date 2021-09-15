pub mod color;
pub mod point;
pub use color::*;
pub use point::*;

pub trait Data {
    type Inner;

    fn list(&self) -> &[Self::Inner; 3];
    fn list_mut(&mut self) -> &mut [Self::Inner; 3];

    fn back(&self) -> &Self::Inner {
        &self.list()[0]
    }
    fn front(&self) -> &Self::Inner {
        &self.list()[1]
    }
    fn dest(&self) -> &Self::Inner {
        &self.list()[2]
    }

    fn back_mut(&mut self) -> &mut Self::Inner {
        &mut self.list_mut()[0]
    }
    fn front_mut(&mut self) -> &mut Self::Inner {
        &mut self.list_mut()[1]
    }
    fn dest_mut(&mut self) -> &mut Self::Inner {
        &mut self.list_mut()[2]
    }
}