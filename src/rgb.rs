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
use std::ops::{ Add, Sub, Mul };
use std::mem;
use rand::{ Rand, Rng, thread_rng };
#[cfg(test)]
use quickcheck::*;

/// The linear RGB color space.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Rgb {
    r: f32,
    g: f32,
    b: f32,
}

impl ColorSpace for Rgb {
    #[inline(always)]
    fn from_rgb(rgb: Rgb) -> Rgb {
        rgb
    }
    #[inline(always)]
    fn to_rgb(&self) -> Rgb {
        *self
    }
}

const ONE_OVER_256: f32 = 0.0039215686274509803921568627451_f32;

impl Rgb {

    /// Constructs a `Rgb` color from given `red`, `green` and `blue` values.
    ///
    /// Parameter values are clamped into the interval _[0, 1]_.
    ///
    /// # Example
    ///
    /// ```
    /// use glm_color::*;
    ///
    /// let blue = Rgb::new(-10., 0., 1000.);
    /// assert_eq!(blue, BLUE);
    /// ```
    #[inline]
    pub fn new(red: f32, green: f32, blue: f32) -> Rgb {
        let v = clamp_s(vec3(red, green, blue), 0., 1.);
        Rgb { r: v.x, g: v.y, b: v.z }
    }

    /// Returns the value of red channel of _self_.
    ///
    /// # Example
    ///
    /// ```
    /// use glm_color::*;
    ///
    /// assert_eq!(RED.red(), 1.);
    /// ```
    #[inline(always)]
    pub fn red(&self) -> f32 {
        self.r
    }

    /// Returns the value of green channel of _self_.
    #[inline(always)]
    pub fn green(&self) -> f32 {
        self.g
    }

    /// Returns the value of blue channel of _self_.
    #[inline(always)]
    pub fn blue(&self) -> f32 {
        self.b
    }

    /// Returns a new `Rgb` value that has given red valud `r` and same
    /// green and blue values as _self_.
    ///
    /// # Example
    ///
    /// ```
    /// use glm_color::*;
    ///
    /// let clr = RED.with_red(0.75);
    /// assert_eq!(clr.red(), 0.75);
    /// assert_eq!(clr.green(), 0.);
    /// assert_eq!(clr.blue(), 0.)
    /// ```
    #[inline]
    pub fn with_red(&self, r: f32) -> Rgb {
        Rgb { r: clamp(r, 0., 1.), g: self.g, b: self.b }
    }

    /// Changes the red value of _self_ to `r`.
    ///
    /// # Example
    ///
    /// ```
    /// use glm_color::*;
    ///
    /// let mut clr = BLUE;
    /// clr.set_red(100.);
    /// assert_eq!(clr.red(), 1.);
    /// ```
    pub fn set_red(&mut self, r: f32) {
        self.r = clamp(r, 0., 1.)
    }

    /// Returns a new `Rgb` value that has given green valud `g` and same
    /// red and blue values as _self_.
    #[inline]
    pub fn with_green(&self, g: f32) -> Rgb {
        Rgb { r: self.r, g: clamp(g, 0., 1.), b: self.b }
    }

    /// Changes the green value of _self_ to `g`.
    #[inline]
    pub fn set_green(&mut self, g: f32) {
        self.g = clamp(g, 0., 1.)
    }

    /// Returns a new `Rgb` value that has given blue valud `b` and same
    /// red and green values as _self_.
    #[inline]
    pub fn with_blue(&self, b: f32) -> Rgb {
        Rgb { r: self.r, g: self.g, b: clamp(b, 0., 1.) }
    }

    /// Changes the blue value of _self_ to `b`.
    #[inline]
    pub fn set_blue(&mut self, b: f32) {
        self.b = clamp(b, 0., 1.)
    }

    /// Creates a `Rgb` value by specifying red, green and blue values in
    /// `u8` type.
    ///
    /// # Example
    /// ```
    /// use glm_color::*;
    ///
    /// let red = Rgb::from_u8(255, 0, 0);
    /// assert_eq!(red.red(), 1f32);
    /// ```
    #[inline]
    pub fn from_u8(r: u8, g: u8, b: u8) -> Rgb {
        let cv = |c: u8| -> f32 {
            match c {
                255 => 1f32,
                0   => 0f32,
                _   => (c as f32) * ONE_OVER_256,
            }
        };
        Rgb { r: cv(r), g: cv(g), b: cv(b) }
    }

