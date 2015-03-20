//
// GLM-COLOR.
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

use super::rgb::Rgb;

/// `ColorSpace` is the representation and interpretation of color values.
///
/// The only requirement for a implementation is that it can be converted to
/// and from linear RGB color space.
pub trait ColorSpace {

    /// Constructs from color value `rgb` in RGB color space.
    fn from_rgb(rgb: Rgb) -> Self;

    /// Converts _self_ to a color value in RGB color space.
    fn to_rgb(&self) -> Rgb;
}

/// Converts `clr` in linear RGB space to color space `T`.
#[inline]
pub fn from_rgb<T: ColorSpace>(clr: Rgb) -> T {
    <T as ColorSpace>::from_rgb(clr)
}

/// Converts `clr` in color space `T` to linear RGB color space.
#[inline]
pub fn to_rgb<T: ColorSpace>(clr: &T) -> Rgb {
    clr.to_rgb()
}
