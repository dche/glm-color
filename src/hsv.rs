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
use glm::ext::*;
use super::space::ColorSpace;
use super::rgb::Rgb;
use std::mem;
use rand::{ Rand, Rng, thread_rng };

/// The HSV color space.
///
/// # See
///
/// - ["HSV" in Wikipedia](http://en.wikipedia.org/wiki/HSL_and_HSV).
/// - An excellent [tutorial of color theory in worqx.com](http://www.worqx.com/color/index.htm).
/// - [How to choose colours procedurally](http://devmag.org.za/2012/07/29/how-to-choose-colours-procedurally-algorithms/).
/// - [Tints and shades](http://en.wikipedia.org/wiki/Tints_and_shades).
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Hsv {
    h: f32,
    s: f32,
    v: f32
}

impl Rand for Hsv {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> Hsv {
        let h = rng.gen::<f32>() * f32::tau();
        let s = rng.gen();
        let b = rng.gen();
        Hsv { h: h, s: s, v: b }
    }
}

impl Hsv {
    /// Constructs an `Hsv` value from given `hue`, `saturation` and
    /// `brightness` values.
    ///
    /// Parameter `hue` is clampped to the interval _[0, 2π)_, and
    /// `saturation` and `brightness` are clampped to interval _[0, 1]_.
    ///
    /// # Example
    ///
    /// ```rust
    /// use glm_color::*;
    ///
    /// let red = Hsv::new(7., 2., 1.);
    /// assert_eq!(red.hue(), 0.);
    /// assert_eq!(red.saturation(), 1.);
    /// ```
    #[inline]
    pub fn new(hue: f32, saturation: f32, brightness: f32) -> Hsv {
        let pi2 = tau();
        let mut h = clamp(hue, 0., pi2);
        if h == pi2 {
            h = 0.
        };
        let s = clamp(saturation, 0., 1.);
        let b = clamp(brightness, 0., 1.);
        Hsv { h: h, s: s, v: b }
    }

    /// Constructs an `Hsv` value by randomly choosing values for each of the
    /// three HSV channels using the thread local RNG.
    ///
    /// # Note
    ///
    /// As the `Rgb::rand()` function, this function is not good for
    /// generating the whole color palette.
    #[inline]
    pub fn rand() -> Hsv {
        let mut rng = thread_rng();
        rng.gen()
    }

    /// Constructs an `Hsv` from hue value `degree`, which is the angle on the
    /// color wheel.
    ///
    /// Both saturation and brightness of the returned value are set to `1.0`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate glm;
    /// # extern crate glm_color;
    /// # fn main() {
    /// use glm::*;
    /// use glm_color::*;
    ///
    /// let y = Hsv::from_hue(radians(60.));
    /// assert_eq!(y.hue(), glm::radians(60.));
    /// assert_eq!(y.saturation(), 1.);
    /// assert_eq!(y.brightness(), 1.);
    /// # }
    /// ```
    #[inline]
    pub fn from_hue(h: f32) -> Hsv {
        let mut clr = Hsv { h: 0., s: 1., v: 1. };
        clr.set_hue(h);
        clr
    }

    /// Returns the hue of _self_.
    #[inline]
    pub fn hue(&self) -> f32 {
        self.h
    }

    /// Returns the saturation of _self_.
    #[inline]
    pub fn saturation(&self) -> f32 {
        self.s
    }

    /// Returns the brightness of _self_.
    #[inline]
    pub fn brightness(&self) -> f32 {
        self.v
    }

    /// Changes _self_'s hue value to `h`.
    ///
    /// The parameter `h` is clampped to the range [0, 2π).
    #[inline]
    pub fn set_hue(&mut self, h: f32) {
        let pi2 = tau();
        let mut hv = clamp(h, 0., pi2);
        if hv == pi2 {
            hv = 0.;
        }
        self.h = hv
    }

    /// Returns a new `Hsv` value with given hue value `h`, and saturation and
    /// brightness values from _self_.
    #[inline]
    pub fn with_hue(&self, h: f32) -> Hsv {
        let mut c = *self;
        c.set_hue(h);
        c
    }

    /// Changes _self_'s saturation value to `s`.
    ///
    /// The parameter 's' is clampped to the rnage [0, 1];
    #[inline]
    pub fn set_saturation(&mut self, s: f32) {
        self.s = clamp(s, 0., 1.);
    }

    /// Returns a new `Hsv` value with given saturation value `s`, and hue and
    /// brightness values from _self_.
    #[inline]
    pub fn with_saturation(&self, s: f32) -> Hsv {
        let mut c = *self;
        c.set_saturation(s);
        c
    }

