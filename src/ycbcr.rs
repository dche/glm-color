//
// GLM-COLOR
//
// Copyright (c) 2015 The glm-color authors.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

use glm::*;
use super::space::ColorSpace;
use super::rgb::Rgb;
use std::mem;
use rand::{ Rand, Rng };

/// The YCbCr color model.
///
/// # See
///
/// Wikipedia page [YCbCr](http://en.wikipedia.org/wiki/YCbCr).
///
/// # Example
///
/// ```rust
/// use glm_color::*;
///
/// let brown_yuv: YCbCr = ColorSpace::from_rgb(BROWN);
/// assert_eq!(brown_yuv.y(), BROWN.lunimance());
/// ```
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct YCbCr {
    y:  f32,
    cb: f32,
    cr: f32
}

impl YCbCr {
    /// Constructs a `YCbCr` value.
    ///
    /// Parameter `y` is clampped to the range *[0, 1]*, and `cb` and `cr` are
    /// clampped to the range *[-0.5, 0.5]*.
    #[inline]
    pub fn new(y: f32, cb: f32, cr: f32) -> YCbCr {
        let lum = clamp(y, 0., 1.);
        let uv = clamp_s(vec2(cb, cr), -0.5, 0.5);
        YCbCr { y: lum, cb: uv.x, cr: uv.y }
    }

    /// Re-interprets a reference of `YCbCr` to `Vec3`.
    pub fn as_vec3(&self) -> &Vec3 {
        let v: &Vec3 = unsafe { mem::transmute(self) };
        v
    }

    /// Returns the Y` component, which is a value in the range *[0, 1]*.
    pub fn y(&self) -> f32 {
        self.y
    }

    /// Returns the`Cb` component, which is a value in the range *[-0.5, 0.5]*.
    pub fn cb(&self) -> f32 {
        self.cb
    }

    /// Returns the Cr` component, which is a value in the range *[-0.5, 0.5]*.
    pub fn cr(&self) -> f32 {
        self.cr
    }
}

impl Eq for YCbCr {}

impl ApproxEq for YCbCr {
    type BaseType = f32;
    #[inline]
    fn is_close_to(&self, other: &YCbCr, max_diff: f32) -> bool {
        self.as_vec3().is_close_to(other.as_vec3(), max_diff)
    }
}

impl Rand for YCbCr {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> YCbCr {
        let y = rng.gen::<f32>();
        let uv = rng.gen::<Vec2>() - 0.5;
        YCbCr::new(y, uv.x, uv.y)
    }
}

impl ColorSpace for YCbCr {
    /// # Note
    ///
    /// Constants defined in [ITU-R BT-709 conversion](http://en.wikipedia.org/wiki/YCbCr#ITU-R_BT.709_conversion)
    /// are used.
    #[inline]
    fn from_rgb(rgb: Rgb) -> YCbCr {
        // column major.
        // ITU.BT-709
        let clr_mat = mat3(
            0.2126, -0.1146,  0.5,
            0.7152, -0.3854, -0.4542,
            0.0722,  0.5,    -0.0458
        );
        let v = clr_mat.mul_v(rgb.as_vec3());
        YCbCr { y: v.x, cb: v.y, cr: v.z }
    }
    #[inline]
    fn to_rgb(&self) -> Rgb {
        // not exactly the inverse of color matrix in `from_rgb`,
        // so the rounding errors of conversion back and forth are large.
        let clr_mat = mat3(
            1.,      1.,     1.,
            0.,     -0.1873, 1.8556,
            1.5748, -0.4682, 0.
        );
        let v = clr_mat.mul_v(self.as_vec3());
        Rgb::new(v.x, v.y, v.z)
    }
}

/// Equivalent to `YCbCr::new()`.
#[inline]
pub fn ycbcr(y: f32, cb: f32, cr: f32) -> YCbCr {
    YCbCr::new(y, cb, cr)
}

#[cfg(test)]
mod test {
    use glm::*;
    use space::ColorSpace;
    use rgb::Rgb;
    use super::YCbCr;
    use quickcheck::*;

    #[test]
    fn test_to_rgb() {
        fn prop(clr: Rgb) -> bool {
            let yuv: YCbCr = ColorSpace::from_rgb(clr);
            let rgb = yuv.to_rgb();
            abs(yuv.cb) <= 0.5 &&
            abs(yuv.cr) <= 0.5 &&
            is_close_to(&rgb, &clr, 0.0001)
        }
        quickcheck(prop as fn(Rgb) -> bool);
    }
}
