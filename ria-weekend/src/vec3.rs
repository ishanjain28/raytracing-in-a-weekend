use std::ops::{Add, Div, Index, Mul, Sub};

pub struct Vec3 {
    inner: [f32; 3],
}

impl Vec3 {
    pub fn new(a: f32, b: f32, c: f32) -> Vec3 {
        Vec3 { inner: [a, b, c] }
    }

    pub fn x(&self) -> f32 {
        self.inner[0]
    }
    pub fn y(&self) -> f32 {
        self.inner[1]
    }
    pub fn z(&self) -> f32 {
        self.inner[2]
    }
    pub fn r(&self) -> f32 {
        self.inner[0]
    }
    pub fn g(&self) -> f32 {
        self.inner[1]
    }
    pub fn b(&self) -> f32 {
        self.inner[2]
    }

    pub fn length(&self) -> f32 {
        (self.inner[0] * self.inner[0]
            + self.inner[1] * self.inner[1]
            + self.inner[2] * self.inner[2])
            .sqrt()
    }

    pub fn sq_len(&self) -> f32 {
        self.inner[0] * self.inner[0]
            + self.inner[1] * self.inner[1]
            + self.inner[2] * self.inner[2]
    }

    pub fn dot(&self, v: &Vec3) -> f32 {
        self.inner[0] * v.inner[0] + self.inner[1] * v.inner[1] + self.inner[2] * v.inner[2]
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            inner: [
                self.inner[1] * v.inner[2] - self.inner[2] * v.inner[1],
                self.inner[2] * v.inner[0] - self.inner[0] * v.inner[2],
                self.inner[0] * v.inner[1] - self.inner[1] * v.inner[0],
            ],
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, o: Vec3) -> Vec3 {
        Vec3 {
            inner: [
                self.inner[0] + o.inner[0],
                self.inner[1] + o.inner[1],
                self.inner[2] + o.inner[2],
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, o: Vec3) -> Vec3 {
        Vec3 {
            inner: [
                self.inner[0] - o.inner[0],
                self.inner[1] - o.inner[1],
                self.inner[2] - o.inner[2],
            ],
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, o: Vec3) -> Vec3 {
        Vec3 {
            inner: [
                self.inner[0] * o.inner[0],
                self.inner[1] * o.inner[1],
                self.inner[2] * o.inner[2],
            ],
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, o: f32) -> Vec3 {
        Vec3 {
            inner: [self.inner[0] * o, self.inner[1] * o, self.inner[2] * o],
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, o: Vec3) -> Vec3 {
        Vec3 {
            inner: [
                self.inner[0] / o.inner[0],
                self.inner[1] / o.inner[1],
                self.inner[2] / o.inner[2],
            ],
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, o: f32) -> Vec3 {
        let o = 1.0 / o;
        Vec3 {
            inner: [self.inner[0] * o, self.inner[1] * o, self.inner[2] * o],
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, q: usize) -> &f32 {
        &self.inner[q]
    }
}
