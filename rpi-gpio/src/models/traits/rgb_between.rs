//! A trait for RGB colors that can be interpolated between.

use super::from_tuple_rgb::FromTupleRGB;
use embedded_graphics::pixelcolor::RgbColor;

/// A trait for RGB colors that can be interpolated between.
pub trait RgbBetween<RGB>
where
    RGB: RgbColor,
{
    fn rgb_between(&self, other: &RGB, factor: f32) -> RGB;
}

/// Global implementation of [`RgbBetween`] for all [`RgbColor`]s.
impl<RGB> RgbBetween<RGB> for RGB
where
    RGB: RgbColor + FromTupleRGB,
{
    /// Interpolate between two colors.
    fn rgb_between(&self, other: &RGB, factor: f32) -> RGB {
        let factor = factor.max(0.0).min(1.0);

        // Use square root of sum of squares to get a more perceptually linear
        // interpolation.
        let factorise = |a: u8, b: u8| {
            ((a as f32).powi(2) * (1. - factor) + (b as f32).powi(2) * factor)
                .sqrt()
                .round() as u8
        };

        // TODO Possible SIMD optimisation here?
        RGB::from_rgb(
            factorise(self.r(), other.r()),
            factorise(self.g(), other.g()),
            factorise(self.b(), other.b()),
        )
    }
}