    /// Changes _self_'s brightness value to `b`.
    ///
    /// The parameter 'b' will be clampped to the rnage [0, 1];
    #[inline]
    pub fn set_brightness(&mut self, b: f32) {
        self.v = clamp(b, 0., 1.);
    }

    /// Returns a new `Hsv` value with given brightness value `b`, and hue and
    /// saturation values from _self_.
    #[inline]
    pub fn with_brightness(&self, b: f32) -> Hsv {
        let mut c = *self;
        c.set_brightness(b);
        c
    }

    /// Re-interpret the reference of `Hsv` to `Vec3`.
    #[inline(always)]
    pub fn as_vec3(&self) -> &Vec3 {
        let vec: &Vec3 = unsafe { mem::transmute(self) };
        vec
    }

    /// Returns the complementary color of _self_.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate glm;
    /// # extern crate glm_color;
    /// # fn main() {
    /// use glm::*;
    /// use glm::ext::pi;
    /// use glm_color::*;
    ///
    /// let red = Hsv::from_hue(0.);
    /// let green = Hsv::from_hue(pi());
    /// assert_eq!(red.complement(), green);
    /// # }
    /// ```
    #[inline]
    pub fn complement(&self) -> Hsv {
        self.with_hue(fmod(self.hue() + f32::pi(), tau()))
    }

    /// Returns a pair of colors that are splited from the complementary color
    /// of _self_,
    ///
    /// The 2 colors have same distances to the complementary color on the
    /// color wheel. In our implementation, this distance is fixed to `30`
    /// degrees.
    #[inline]
    pub fn split_complement(&self) -> (Self, Self) {
        let pi2 = tau();
        let h1 = fmod(self.hue() + radians(150.), pi2);
        let h2 = fmod(self.hue() + radians(210.), pi2);
        (self.with_hue(h1), self.with_hue(h2))
    }

    /// Returns 2 pairs of complementary colors. The first colors of both pairs
    /// are the result of `split_complement` of _self_.
    ///
    // TODO: Example
    #[inline]
    pub fn double_complement(&self) -> ((Self, Self), (Self, Self)) {
        let (c1, c2) = self.split_complement();
        ((c1.complement(), c1), (c2.complement(), c2))
    }

    /// Returns other 2 colors of triad colors that includes _self_.
    ///
    /// # Example
    ///
    /// ```rust
    /// # extern crate glm;
    /// # extern crate glm_color;
    /// # fn main() {
    /// use glm::*;
    /// use glm_color::*;
    ///
    /// let red = Hsv::from_rgb(RED);
    /// let (green, blue) = red.triad();
    ///
    /// assert!(is_close_to(&green.hue(), &radians(120.), 0.000001));
    /// assert!(is_close_to(&blue.hue(), &radians(240.), 0.000001));
    /// # }
    /// ```
    #[inline]
    pub fn triad(&self) -> (Hsv, Hsv) {
        let pi2 = tau();
        let d120 = radians(120.);
        let h1 = fmod(self.hue() + d120, pi2);
        let h2 = fmod(self.hue() + d120 + d120, pi2);
        (self.with_hue(h1), self.with_hue(h2))
    }

    /// Returns `n` colors that are analogous to the receiver.
    ///
    // TODO: Example
    #[inline]
    pub fn analogs(&self, n: usize, span: f32) -> Vec<Hsv> {
        if span == 0. || n == 0 {
            vec!()
        } else {
            let d = span / (n as f32);
            let h = self.hue();
            (0..n).map(|i| -> Hsv {
                let hue = fmod(h + d * (i as f32), tau());
                Hsv { h: hue, s: self.s, v: self.v }
            }).collect()
        }
    }

    /// Returns `n` colors distributed on the color wheel evenly.
    ///
    /// The returned colors have full saturation and lightness. Red is the
    /// first color in the vector.
    ///
    /// If `n` is `0`, an empty vector is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use glm_color::*;
    ///
    /// let cs0 = Hsv::color_wheel(0);
    /// assert_eq!(cs0.len(), 0);
    /// let cs1 = Hsv::color_wheel(1);
    /// assert_eq!(cs1[0], Hsv::from_hue(0.));
    /// let csn = Hsv::color_wheel(3);
    /// assert_eq!(csn.len(), 3);
    /// ```
    #[inline]
    pub fn color_wheel(n: usize) -> Vec<Hsv> {
        let red = Hsv::from_hue(0.);
        red.analogs(n, tau())
    }

    /// Produces a color by adding white to the receiver.
    ///
    /// For `Hsv` color model, adding white means increasing the lightness.
    ///
    /// Parameter `amt` specifies absolute lightness to be added.
    ///
    /// # Example
    ///
    /// ```rust
    /// use glm_color::*;
    ///
    /// let red = Hsv::new(1., 0.5, 0.2);
    /// assert_eq!(red.tint(1.), Hsv::new(1., 0.5, 1.));
    /// ```
    #[inline]
    pub fn tint(&self, amt: f32) -> Hsv {
        let b = clamp(amt, 0., 1.);
        self.with_brightness(self.brightness() + b)
    }

