use super::Data;

pub struct ColorData {

    pub colors: [Color; 3],

}

impl ColorData {
    pub fn new(back: Color, front: Color, dest: Color) -> Self {
        Self {
            colors: [
                back,
                front,
                dest,
            ],
        }
    }

}

impl Data for ColorData {
    type Inner = Color;

    fn list(&self) -> &[Self::Inner; 3] {
        &self.colors
    }

    fn list_mut(&mut self) -> &mut [Self::Inner; 3] {
        &mut self.colors
    }

}

pub struct ColorDataHSV {

    pub colors: [ColorHSV; 3],

}

impl ColorDataHSV {
    pub fn new(back: ColorHSV, front: ColorHSV, dest: ColorHSV) -> Self {
        Self {
            colors: [
                back,
                front,
                dest,
            ],
        }
    }

}

impl Data for ColorDataHSV {
    type Inner = ColorHSV;

    fn list(&self) -> &[Self::Inner; 3] {
        &self.colors
    }

    fn list_mut(&mut self) -> &mut [Self::Inner; 3] {
        &mut self.colors
    }

}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorHSV {
    pub channels: [u16; 3]
}

impl ColorHSV {
    pub fn new(h: u16, s: u16, v: u16) -> Self {
        Self {
            channels: [h,s,v],
        }
    }

    pub fn h(&self) -> u16 {
        self.channels[0]
    }
    pub fn s(&self) -> u16 {
        self.channels[1]
    }
    pub fn v(&self) -> u16 {
        self.channels[2]
    }

    pub fn h_mut(&mut self) -> &mut u16 {
        &mut self.channels[0]
    }
    pub fn s_mut(&mut self) -> &mut u16 {
        &mut self.channels[1]
    }
    pub fn v_mut(&mut self) -> &mut u16 {
        &mut self.channels[2]
    }

    pub fn red() -> Self {
        Self { channels: [0,100,100] }
    }
    pub fn green() -> Self {
        Self { channels: [120,100,100] }
    }
    pub fn blue() -> Self {
        Self { channels: [240,100,100] }
    }
    pub fn black() -> Self {
        Self { channels: [0,0,0] }
    }
    pub fn white() -> Self {
        Self { channels: [0,0,100] }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub channels: [u8; 3]
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            channels: [r, g, b]
        }
    }

    pub fn black() -> Self {
        Self {
            channels: [0; 3]
        }
    }
    pub fn white() -> Self {
        Self {
            channels: [255; 3]
        }
    }
    pub fn red() -> Self {
        Self {
            channels: [255, 0, 0]
        }
    }
    pub fn green() -> Self {
        Self {
            channels: [0, 255, 0]
        }
    }
    pub fn blue() -> Self {
        Self {
            channels: [0, 0, 255]
        }
    }

    pub fn r(&self) -> u8 {
        self.channels[0]
    }
    pub fn g(&self) -> u8 {
        self.channels[1]
    }
    pub fn b(&self) -> u8 {
        self.channels[2]
    }
    pub fn r_mut(&mut self) -> &mut u8 {
        &mut self.channels[0]
    }
    pub fn g_mut(&mut self) -> &mut u8 {
        &mut self.channels[1]
    }
    pub fn b_mut(&mut self) -> &mut u8 {
        &mut self.channels[2]
    }
}

// Operations for color




use std::ops;

impl ops::Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            channels: [
                self.channels[0].saturating_add(rhs.channels[0]),
                self.channels[1].saturating_add(rhs.channels[1]),
                self.channels[2].saturating_add(rhs.channels[2]),
            ]
        }
    }
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.channels[0] = self.channels[0].saturating_add(rhs.channels[0]);
        self.channels[1] = self.channels[1].saturating_add(rhs.channels[1]);
        self.channels[2] = self.channels[2].saturating_add(rhs.channels[2]);
    }
}

impl ops::Sub for Color {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            channels: [
                self.channels[0].saturating_sub(rhs.channels[0]),
                self.channels[1].saturating_sub(rhs.channels[1]),
                self.channels[2].saturating_sub(rhs.channels[2]),
            ]
        }
    }
}

impl ops::SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        self.channels[0] = self.channels[0].saturating_sub(rhs.channels[0]);
        self.channels[1] = self.channels[1].saturating_sub(rhs.channels[1]);
        self.channels[2] = self.channels[2].saturating_sub(rhs.channels[2]);
    }
}

impl ops::Mul for Color {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            channels: [
                self.channels[0].saturating_mul(rhs.channels[0]),
                self.channels[1].saturating_mul(rhs.channels[1]),
                self.channels[2].saturating_mul(rhs.channels[2]),
            ]
        }
    }
}

