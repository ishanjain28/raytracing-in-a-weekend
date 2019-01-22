use std::ops::{Add, Div, Index, IndexMut, Mul, MulAssign, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    inner: [f32; 3],
}

impl Vec3 {
    pub fn new(a: f32, b: f32, c: f32) -> Vec3 {
        Vec3 { inner: [a, b, c] }
    }

    pub fn x(&self) -> f32 {
        self[0]
    }
    pub fn y(&self) -> f32 {
        self[1]
    }
    pub fn z(&self) -> f32 {
        self[2]
    }
    pub fn r(&self) -> f32 {
        self[0]
    }
    pub fn g(&self) -> f32 {
        self[1]
    }
    pub fn b(&self) -> f32 {
        self[2]
    }

    pub fn length(&self) -> f32 {
        self.sq_len().sqrt()
    }

    pub fn sq_len(&self) -> f32 {
        self[0] * self[0]
            + self[1] * self[1]
            + self[2] * self[2]
    }

    pub fn dot(&self, v: &Vec3) -> f32 {
        self[0] * v[0] + self[1] * v[1] + self[2] * v[2]
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            inner: [
                self[1] * v[2] - self[2] * v[1],
                self[2] * v[0] - self[0] * v[2],
                self[0] * v[1] - self[1] * v[0],
            ],
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        let length = self.length();
        Vec3 {
            inner: [self[0] / length, self[1] / length, self[2] / length],
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, o: Vec3) -> Vec3 {
        Vec3 {
            inner: [
                self[0] + o[0],
                self[1] + o[1],
                self[2] + o[2],
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, o: Vec3) -> Vec3 {
        Vec3 {
            inner: [
                self[0] - o[0],
                self[1] - o[1],
                self[2] - o[2],
            ],
        }
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
        Vec3 {
            inner: [self[0] * o, self[1] * o, self[2] * o],
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, o: Vec3) -> Vec3 {
        Vec3 {
            inner: [
                self[0] / o[0],
                self[1] / o[1],
                self[2] / o[2],
            ],
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, o: f32) -> Vec3 {
        let o = 1.0 / o;
        Vec3 {
            inner: [self[0] * o, self[1] * o, self[2] * o],
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, q: usize) -> &f32 {
        &self.inner[q]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, q: usize) -> &mut f32 {
        &mut self.inner[q]
    }
}
