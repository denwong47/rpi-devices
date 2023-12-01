//! Traits for structs that have the ability to iterate over its pixels.
//!

use std::simd::{prelude::*, LaneCount, SupportedLaneCount};

use crate::foreign_types::*;

pub trait IterPixels<COLOUR>
where
    COLOUR: RgbColor,
{
    /// Returns an iterator over the pixels of an image.
    fn iter_pixels(&self) -> impl Iterator<Item = Pixel<COLOUR>>;

    /// Returns an iterator over the pixels of an image in chunks.
    fn iter_pixels_chunked(
        &self,
        chunk_size: usize,
    ) -> IntoChunks<impl Iterator<Item = Pixel<COLOUR>>> {
        self.iter_pixels().chunks(chunk_size)
    }

    /// Returns an iterator over the pixels of an image in SIMD chunks.
    #[cfg(feature = "simd")]
    fn iter_pixels_simd<const N: usize>(&self) -> Vec<[Simd<u8, N>; 3]>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        self.iter_pixels_chunked(N)
            .into_iter()
            .map(|chunk| {
                let values: (Vec<_>, Vec<_>, Vec<_>) = chunk
                    .into_iter()
                    .map(|pixel| (pixel.1.r(), pixel.1.g(), pixel.1.b()))
                    .multiunzip();

                let idxs = Simd::from_slice(&(0..N).collect_vec());

                [
                    Simd::gather_or_default(&values.0, idxs),
                    Simd::gather_or_default(&values.1, idxs),
                    Simd::gather_or_default(&values.2, idxs),
                ]
            })
            .collect()
    }
}

impl<'a, COLOUR, BO> IterPixels<COLOUR> for ImageRaw<'a, COLOUR, BO>
where
    BO: pixelcolor::raw::ByteOrder,
    COLOUR: RgbColor + From<<COLOUR as PixelColor>::Raw>,
    embedded_graphics::iterator::raw::RawDataSlice<'a, COLOUR::Raw, BO>:
        IntoIterator<Item = COLOUR::Raw>,
{
    /// Returns an iterator over the pixels of an image.
    ///
    /// # Note
    ///
    /// Very bad in performance. Try not doing this on a [`ImageRaw`].
    fn iter_pixels(&self) -> impl Iterator<Item = Pixel<COLOUR>> {
        let size = self.size();

        (0..size.height).flat_map(move |y| {
            (0..size.width).map(move |x| {
                let point = Point::new(x as i32, y as i32);
                let colour = self.pixel(point.clone()).unwrap();
                Pixel(point, colour)
            })
        })
    }
}

#[cfg(feature = "bmp")]
impl<COLOUR> IterPixels<COLOUR> for tinybmp::Bmp<'_, COLOUR>
where
    COLOUR:
        RgbColor + From<pixelcolor::Rgb555> + From<pixelcolor::Rgb565> + From<pixelcolor::Rgb888>,
{
    /// Returns an iterator over the pixels of an image.
    fn iter_pixels(&self) -> impl Iterator<Item = Pixel<COLOUR>> {
        self.pixels()
    }
}
