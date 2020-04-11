#[cfg(feature = "avx2")]
pub use avx2::*;

#[cfg(not(feature = "avx2"))]
pub use scalar::*;

#[cfg(feature = "avx2")]
mod avx2 {
    #[cfg(feature = "avx2")]
    use packed_simd::f64x4;

    use std::{
        fmt::{Display, Formatter, Result as FmtResult},
        ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    };
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub struct Vec3(f64x4);

    impl Vec3 {
        #[inline]
        pub const fn new(a: f64, b: f64, c: f64) -> Vec3 {
            Vec3(f64x4::new(a, b, c, 0.0))
        }

        #[inline]
        pub fn x(&self) -> f64 {
            self.0.extract(0)
        }
        #[inline]
        pub fn y(&self) -> f64 {
            self.0.extract(1)
        }
        #[inline]
        pub fn z(&self) -> f64 {
            self.0.extract(2)
        }
        #[inline]
        pub fn r(&self) -> f64 {
            self.0.extract(0)
        }
        #[inline]
        pub fn g(&self) -> f64 {
            self.0.extract(1)
        }
        #[inline]
        pub fn b(&self) -> f64 {
            self.0.extract(2)
        }

        #[inline]
        pub fn length(&self) -> f64 {
            self.sq_len().sqrt()
        }

        #[inline]
        pub fn sq_len(&self) -> f64 {
            let p = self.0.powf(f64x4::new(2.0, 2.0, 2.0, 2.0));
            p.sum()
        }

        #[inline]
        pub fn dot(&self, v: &Vec3) -> f64 {
            let p = self.0 * v.0;
            p.sum()
        }

        #[inline]
        pub fn cross(&self, v: &Vec3) -> Vec3 {
            let p1 = self.0 * f64x4::new(v.0.extract(1), v.0.extract(2), v.0.extract(0), 0.0);
            let p2 = self.0 * f64x4::new(v.0.extract(2), v.0.extract(0), v.0.extract(1), 0.0);

            Vec3(f64x4::new(
                p1.extract(1) - p2.extract(2),
                p1.extract(2) - p2.extract(0),
                p1.extract(0) - p2.extract(1),
                0.0,
            ))
        }

        #[inline]
        pub fn unit_vector(&self) -> Vec3 {
            let length = self.length();
            Vec3(self.0 / length)
        }
    }

    impl Add for Vec3 {
        type Output = Vec3;

        fn add(self, o: Vec3) -> Vec3 {
            Vec3(self.0 + o.0)
        }
    }

    impl AddAssign for Vec3 {
        fn add_assign(&mut self, o: Vec3) {
            self.0 += o.0
        }
    }

    impl Sub for Vec3 {
        type Output = Vec3;

        fn sub(self, o: Vec3) -> Vec3 {
            Vec3(self.0 - o.0)
        }
    }

    impl SubAssign for Vec3 {
        fn sub_assign(&mut self, o: Vec3) {
            self.0 -= o.0
        }
    }

    impl Neg for Vec3 {
        type Output = Vec3;

        fn neg(self) -> Vec3 {
            Vec3(-self.0)
        }
    }

    impl MulAssign<Vec3> for Vec3 {
        fn mul_assign(&mut self, o: Vec3) {
            self.0 *= o.0
        }
    }

    impl MulAssign<f64> for Vec3 {
        fn mul_assign(&mut self, o: f64) {
            self.0 *= o
        }
    }

    impl Mul<f64> for Vec3 {
        type Output = Vec3;
        fn mul(self, o: f64) -> Vec3 {
            Vec3(self.0 * o)
        }
    }

    impl Mul<Vec3> for Vec3 {
        type Output = Vec3;
        fn mul(self, o: Vec3) -> Vec3 {
            Vec3(self.0 * o.0)
        }
    }

    impl Div<Vec3> for Vec3 {
        type Output = Vec3;

        fn div(self, o: Vec3) -> Vec3 {
            Vec3(self.0 / o.0)
        }
    }

