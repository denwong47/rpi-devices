//! Dissolve transition between two images.
//!

use super::TransitionFunction;
use crate::{foreign_types::*, func, traits::IterPixels, SubImage};
use embedded_graphics::pixelcolor::{raw::ToBytes, Rgb888};
use rand::{thread_rng, Rng};
use std::simd::{prelude::*, LaneCount, SupportedLaneCount};

/// Transition between two images by dissolving the first image into the second.
///
/// The `random_size` parameter controls the size of the random array used to determine
/// the order in which pixels are drawn. The larger the array, the more random the
/// transition will be.
pub fn dissolve<'a, COLOUR, T1, T2>(
    random_size: usize,
    steps: u32,
) -> impl TransitionFunction<'a, COLOUR, T1, T2, ImageRaw<'a, COLOUR>>
where
    COLOUR: RgbColor + From<<COLOUR as PixelColor>::Raw> + From<Rgb888>,
    pixelcolor::raw::RawU24: From<COLOUR>,
    T1: ImageDrawable<Color = COLOUR> + IterPixels<COLOUR> + 'a,
    T2: ImageDrawable<Color = COLOUR> + IterPixels<COLOUR> + 'a,
{
    let mut rng = thread_rng();

    let rand_sample = (0..random_size)
        .map(|_| rng.gen_range(0..steps))
        .collect::<Vec<u32>>();

    move |image1: &T1, image2: &T2, step: u32, steps: u32, w: u16, h: u16| {
        if image1.size() != image2.size() {
            return Err(RPiError::InvalidInput(
                "images to be dissolved".into(),
                "must be the same size".into(),
            ));
        }

        image1
            .iter_pixels_simd::<64>()
            .into_iter()
            .zip(image2.iter_pixels_simd::<64>().into_iter())
            .enumerate()
            .map(|(chunk_id, ([r1, g1, b1], [r2, g2, b2]))| {
                let pos_step = (0..64)
                    .into_iter()
                    .map(|i| {
                        rand_sample
                            .get(i + chunk_id * 64 % random_size)
                            .unwrap_or(&0)
                    })
                    .map(|i| if *i >= step { 0_u8 } else { 1_u8 })
                    .collect::<Vec<_>>();

                let mask = Simd::<u8, 64>::from_slice(&pos_step);
                let neg_mask = Simd::<u8, 64>::splat(1) - mask;

                let r = r1 * mask + r2 * neg_mask;
                let g = g1 * mask + g2 * neg_mask;
                let b = b1 * mask + b2 * neg_mask;

                r.to_array()
                    .into_iter()
                    .zip(g.to_array().into_iter())
                    .zip(b.to_array().into_iter())
                    .map(|((r, g), b)| {
                        pixelcolor::raw::RawU24::from(COLOUR::from(Rgb888::new(r, g, b)))
                            .to_be_bytes()
                    })
            })
            .flatten()
            .take((w * h * 3) as usize)
            .fold(
                Vec::with_capacity((w * h * 3) as usize),
                |mut bytes, array| {
                    bytes.extend(array);

                    bytes
                },
            );

        // This is hopeless, we'll never be able to export this `ImageRaw` under ownership
        // rules
        todo!();

        Ok(())
    };

    todo!()
}