    /// Constructs a `Rgb` value from a 32-bit unsigned integer `clr`.
    ///
    /// The lower 24 bits of `clr` are intepreted as 3 8-bit values,
    /// from low to high, for B, G and R respectively.
    ///
    /// # Example
    ///
    /// ```
    /// use glm_color::{ Rgb, CYAN };
    /// let cyan = Rgb::from_u32(0x00FFFF);
    /// assert_eq!(cyan, CYAN);
    /// assert_eq!(cyan.red(), 0.);
    /// ```
    #[inline]
    pub fn from_u32(clr: u32) -> Rgb {
        let cv = |c: u32| -> u8 {
            ((clr >> (8 * c)) & 0xFF) as u8
        };
        Rgb::from_u8(cv(2), cv(1), cv(0))
    }

    /// Returns the hue of _self_. It is a value in the interval [0, 2π).
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
    /// assert_eq!(RED.hue(), 0.);
    /// assert_eq!(GREEN.hue(), radians(120.));
    /// assert_eq!(BLUE.hue(), radians(240.));
    /// # }
    /// ```
    #[inline]
    pub fn hue(&self) -> f32 {
        let &Rgb { r, g, b } = self;
        let max = max(r, max(g, b));
        let min = min(r, min(g, b));

        if min.is_approx_eq(&max) {
            // gray color, hue is irrevalent.
            0.
        } else {
            let c = max - min;
            let deg =
                match max {
                    _ if max == r => ((g - b) / c) % 6.,
                    _ if max == g => ((b - r) / c) + 2.,
                    _ => ((r - g) / c) + 4.,
                };
            radians((deg * 60. + 360.) % 360.)
        }
    }

    /// Returns the saturation of _self_. It is a value in the interval _[0,
    /// 1]_.
    ///
    /// # Example
    ///
    /// ``` rust
    /// use glm_color::*;
    ///
    /// // gray colors have 0 saturation.
    /// assert_eq!(GRAY.saturation(), 0.);
    /// // pure color has full saturation.
    /// assert_eq!(RED.saturation(), 1.);
    /// ```
    #[inline]
    pub fn saturation(&self) -> f32 {
        let &Rgb { r, g, b } = self;
        let max = max(r, max(g, b));
        let min = min(r, min(g, b));

        if max.is_approx_eq(&min) {
            // gray.
            0.
        } else {
            1. - min / max
        }
    }

    /// Returns the lightness of _self_.
    ///
    /// # Note
    ///
    /// The returned value is maximum of red, green, and blue channels.
    #[inline(always)]
    pub fn brightness(&self) -> f32 {
        self.as_vec3().max()
    }

    /// Returns _self_'s lunimance.
    ///
    /// Luminance is calculated with the same parameters as `YCbCr` color space's
    /// `from_rgb()` function.
    ///
    /// # Example
    ///
    /// ```
    /// use glm_color::*;
    ///
    /// assert_eq!(WHITE.lunimance(), 1.);
    /// assert_eq!(BLACK.lunimance(), 0.);
    /// ```
    #[inline]
    pub fn lunimance(&self) -> f32 {
        dot(*self.as_vec3(), vec3(0.2126, 0.7152, 0.0722))
    }

    /// Re-interprets the reference of a `Rgb` to a reference of `Vec3`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use glm_color as color;
    ///
    /// let red = color::RED;
    /// assert_eq!(red.as_vec3().x, red.red());
    /// ```
    #[inline]
    pub fn as_vec3(&self) -> &Vec3 {
        let vec: &Vec3 = unsafe { mem::transmute(self) };
        vec
    }

    /// Constructs an `Rgb` value by randomly choosing values for each of the
    /// three RGB channels using the thread local RNG.
    ///
    /// # Note
    ///
    /// This function is used to generates a single seed color. Use other
    /// member methods to generates colors that have certain relationships with
    /// the seed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use glm_color::*;
    ///
    /// let seed = Rgb::rand();
    /// // generating 10 colors based on `seed`.
    /// let clrs: Vec<Rgb> = (0..10).map(|_| -> Rgb {
    ///     seed.rand_offset(0.3)
    /// }).collect();
    /// ```
    pub fn rand() -> Rgb {
        let mut rng = thread_rng();
        rng.gen()
    }