    impl Div<f64> for Vec3 {
        type Output = Vec3;

        fn div(self, o: f64) -> Vec3 {
            let o = 1.0 / o;
            self * o
        }
    }

    impl DivAssign<f64> for Vec3 {
        fn div_assign(&mut self, o: f64) {
            let o = 1.0 / o;
            *self *= o;
        }
    }

    impl Display for Vec3 {
        fn fmt(&self, f: &mut Formatter) -> FmtResult {
            f.write_fmt(format_args!(
                "{} {} {}",
                self.0.extract(0),
                self.0.extract(1),
                self.0.extract(2)
            ))
        }
    }

    #[test]
    fn vec3_test() {
        let v = Vec3::new(0.5, 0.6, 0.8);
        let q = Vec3::new(0.4, 0.2, 0.1);
        let cross = Vec3::new(
            -0.10000000000000003,
            0.2700000000000001,
            -0.13999999999999999,
        );
        let unit_vector = Vec3::new(0.4472135954999579, 0.5366563145999494, 0.7155417527999327);
        let add = Vec3::new(0.9, 0.8, 0.9);
        let sub = Vec3::new(0.09999999999999998, 0.39999999999999997, 0.7000000000000001);
        let mul = Vec3::new(0.2, 0.12, 0.08000000000000002);
        let div = Vec3::new(1.25, 2.9999999999999996, 8.0);

        assert_eq!(v.x(), 0.5);
        assert_eq!(v.y(), 0.6);
        assert_eq!(v.z(), 0.8);
        assert_eq!(v.r(), 0.5);
        assert_eq!(v.g(), 0.6);
        assert_eq!(v.b(), 0.8);
        assert_eq!(v.length(), 1.118033988749895);
        assert_eq!(v.sq_len(), 1.25);
        assert_eq!(v.dot(&q), 0.4);
        assert_eq!(v.cross(&q), cross);
        assert_eq!(v.unit_vector(), unit_vector);
        assert_eq!(v + q, add);
        assert_eq!(v - q, sub);
        assert_eq!(v * q, mul);
        assert_eq!(v / q, div);
    }
}

mod scalar {
    use std::{
        fmt::{Display, Formatter, Result as FmtResult},
        ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    };
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub struct Vec3(f64, f64, f64);

    impl Vec3 {
        #[inline]
        pub const fn new(a: f64, b: f64, c: f64) -> Vec3 {
            Vec3(a, b, c)
        }

        #[inline]
        pub fn x(&self) -> f64 {
            self.0
        }
        #[inline]
        pub fn y(&self) -> f64 {
            self.1
        }
        #[inline]
        pub fn z(&self) -> f64 {
            self.2
        }
        #[inline]
        pub fn r(&self) -> f64 {
            self.0
        }
        #[inline]
        pub fn g(&self) -> f64 {
            self.1
        }
        #[inline]
        pub fn b(&self) -> f64 {
            self.2
        }

        #[inline]
        pub fn length(&self) -> f64 {
            self.sq_len().sqrt()
        }

        #[inline]
        pub fn sq_len(&self) -> f64 {
            self.0 * self.0 + self.1 * self.1 + self.2 * self.2
        }

        #[inline]
        pub fn dot(&self, v: &Vec3) -> f64 {
            self.0 * v.0 + self.1 * v.1 + self.2 * v.2
        }

        #[inline]
        pub fn cross(&self, v: &Vec3) -> Vec3 {
            Vec3(
                self.1 * v.2 - self.2 * v.1,
                self.2 * v.0 - self.0 * v.2,
                self.0 * v.1 - self.1 * v.0,
            )
        }

        #[inline]
        pub fn unit_vector(&self) -> Vec3 {
            let length = self.length();
            Vec3(self.0 / length, self.1 / length, self.2 / length)
        }
    }

    impl Add for Vec3 {
        type Output = Vec3;

        fn add(self, o: Vec3) -> Vec3 {
            Vec3::new(self.0 + o.0, self.1 + o.1, self.2 + o.2)
        }
    }

