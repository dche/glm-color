
#![feature(test)]

extern crate test;
extern crate rand;
extern crate glm;
extern crate "glm_color" as color;

use color::*;
use test::Bencher;
use rand::{ IsaacRng, Rng };

macro_rules! bench_to_rgb(
    ($name: ident, $t: ty) => {
        #[bench]
        fn $name(bh: &mut Bencher) {
            const LEN: usize = 1 << 13;

            let mut rng = IsaacRng::new_unseeded();

            let mut elems: Vec<$t> = (0..LEN).map(|_| rng.gen::<$t>()).collect();
            let mut i = 0;

            bh.iter(|| {
                i = (i + 1) & (LEN - 1);

                unsafe {
                    let e = elems.get_unchecked_mut(i);
                    test::black_box(e.to_rgb());
                }
            })
        }
    }
);

macro_rules! bench_from_rgb(
    ($name: ident, $t: ty, $ret: ident) => {
        #[bench]
        fn $name(bh: &mut Bencher) {
            const LEN: usize = 1 << 13;

            let mut rng = IsaacRng::new_unseeded();

            let mut elems: Vec<$t> = (0..LEN).map(|_| rng.gen::<$t>()).collect();
            let mut i = 0;

            bh.iter(|| {
                i = (i + 1) & (LEN - 1);

                unsafe {
                    let e = elems.get_unchecked_mut(i);
                    test::black_box(<$ret as ColorSpace>::from_rgb(*e));
                }
            })
        }
    }
);

bench_to_rgb!(_hsv2rgb, Hsv);
bench_from_rgb!(_rgb2hsv, Rgb, Hsv);

bench_to_rgb!(_srgb2rgb, Srgb);
bench_from_rgb!(_rgb2srgb, Rgb, Srgb);

bench_to_rgb!(_ycbcr2rgb, YCbCr);
bench_from_rgb!(_rgb2ycbcr, Rgb, YCbCr);