impl ops::MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        self.channels[0] = self.channels[0].saturating_mul(rhs.channels[0]);
        self.channels[1] = self.channels[1].saturating_mul(rhs.channels[1]);
        self.channels[2] = self.channels[2].saturating_mul(rhs.channels[2]);
    }
}

impl ops::Div for Color {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            channels: [
                self.channels[0] / rhs.channels[0],
                self.channels[1] / rhs.channels[1],
                self.channels[2] / rhs.channels[2],
            ]
        }
    }
}

impl ops::DivAssign for Color {
    fn div_assign(&mut self, rhs: Self) {
        self.channels[0] /= rhs.channels[0];
        self.channels[1] /= rhs.channels[1];
        self.channels[2] /= rhs.channels[2];
    }
}

impl ops::Rem for Color {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            channels: [
                self.channels[0] % rhs.channels[0],
                self.channels[1] % rhs.channels[1],
                self.channels[2] % rhs.channels[2],
            ]
        }
    }
}

impl ops::RemAssign for Color {
    fn rem_assign(&mut self, rhs: Self) {
        self.channels[0] %= rhs.channels[0];
        self.channels[1] %= rhs.channels[1];
        self.channels[2] %= rhs.channels[2];
    }
}

impl ops::Neg for Color {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            channels: [
                255 - self.channels[0],
                255 - self.channels[1],
                255 - self.channels[2],
            ]
        }
    }
}


impl ops::Add<u8> for Color {
    type Output = Self;
    fn add(self, rhs: u8) -> Self::Output {
        Self {
            channels: [
                self.channels[0].saturating_add(rhs),
                self.channels[1].saturating_add(rhs),
                self.channels[2].saturating_add(rhs),
            ]
        }
    }
}

impl ops::AddAssign<u8> for Color {
    fn add_assign(&mut self, rhs: u8) {
        self.channels[0] = self.channels[0].saturating_add(rhs);
        self.channels[1] = self.channels[1].saturating_add(rhs);
        self.channels[2] = self.channels[2].saturating_add(rhs);
    }
}

impl ops::Sub<u8> for Color {
    type Output = Self;
    fn sub(self, rhs: u8) -> Self::Output {
        Self {
            channels: [
                self.channels[0].saturating_sub(rhs),
                self.channels[1].saturating_sub(rhs),
                self.channels[2].saturating_sub(rhs),
            ]
        }
    }
}

impl ops::SubAssign<u8> for Color {
    fn sub_assign(&mut self, rhs: u8) {
        self.channels[0] = self.channels[0].saturating_sub(rhs);
        self.channels[1] = self.channels[1].saturating_sub(rhs);
        self.channels[2] = self.channels[2].saturating_sub(rhs);
    }
}

impl ops::Mul<u8> for Color {
    type Output = Self;
    fn mul(self, rhs: u8) -> Self::Output {
        Self {
            channels: [
                self.channels[0].saturating_mul(rhs),
                self.channels[1].saturating_mul(rhs),
                self.channels[2].saturating_mul(rhs),
            ]
        }
    }
}

impl ops::MulAssign<u8> for Color {
    fn mul_assign(&mut self, rhs: u8) {
        self.channels[0] = self.channels[0].saturating_mul(rhs);
        self.channels[1] = self.channels[1].saturating_mul(rhs);
        self.channels[2] = self.channels[2].saturating_mul(rhs);
    }
}

impl ops::Div<u8> for Color {
    type Output = Self;
    fn div(self, rhs: u8) -> Self::Output {
        Self {
            channels: [
                self.channels[0] / rhs,
                self.channels[1] / rhs,
                self.channels[2] / rhs,
            ]
        }
    }
}

impl ops::DivAssign<u8> for Color {
    fn div_assign(&mut self, rhs: u8) {
        self.channels[0] /= rhs;
        self.channels[1] /= rhs;
        self.channels[2] /= rhs;
    }
}

impl ops::Rem<u8> for Color {
    type Output = Self;
    fn rem(self, rhs: u8) -> Self::Output {
        Self {
            channels: [
                self.channels[0] % rhs,
                self.channels[1] % rhs,
                self.channels[2] % rhs,
            ]
        }
    }
}

impl ops::RemAssign<u8> for Color {
    fn rem_assign(&mut self, rhs: u8) {
        self.channels[0] %= rhs;
        self.channels[1] %= rhs;
        self.channels[2] %= rhs;
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_ops() {
        assert_eq!(Color::new(75, 75, 75) * Color::new(100, 100, 100), Color::white());
        assert_eq!(Color::new(50, 50, 50) - Color::new(20, 20, 20), Color::new(30, 30, 30));
        assert_eq!(Color::new(30, 30, 30) - 20, Color::new(10, 10, 10));
        assert_eq!(Color::new(30, 30, 30) - 40, Color::new(0, 0, 0));
    }

}