    impl AddAssign for Vec3 {
        fn add_assign(&mut self, o: Vec3) {
            self.0 += o.0;
            self.1 += o.1;
            self.2 += o.2;
        }
    }

    impl Sub for Vec3 {
        type Output = Vec3;

        fn sub(self, o: Vec3) -> Vec3 {
            Vec3::new(self.0 - o.0, self.1 - o.1, self.2 - o.2)
        }
    }

    impl SubAssign for Vec3 {
        fn sub_assign(&mut self, o: Vec3) {
            self.0 -= o.0;
            self.1 -= o.1;
            self.2 -= o.2;
        }
    }

    impl Neg for Vec3 {
        type Output = Vec3;

        fn neg(self) -> Vec3 {
            Vec3(-self.0, -self.1, -self.2)
        }
    }

    impl MulAssign<Vec3> for Vec3 {
        fn mul_assign(&mut self, o: Vec3) {
            self.0 *= o.0;
            self.1 *= o.1;
            self.2 *= o.2;
        }
    }

    impl MulAssign<f64> for Vec3 {
        fn mul_assign(&mut self, o: f64) {
            self.0 *= o;
            self.1 *= o;
            self.2 *= o;
        }
    }

    impl Mul<f64> for Vec3 {
        type Output = Vec3;
        fn mul(self, o: f64) -> Vec3 {
            Vec3::new(self.0 * o, self.1 * o, self.2 * o)
        }
    }

    impl Mul<Vec3> for Vec3 {
        type Output = Vec3;
        fn mul(self, o: Vec3) -> Vec3 {
            Vec3::new(self.0 * o.0, self.1 * o.1, self.2 * o.2)
        }
    }

    impl Div<Vec3> for Vec3 {
        type Output = Vec3;

        fn div(self, o: Vec3) -> Vec3 {
            Vec3::new(self.0 / o.0, self.1 / o.1, self.2 / o.2)
        }
    }

    impl Div<f64> for Vec3 {
        type Output = Vec3;

        fn div(self, o: f64) -> Vec3 {
            let o = 1.0 / o;
            self * o
        }
    }

    impl DivAssign<f64> for Vec3 {
        fn div_assign(&mut self, o: f64) {
            let o = 1.0 / o;
            *self *= o;
        }
    }

    impl Display for Vec3 {
        fn fmt(&self, f: &mut Formatter) -> FmtResult {
            f.write_fmt(format_args!("{} {} {}", self.0, self.1, self.2))
        }
    }

    #[test]
    fn vec3_test() {
        let v = Vec3::new(0.5, 0.6, 0.8);
        let q = Vec3::new(0.4, 0.2, 0.1);
        let cross = Vec3::new(
            -0.10000000000000003,
            0.2700000000000001,
            -0.13999999999999999,
        );
        let unit_vector = Vec3::new(0.4472135954999579, 0.5366563145999494, 0.7155417527999327);
        let add = Vec3::new(0.9, 0.8, 0.9);
        let sub = Vec3::new(0.09999999999999998, 0.39999999999999997, 0.7000000000000001);
        let mul = Vec3::new(0.2, 0.12, 0.08000000000000002);
        let div = Vec3::new(1.25, 2.9999999999999996, 8.0);

        assert_eq!(v.x(), 0.5);
        assert_eq!(v.y(), 0.6);
        assert_eq!(v.z(), 0.8);
        assert_eq!(v.r(), 0.5);
        assert_eq!(v.g(), 0.6);
        assert_eq!(v.b(), 0.8);
        assert_eq!(v.length(), 1.118033988749895);
        assert_eq!(v.sq_len(), 1.25);
        assert_eq!(v.dot(&q), 0.4);
        assert_eq!(v.cross(&q), cross);
        assert_eq!(v.unit_vector(), unit_vector);
        assert_eq!(v + q, add);
        assert_eq!(v - q, sub);
        assert_eq!(v * q, mul);
        assert_eq!(v / q, div);
    }
}
