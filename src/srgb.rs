//
// Color Manipulating Library, using GLM.
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
use glm::ext::recip;
use super::space::{ ColorSpace, from_rgb };
use super::rgb::Rgb;
use std::mem;
use rand::{ Rand, Rng };

/// The sRGB color space.
///
/// # See
///
/// Wikipedia page [sRGB](http://en.wikipedia.org/wiki/SRGB), from where the
/// numbers used in conversion between linear RGB and sRGB are got.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Srgb {
    r: f32,
    g: f32,
    b: f32
}

impl Srgb {

    /// Creates an `Srgb` value.
    ///
    /// Parameters are clampped to range *[0, 1]*.
    #[inline]
    pub fn new(r: f32, g: f32, b: f32) -> Srgb {
        let v3 = clamp_s(vec3(r, g, b), 0., 1.);
        Srgb { r: v3.x, g: v3.y, b: v3.z }
    }

    /// Returns the value of red channel of _self_.
    pub fn red(&self) -> f32 {
        self.r
    }

    /// Returns the value of green channel of _self_.
    pub fn green(&self) -> f32 {
        self.g
    }

    /// Returns the value of blue channel of _self_.
    pub fn blue(&self) -> f32 {
        self.b
    }

    /// Re-interprets a reference of `Srgb` to `Vec3`.
    pub fn as_vec3(&self) -> &Vec3 {
        let v: &Vec3 = unsafe { mem::transmute(self) };
        v
    }
}

impl Rand for Srgb {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> Srgb {
        let rgb = rng.gen::<Rgb>();
        from_rgb(rgb)
    }
}

impl Eq for Srgb {}

impl ApproxEq for Srgb {
    type BaseType = f32;
    #[inline]
    fn is_close_to(&self, other: &Srgb, max_diff: f32) -> bool {
        self.as_vec3().is_close_to(other.as_vec3(), max_diff)
    }
}

impl ColorSpace for Srgb {
    #[inline]
    fn from_rgb(rgb: Rgb) -> Srgb {
        let v3 = rgb.as_vec3().map(|f| {
            // numbers from Wikipedia.
            if f <= 0.0031308 {
                f * 12.92
            } else {
                let a = 0.055_f32;
                (1. + a) * pow(f, recip(2.4_f32)) - a
            }
        });
        Srgb { r: v3.x, g: v3.y, b: v3.z }
    }
    #[inline]
    fn to_rgb(&self) -> Rgb {
        let v3 = self.as_vec3().map(|f| {
            if f <= 0.04045 {
                f / 12.92
            } else {
                let a = 0.055_f32;
                pow((f + a) * recip(1. + a), 2.4)
            }
        });
        Rgb::new(v3.x, v3.y, v3.z)
    }
}

/// Equivalent to `Srgb::new()`.
pub fn srgb(r: f32, g: f32, b: f32) -> Srgb {
    Srgb::new(r, g, b)
}

#[cfg(test)]
mod test {
    use glm::*;
    use space::ColorSpace;
    use rgb::Rgb;
    use super::Srgb;
    use quickcheck::*;

    #[test]
    fn test_to_rgb() {
        fn prop(clr: Rgb) -> bool {
            let s: Srgb = ColorSpace::from_rgb(clr);
            let rgb = s.to_rgb();
            is_close_to(&rgb, &clr, 0.0001)
        }
        quickcheck(prop as fn(Rgb) -> bool);
    }
}
