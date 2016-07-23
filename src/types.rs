extern crate nalgebra as na;

use std::convert::{From};
use self::na::Matrix4;

pub type Id = i64;
pub struct Transform(Matrix4<f32>);

impl Transform {
    fn from_vec(value: Vec<f32>) -> Transform {
        Transform(Matrix4::new(
            value[0],
            value[1],
            value[2],
            value[3],
            value[4],
            value[5],
            value[6],
            value[7],
            value[8],
            value[9],
            value[10],
            value[11],
            value[12],
            value[13],
            value[14],
            value[15]
        ))
    }

    pub fn to_vec(&self) -> Vec<f32> {
        vec![
            self.0.m11,
            self.0.m21,
            self.0.m31,
            self.0.m41,
            self.0.m12,
            self.0.m22,
            self.0.m32,
            self.0.m42,
            self.0.m13,
            self.0.m23,
            self.0.m33,
            self.0.m43,
            self.0.m14,
            self.0.m24,
            self.0.m34,
            self.0.m44
        ]
    }
}