    /// Produces a vector of `n` colors whose brightness increase monotonically
    /// and evenly.
    ///
    /// If `n` is `0`, returns an empty vector.
    ///
    /// Other wise, the receiver is the first element of the vector and a
    /// color with full brightness is the last.
    ///
    /// # Example
    ///
    /// ```rust
    /// use glm_color::*;
    ///
    /// let red: Hsv = from_rgb(RED);
    /// let clrs = red.tints(3);
    ///
    /// ```
    #[inline]
    pub fn tints(&self, n: usize) -> Vec<Hsv> {
        if n == 0 {
            vec!()
        } else {
            let b = self.brightness();
            let d = (1. - b) / (n as f32);
            (0..n).map(|i| {
                self.with_brightness(b + d * (i as f32))
            }).collect()
        }
    }

    /// Produces a color by adding black to the receiver.
    #[inline]
    pub fn shade(&self, amt: f32) -> Hsv {
        let b = clamp(amt, 0., 1.);
        self.with_brightness(self.brightness() - b)
    }

    /// Produces`n` colors whose brightness decrease monotonically and evenly.
    #[inline]
    pub fn shades(&self, n: usize) -> Vec<Hsv> {
        if n == 0 {
            vec!()
        } else {
            let b = self.brightness();
            let d = self.brightness() / (n as f32);
            (0..n).map(|i| {
                self.with_brightness(b - d * (i as f32))
            }).collect()
        }
    }

    /// Produces a color by adding gray to the receiver.
    #[inline]
    pub fn tone(&self, amt: f32) -> Hsv {
        let s = clamp(amt, 0., 1.);
        self.with_saturation(self.saturation() - s)
    }

    /// Produces `n` colors whose saturation decrease monotonically and evenly.
    #[inline]
    pub fn tones(&self, n: usize) -> Vec<Hsv> {
        if n == 0 {
            vec!()
        } else {
            let s = self.saturation();
            let d = (1. - s) / (n as f32);
            (0..n).map(|i| {
                self.with_brightness(s + d * (i as f32))
            }).collect()
        }
    }
}

/// Equivalent to call `Hsv::new(h, s, v)`.
#[inline]
pub fn hsv(h: f32, s: f32, v: f32) -> Hsv {
    Hsv::new(h, s, v)
}

impl Eq for Hsv {}

impl ApproxEq for Hsv {
    type BaseType = f32;
    #[inline]
    fn is_close_to(&self, other: &Hsv, max_diff: f32) -> bool {
        self.as_vec3().is_close_to(other.as_vec3(), max_diff)
    }
}

impl ColorSpace for Hsv {
    #[inline]
    fn from_rgb(rgb: Rgb) -> Hsv {
        Hsv { h: rgb.hue(), s: rgb.saturation(), v: rgb.brightness() }
    }
    #[inline]
    fn to_rgb(&self) -> Rgb {
        let Hsv { h, s, v } = *self;
        if is_approx_eq(&s, &0.) {
            Rgb::new(v, v, v)
        } else {
            let hv = degrees(h) / 60.;
            let hi = floor(hv) % 6.;
            let f = hv - hi;
            let p = v * (1. - s);
            let q = v * (1. - f * s);
            let t = v * (1. - (1. - f) * s);
            match hi {
                0. => Rgb::new(v, t, p),
                1. => Rgb::new(q, v, p),
                2. => Rgb::new(p, v, t),
                3. => Rgb::new(p, q, v),
                4. => Rgb::new(t, p, v),
                5. => Rgb::new(v, p, q),
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod test {

    use glm::*;
    use space::ColorSpace;
    use rgb::{ Rgb, gray };
    use super::Hsv;
    use quickcheck::*;

    #[test]
    fn test_to_rgb() {
        fn prop(clr: Rgb) -> bool {
            let hsv: Hsv = ColorSpace::from_rgb(clr);
            hsv.to_rgb().is_close_to(&clr, 0.000001)
        }
        quickcheck(prop as fn(Rgb) -> bool)
    }

    #[test]
    fn test_to_rgb_gray() {
        fn prop(v: f32) -> bool {
            let gray = Rgb::new(v, v, v);
            let hsv: Hsv = ColorSpace::from_rgb(gray);
            hsv.saturation() == 0. &&
            hsv.to_rgb().is_close_to(&gray, 0.000001)
        }
        quickcheck(prop as fn(f32) -> bool)
    }
}
