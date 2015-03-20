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

#![allow(unused_variables)]
#![feature(core)]

//! A simple crate for manipulating and generating color values. It is an
//! extension to the [`glm`](https://crates.io/crates/glm) crate.
//!
//! `glm_color` treats color values as numbers, instead of somthing that can be
//! used in rendering directly. So things like data format, order of channels,
//! and even alpha channel, are not handled by this library.
//!
//! The only interesting part of this crate is functions in `Rgb` and `Hsv`
//! color spaces that produce colors procedurally. The design of these functions
//! are based on Wikipedia page [Color Theory](http://en.wikipedia.org/wiki/Color_theory)
//! and [this blog](http://devmag.org.za/2012/07/29/how-to-choose-colours-procedurally-algorithms/).
//!
//! # Example
//!
//! ## Generating colors
//!
//! ```rust
//! # extern crate glm;
//! # extern crate glm_color;
//! # fn main() {
//! use glm::*;
//! use glm_color::*;
//!
//! // constant color values.
//! let mut red = RED;
//!
//! // with constructors.
//! red = Rgb::new(1., 0., 0.);
//! red = rgb(255, 0, 0);
//!
//! // from other color spaces.
//! red = hsv(radians(360.), 1., 1.).to_rgb();
//! red = ycbcr(1., 0., 0.5).to_rgb();
//!
//! // randomly.
//! let rnd = Rgb::rand();
//! let rnd_hsv = Hsv::rand();
//!
//! // procedurally.
//! let blues = from_rgb::<Hsv>(BLUE).analogs(5, radians(30.));
//! let yellows: Vec<Hsv> = blues.iter().map(|clr| -> Hsv {
//!     clr.complement()
//! }).collect();
//! let darker_red = from_rgb::<Hsv>(RED).shade(0.3);
//! # }
//! ```
//! ## Manipulating colors
//!
//! ```rust
//! # extern crate glm;
//! # extern crate glm_color;
//! # fn main() {
//! use glm_color::*;
//!
//! // Linear RGB color space supports some arithmetics.
//! let yellow = RED + GREEN;
//! let white = yellow + BLUE;
//! assert_eq!(white, WHITE);
//! # }
//! ```
//! ## Converting colors
//!
//! ```rust
//! # extern crate glm;
//! # extern crate glm_color;
//! # fn main() {
//! use glm::*;
//! use glm_color::*;
//!
//! let rgb = RED;
//! // All color spaces can be converted to and from the linear RGB color space.
//! let hsv = Hsv::from_rgb(rgb);
//! let mut red = hsv.to_rgb();
//! assert!(is_close_to(&rgb, &red, 0.000001));
//! let ybr: YCbCr = from_rgb(rgb);
//! red = to_rgb(&ybr);
//! let srgb = Srgb::from_rgb(rgb);
//! # }
//! ```
// TODO: examples for packing/unpacking colors.

extern crate rand;
extern crate glm;
#[cfg(test)]
extern crate quickcheck;

pub use space::{ ColorSpace, from_rgb, to_rgb };

pub use rgb::{ Rgb, rgb, gray, grey };

pub use rgb::consts::*;

pub use hsv::{ Hsv, hsv };

pub use ycbcr::{ YCbCr, ycbcr };

pub use srgb::{ Srgb, srgb };

mod space;
pub mod rgb;
pub mod hsv;
pub mod ycbcr;
pub mod srgb;
// TODO: pub mod Lab;