    /// Constructs an `Rgb` value by adding a random `offset` to all RGB channels.
    ///
    /// `offset` is clamped to the interval _[0, 1]_.
    #[inline]
    pub fn rand_offset(&self, offset: f32) -> Rgb {
        let v3 = self.as_vec3();
        let val = v3.sum() / 3.;
        let mut rng = thread_rng();
        let rnd = rng.gen::<f32>();
        let os = clamp(offset, 0., 1.);
        if is_approx_eq(&val, &0.) {
            Rgb { r: os, g: os, b: os }
        } else {
            let ratio = 1. + (2. * rnd * os - os) / val;
            let v = *v3 * ratio;
            Rgb::new(v.x, v.y, v.z)
        }
    }

    // TODO: more color generation algothrithm. esp. the harmonic one.
}

// values of all components are in the range [0, 1].
impl Eq for Rgb {}

impl Rand for Rgb {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> Rgb {
        // generates values for each channel independently.
        let r: f32 = rng.gen();
        let g: f32 = rng.gen();
        let b: f32 = rng.gen();
        debug_assert!(vec3(r, g, b).sum() <= 3.);
        Rgb { r: r, g: b, b: b }
    }
}

impl ApproxEq for Rgb {
    type BaseType = f32;
    #[inline]
    fn is_close_to(&self, other: &Rgb, max_diff: f32) -> bool {
        self.as_vec3().is_close_to(other.as_vec3(), max_diff)
    }
}

