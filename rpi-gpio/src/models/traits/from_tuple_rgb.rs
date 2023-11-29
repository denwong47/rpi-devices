//! Trait to convert from a tuple of RGB values to a foreign RGB struct.
//!

/// Trait to convert from a tuple of RGB values to a foreign RGB struct.
///
/// This complements the [`embedded_graphics::pixelcolor::RgbColor`] trait, which does
/// not include a builder method.
pub trait FromTupleRGB
where
    Self: Sized,
{
    /// Convert from a tuple of RGB values to a foreign RGB struct.
    fn from_rgb(r: u8, g: u8, b: u8) -> Self;

    /// Convert from a slice of RGB values to a foreign RGB struct.
    ///
    /// # Panics
    ///
    /// Panics if the slice does not have more than three elements.
    fn from_slice_rgb(rgb: &[u8]) -> Self {
        Self::from_rgb(rgb[0], rgb[1], rgb[2])
    }

    /// Convert from a tuple of RGB values to a foreign RGB struct.
    fn from_tuple_rgb(rgb: (u8, u8, u8)) -> Self {
        Self::from_rgb(rgb.0, rgb.1, rgb.2)
    }
}

macro_rules! expand_rgb_structs {
    ($($name:ident),*) => {
        use embedded_graphics::pixelcolor::{
            $($name),*
        };

        $(
            impl FromTupleRGB for $name {
                /// Convert from a tuple of RGB values to a foreign RGB struct.
                fn from_rgb(r: u8, g: u8, b: u8) -> Self {
                    Self::new(r, g, b)
                }

            }
        )*
    };
}

expand_rgb_structs!(Rgb555, Rgb565, Rgb666, Rgb888, Bgr555, Bgr565, Bgr666, Bgr888);
