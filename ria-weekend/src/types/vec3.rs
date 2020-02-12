use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Copy, Clone)]
pub struct Vec3([f32; 3]);

impl Vec3 {
    #[inline]
    pub const fn new(a: f32, b: f32, c: f32) -> Vec3 {
        Vec3([a, b, c])
    }
    #[inline]
    pub fn x(&self) -> f32 {
        self[0]
    }
    #[inline]
    pub fn y(&self) -> f32 {
        self[1]
    }
    #[inline]
    pub fn z(&self) -> f32 {
        self[2]
    }
    #[inline]
    pub fn r(&self) -> f32 {
        self[0]
    }
    #[inline]
    pub fn g(&self) -> f32 {
        self[1]
    }
    #[inline]
    pub fn b(&self) -> f32 {
        self[2]
    }

    #[inline]
    pub fn length(&self) -> f32 {
        self.sq_len().sqrt()
    }

    #[inline]
    pub fn sq_len(&self) -> f32 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    #[inline]
    pub fn dot(&self, v: &Vec3) -> f32 {
        self[0] * v[0] + self[1] * v[1] + self[2] * v[2]
    }

    #[inline]
    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3([
            self[1] * v[2] - self[2] * v[1],
            self[2] * v[0] - self[0] * v[2],
            self[0] * v[1] - self[1] * v[0],
        ])
    }

    #[inline]
    pub fn make_unit_vector(&mut self) {
        let k = 1.0f32 / (self[0] * self[0] + self[1] * self[1] + self[2] * self[2]);
        self[0] *= k;
        self[1] *= k;
        self[2] *= k;
    }

    #[inline]
    pub fn unit_vector(&self) -> Vec3 {
        let length = self.length();
        Vec3([self[0] / length, self[1] / length, self[2] / length])
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, o: Vec3) -> Vec3 {
        Vec3([self[0] + o[0], self[1] + o[1], self[2] + o[2]])
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, o: Vec3) {
        self.0[0] += o.0[0];
        self.0[1] += o.0[1];
        self.0[2] += o.0[2];
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, o: Vec3) -> Vec3 {
        Vec3([self[0] - o[0], self[1] - o[1], self[2] - o[2]])
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, o: Vec3) {
        self[0] -= o[0];
        self[1] -= o[1];
        self[2] -= o[2];
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, o: Vec3) {
        self[0] *= o[0];
        self[1] *= o[1];
        self[2] *= o[2];
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, o: f32) {
        self[0] *= o;
        self[1] *= o;
        self[2] *= o;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, o: f32) -> Vec3 {
        Vec3([self[0] * o, self[1] * o, self[2] * o])
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, o: Vec3) -> Vec3 {
        Vec3([self[0] / o[0], self[1] / o[1], self[2] / o[2]])
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, o: f32) -> Vec3 {
        let o = 1.0 / o;
        Vec3([self[0] * o, self[1] * o, self[2] * o])
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, o: f32) {
        let o = 1.0 / o;
        self.0[0] /= o;
        self.0[1] /= o;
        self.0[2] /= o;
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, q: usize) -> &f32 {
        &self.0[q]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, q: usize) -> &mut f32 {
        &mut self.0[q]
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_fmt(format_args!("{} {} {}", self[0], self[1], self[2]))
    }
}