impl Add<Rgb> for Rgb {
    type Output = Rgb;
    #[inline]
    fn add(self, rhs: Rgb) -> Rgb {
        Rgb::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl Sub<Rgb> for Rgb {
    type Output = Rgb;
    #[inline]
    fn sub(self, rhs: Rgb) -> Rgb {
        Rgb::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl Mul<f32> for Rgb {
    type Output = Rgb;
    #[inline]
    fn mul(self, rhs: f32) -> Rgb {
        let r = abs(rhs);
        Rgb::new(self.r * r, self.g * r, self.b * r)
    }
}

impl Mul<Rgb> for Rgb {
    type Output = Rgb;
    #[inline]
    fn mul(self, rhs: Rgb) -> Rgb {
        Rgb::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

/// Constructs a `Rgb` value with given RGB values `r`, `g` and `b`.
///
/// Equivalent to `Rgb::from_u8(r, g, b)`.
#[inline(always)]
pub fn rgb(r: u8, g: u8, b: u8) -> Rgb {
    Rgb::from_u8(r, g, b)
}

/// Constructs a `Rgb` value with given gray value `x`. All RGB
/// channels are set to this value.
///
/// Equivalent to `Rgb::rgb(x, x, x)`.
#[inline(always)]
pub fn gray(x: u8) -> Rgb {
    rgb(x, x, x)
}

/// Alias of functions `gray`.
#[inline(always)]
pub fn grey(x: u8) -> Rgb {
    gray(x)
}

/// Color constants, derived from [SVG's color keywords](http://www.w3.org/TR/SVGColor12/#syntax).
pub mod consts {

    use super::{ Rgb, ONE_OVER_256 };

    macro_rules! rgb {
        ($r: expr, $g: expr, $b: expr) => {
            Rgb {
                r: ($r as f32) * ONE_OVER_256,
                g: ($g as f32) * ONE_OVER_256,
                b: ($b as f32) * ONE_OVER_256
            }
        }
    }

    pub const ALICE_BLUE            : Rgb = rgb!(240, 248, 255);
    pub const ANTIQUE_WHITE         : Rgb = rgb!(250, 235, 215);
    pub const AQUA                  : Rgb = rgb!(0, 255, 255);
    pub const AQUA_MARINE           : Rgb = rgb!(127, 255, 212);
    pub const AZURE                 : Rgb = rgb!(240, 255, 255);
    pub const BEIGE                 : Rgb = rgb!(245, 245, 220);
    pub const BISQUE                : Rgb = rgb!(255, 228, 196);
    pub const BLACK                 : Rgb = rgb!(0, 0, 0);
    pub const BLANCHED_ALMOND       : Rgb = rgb!(255, 235, 205);
    pub const BLUE                  : Rgb = rgb!(0, 0, 255);
    pub const BLUE_VIOLET           : Rgb = rgb!(138, 43, 226);
    pub const BROWN                 : Rgb = rgb!(165, 42, 42);
    pub const BURLY_WOOD            : Rgb = rgb!(222, 184, 135);
    pub const CADET_BLUE            : Rgb = rgb!(95, 158, 160);
    pub const CHARTREUSE            : Rgb = rgb!(127, 255, 0);
    pub const CHOCOLATE             : Rgb = rgb!(210, 105, 30);
    pub const CORAL                 : Rgb = rgb!(255, 127, 80);
    pub const CORNFLOWER_BLUE       : Rgb = rgb!(100, 149, 237);
    pub const CORNSILK              : Rgb = rgb!(255, 248, 220);
    pub const CRIMSON               : Rgb = rgb!(220, 20, 60);
    pub const CYAN                  : Rgb = rgb!(0, 255, 255);
    pub const DARK_BLUE             : Rgb = rgb!(0, 0, 139);
    pub const DARK_CYAN             : Rgb = rgb!(0, 139, 139);
    pub const DARK_GOLDEN_ROD       : Rgb = rgb!(184, 134, 11);
    pub const DARK_GRAY             : Rgb = rgb!(169, 169, 169);
    pub const DARK_GREEN            : Rgb = rgb!(0, 100, 0);
    pub const DARK_GREY             : Rgb = DARK_GRAY;
    pub const DARK_KHAKI            : Rgb = rgb!(189, 183, 107);
    pub const DARK_MAGENTA          : Rgb = rgb!(139, 0, 139);
    pub const DARK_OLIVEGREEN       : Rgb = rgb!(85, 107, 47);
    pub const DARK_ORANGE           : Rgb = rgb!(255, 140, 0);
    pub const DARK_ORCHID           : Rgb = rgb!(153, 50, 204);
    pub const DARK_RED              : Rgb = rgb!(139, 0, 0);
    pub const DARK_SALMON           : Rgb = rgb!(233, 150, 122);
    pub const DARK_SEA_GREEN        : Rgb = rgb!(143, 188, 143);
    pub const DARK_SLATE_BLUE       : Rgb = rgb!(72, 61, 139);
    pub const DARK_SLATE_GRAY       : Rgb = rgb!(47, 79, 79);
    pub const DARK_SLATE_GREY       : Rgb = DARK_SLATE_GRAY;
    pub const DARK_TURQUOISE        : Rgb = rgb!(0, 206, 209);
    pub const DARK_VIOLET           : Rgb = rgb!(148, 0, 211);
    pub const DEEP_PINK             : Rgb = rgb!(255, 20, 147);
    pub const DEEP_SKY_BLUE         : Rgb = rgb!(0, 191, 255);
    pub const DIM_GRAY              : Rgb = rgb!(105, 105, 105);
    pub const DIM_GREY              : Rgb = DIM_GRAY;
    pub const DODGER_BLUE           : Rgb = rgb!(30, 144, 255);
    pub const FIRE_BRICK            : Rgb = rgb!(178, 34, 34);
    pub const FLORAL_WHITE          : Rgb = rgb!(255, 250, 240);
    pub const FOREST_GREEN          : Rgb = rgb!(34, 139, 34);
    pub const FUCHSIA               : Rgb = rgb!(255, 0, 255);
    pub const GAINSBORO             : Rgb = rgb!(220, 220, 220);
    pub const GHOST_WHITE           : Rgb = rgb!(248, 248, 255);
    pub const GOLD                  : Rgb = rgb!(255, 215, 0);
    pub const GOLDEN_ROD            : Rgb = rgb!(218, 165, 32);
    pub const GRAY                  : Rgb = rgb!(128, 128, 128);
    pub const GREY                  : Rgb = GRAY;
    pub const GREEN                 : Rgb = rgb!(0, 255, 0);
    pub const GREEN_YELLOW          : Rgb = rgb!(173, 255, 47);
    pub const HONEYDEW              : Rgb = rgb!(240, 255, 240);
    pub const HOT_PINK              : Rgb = rgb!(255, 105, 180);
    pub const INDIAN_RED            : Rgb = rgb!(205, 92, 92);
    pub const INDIGO                : Rgb = rgb!(75, 0, 130);
    pub const IVORY                 : Rgb = rgb!(255, 255, 240);
    pub const KHAKI                 : Rgb = rgb!(240, 230, 140);
    pub const LAVENDER              : Rgb = rgb!(230, 230, 250);
    pub const LAVENDER_BLUSH        : Rgb = rgb!(255, 240, 245);
    pub const LAWN_GREEN            : Rgb = rgb!(124, 252, 0);
    pub const LEMON_CHIFFON         : Rgb = rgb!(255, 250, 205);
    pub const LIGHT_BLUE            : Rgb = rgb!(173, 216, 230);
    pub const LIGHT_CORAL           : Rgb = rgb!(240, 128, 128);
    pub const LIGHT_CYAN            : Rgb = rgb!(224, 255, 255);
    pub const LIGHT_GOLDEN_ROD_YELLOW: Rgb = rgb!(250, 250, 210);
    pub const LIGHT_GRAY            : Rgb = rgb!(211, 211, 211);
    pub const LIGHT_GREEN           : Rgb = rgb!(144, 238, 144);
    pub const LIGHT_GREY            : Rgb = LIGHT_GRAY;
    pub const LIGHT_PINK            : Rgb = rgb!(255, 182, 193);
    pub const LIGHT_SALMON          : Rgb = rgb!(255, 160, 122);
    pub const LIGHT_SEA_GREEN       : Rgb = rgb!(32, 178, 170);
    pub const LIGHT_SKY_BLUE        : Rgb = rgb!(135, 206, 250);
    pub const LIGHT_SLATE_GRAY      : Rgb = rgb!(119, 136, 153);
    pub const LIGHT_SLATE_GREY      : Rgb = LIGHT_SLATE_GRAY;
    pub const LIGHT_STEEL_BLUE      : Rgb = rgb!(176, 196, 222);
    pub const LIGHT_YELLOW          : Rgb = rgb!(255, 255, 224);
    pub const LIME                  : Rgb = rgb!(0, 255, 0);
    pub const LIME_GREEN            : Rgb = rgb!(50, 205, 50);
    pub const LINEN                 : Rgb = rgb!(250, 240, 230);
    pub const MAGENTA               : Rgb = rgb!(255, 0, 255);
    pub const MAROON                : Rgb = rgb!(128, 0, 0);
    pub const MEDIUM_AQUA_MARINE    : Rgb = rgb!(102, 205, 170);
    pub const MEDIUM_BLUE           : Rgb = rgb!(0, 0, 205);
    pub const MEDIUM_ORCHID         : Rgb = rgb!(186, 85, 211);
    pub const MEDIUM_PURPLE         : Rgb = rgb!(147, 112, 219);
    pub const MEDIUM_SEA_GREEN      : Rgb = rgb!(60, 179, 113);
    pub const MEDIUM_SLATE_BLUE     : Rgb = rgb!(123, 104, 238);
    pub const MEDIUM_SPRING_GREEN   : Rgb = rgb!(0, 250, 154);
    pub const MEDIUM_TURQUOISE      : Rgb = rgb!(72, 209, 204);
    pub const MEDIUM_VIOLET_RED     : Rgb = rgb!(199, 21, 133);
    pub const MIDNIGHT_BLUE         : Rgb = rgb!(25, 25, 112);
    pub const MINT_CREAM            : Rgb = rgb!(245, 255, 250);
    pub const MISTY_ROSE            : Rgb = rgb!(255, 228, 225);
    pub const MOCCASIN              : Rgb = rgb!(255, 228, 181);
    pub const NAVAJO_WHITE          : Rgb = rgb!(255, 222, 173);
    pub const NAVY                  : Rgb = rgb!(0, 0, 128);
    pub const OLD_LACE              : Rgb = rgb!(253, 245, 230);
    pub const OLIVE                 : Rgb = rgb!(128, 128, 0);
    pub const OLIVE_DRAB            : Rgb = rgb!(107, 142, 35);
    pub const ORANGE                : Rgb = rgb!(255, 165, 0);
    pub const ORANGE_RED            : Rgb = rgb!(255, 69, 0);
    pub const ORCHID                : Rgb = rgb!(218, 112, 214);
    pub const PALE_GOLDEN_ROD       : Rgb = rgb!(238, 232, 170);
    pub const PALE_GREEN            : Rgb = rgb!(152, 251, 152);
    pub const PALE_TURQUOISE        : Rgb = rgb!(175, 238, 238);
    pub const PALE_VIOLET_RED       : Rgb = rgb!(219, 112, 147);
    pub const PAPAYA_WHIP           : Rgb = rgb!(255, 239, 213);
    pub const PEACH_PUFF            : Rgb = rgb!(255, 218, 185);
    pub const PERU                  : Rgb = rgb!(205, 133, 63);
    pub const PINK                  : Rgb = rgb!(255, 192, 203);
    pub const PLUM                  : Rgb = rgb!(221, 160, 221);
    pub const POWDER_BLUE           : Rgb = rgb!(176, 224, 230);
    pub const PURPLE                : Rgb = rgb!(128, 0, 128);
    pub const RED                   : Rgb = rgb!(255, 0, 0);
    pub const ROSY_BROWN            : Rgb = rgb!(188, 143, 143);
    pub const ROYAL_BLUE            : Rgb = rgb!(65, 105, 225);
    pub const SADDLE_BROWN          : Rgb = rgb!(139, 69, 19);
    pub const SALMON                : Rgb = rgb!(250, 128, 114);
    pub const SANDY_BROWN           : Rgb = rgb!(244, 164, 96);
    pub const SEA_GREEN             : Rgb = rgb!(46, 139, 87);
    pub const SEA_SHELL             : Rgb = rgb!(255, 245, 238);
    pub const SIENNA                : Rgb = rgb!(160, 82, 45);
    pub const SILVER                : Rgb = rgb!(192, 192, 192);
    pub const SKY_BLUE              : Rgb = rgb!(135, 206, 235);
    pub const SLATE_BLUE            : Rgb = rgb!(106, 90, 205);
    pub const SLATE_GRAY            : Rgb = rgb!(112, 128, 144);
    pub const SLATE_GREY            : Rgb = SLATE_GRAY;
    pub const SNOW                  : Rgb = rgb!(255, 250, 250);
    pub const SPRING_GREEN          : Rgb = rgb!(0, 255, 127);
    pub const STEEL_BLUE            : Rgb = rgb!(70, 130, 180);
    pub const TAN                   : Rgb = rgb!(210, 180, 140);
    pub const TEAL                  : Rgb = rgb!(0, 128, 128);
    pub const THISTLE               : Rgb = rgb!(216, 191, 216);
    pub const TOMATO                : Rgb = rgb!(255, 99, 71);
    pub const TURQUOISE             : Rgb = rgb!(64, 224, 208);
    pub const VIOLET                : Rgb = rgb!(238, 130, 238);
    pub const WHEAT                 : Rgb = rgb!(245, 222, 179);
    pub const WHITE                 : Rgb = rgb!(255, 255, 255);
    pub const WHITE_SMOKE           : Rgb = rgb!(245, 245, 245);
    pub const YELLOW                : Rgb = rgb!(255, 255, 0);
    pub const YELLOW_GREEN          : Rgb = rgb!(154, 205, 50);
}

#[cfg(test)]
impl Arbitrary for Rgb {
    fn arbitrary<G: Gen>(g: &mut G) -> Rgb {
        g.gen()
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use super::consts::*;
    use quickcheck::*;

    #[test]
    fn test_add() {
        assert_eq!(RED + GREEN, YELLOW);
        assert_eq!(GREEN + BLUE, CYAN);
        assert_eq!(RED + BLUE, MAGENTA);
        assert_eq!(RED + GREEN + BLUE, WHITE);
    }

    #[test]
    fn test_add_clamp() {
        assert_eq!(RED + RED, RED);
    }

    #[test]
    fn test_mul() {
        fn prop(clr: Rgb) -> bool {
            clr * WHITE == clr &&
            clr * BLACK == BLACK
        }
        quickcheck(prop as fn(Rgb) -> bool);
    }
}
