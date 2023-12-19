//! An attempt to create an image that owns its own data.
//!
use std::{marker::PhantomData, path::Path, sync::OnceLock};

use embedded_graphics::transform::Transform;

use crate::{foreign_types::*, func, SubImage};

/// A struct for storing image data. Each instance will own its own data, but cannot be
/// changed once created.
///
/// Typically, to draw an image to the display, you would need to keep ownership of the
/// underlying bytes, then the raw image that points to said bytes, then the image that
/// points to the raw image. This struct will own all of these, so that all the related
/// data can be portably passed around.
///
/// To get around lifetime checks, this struct requires the instance to be created
/// first, before any raw image and image data is added. Raw image and image are only
/// created once;
#[derive(Clone)]
pub struct OwnedImage<'i, IR, COLOUR>
where
    IR: ImageDrawable<Color = COLOUR>,
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw> + 'i,
    Self: OwnsImage<'i, IR>,
{
    pub bytes: OnceLock<Box<[u8]>>,
    pub raw: OnceLock<IR>,
    pub image: OnceLock<Image<'i, IR>>,
    pub size: OnceLock<Size>,

    _phantom: PhantomData<&'i ()>,
}

impl<'i, IR, COLOUR> Default for OwnedImage<'i, IR, COLOUR>
where
    IR: ImageDrawable<Color = COLOUR>,
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw> + 'i,
    Self: OwnsImage<'i, IR>,
{
    /// Create an empty new instance.
    fn default() -> Self {
        Self {
            bytes: OnceLock::new(),
            raw: OnceLock::new(),
            image: OnceLock::new(),
            size: OnceLock::new(),

            _phantom: PhantomData,
        }
    }
}

impl<'i, IR, COLOUR> OwnedImage<'i, IR, COLOUR>
where
    IR: ImageDrawable<Color = COLOUR>,
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw> + 'i,
    Self: OwnsImage<'i, IR>,
{
    /// Create an empty new instance.
    ///
    /// This obviously does not contain any image data, and would not be capable of
    /// drawing anything, but this is needed to satisfy the lifetime checks.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an empty new instance with a given size.
    ///
    /// This obviously does not contain any image data, and would not be capable of
    /// drawing anything, but this is needed to satisfy the lifetime checks.
    ///
    /// Only use this if the size cannot be determined by the image data, such as
    /// an [`ImageRaw`].
    pub fn with_size(size: Size) -> Self {
        let instance = Self::new();
        instance.put_size(size).unwrap();

        instance
    }

    /// Create an empty new instance with a given width and height.
    ///
    /// This obviously does not contain any image data, and would not be capable of
    /// drawing anything, but this is needed to satisfy the lifetime checks.
    ///
    /// Only use this if the size cannot be determined by the image data, such as
    /// an [`ImageRaw`].
    pub fn with_width_height(width: u32, height: u32) -> Self {
        Self::with_size(Size::new(width, height))
    }

    /// Create a new instance from the given bytes.
    pub fn from_bytes<'e>(bytes: impl Into<Box<[u8]>>) -> RPiResult<'e, Self> {
        let instance = Self::new();
        instance.put_bytes(bytes).and(Ok(instance))
    }

    /// Create a new instance from the given bytes and size.
    pub fn from_bytes_size<'e>(bytes: impl Into<Box<[u8]>>, size: Size) -> RPiResult<'e, Self> {
        let instance = Self::with_size(size);
        instance.put_bytes(bytes).and(Ok(instance))
    }

    /// Create a new instance by reading the file at the given path.
    pub async fn from_path<'e>(path: impl AsRef<Path> + std::fmt::Debug) -> RPiResult<'e, Self> {
        let instance = Self::new();
        instance.put_file(path).await?;

        Ok(instance)
    }

    /// Create a new instance by reading the file at the given path, and with the
    /// given size.
    pub async fn from_path_size<'e>(
        path: impl AsRef<Path> + std::fmt::Debug,
        size: Size,
    ) -> RPiResult<'e, Self> {
        let instance = Self::with_size(size);
        instance.put_file(path).await?;

        Ok(instance)
    }

    /// Add the size to the instance.
    ///
    /// Only use this if the size cannot be determined by the image data, such as
    /// an [`ImageRaw`].
    pub fn put_size<'e>(&self, size: Size) -> RPiResult<'e, ()> {
        match self.size.set(size) {
            Ok(_) => Ok(()),
            Err(_) => Err(RPiError::AlreadyInitialised("OwnedImage".into())),
        }
    }

    /// Add the image data to the instance.
    pub fn put_bytes<'e>(&self, value: impl Into<Box<[u8]>>) -> RPiResult<'e, ()> {
        match self.bytes.set(value.into()) {
            Ok(_) => Ok(()),
            Err(_) => Err(RPiError::AlreadyInitialised("OwnedImage".into())),
        }
    }

    /// Get bytes from the instance.
    pub fn bytes<'e>(&self) -> RPiResult<'e, &[u8]> {
        self.bytes
            .get()
            .map(|bytes| bytes.as_ref())
            .ok_or(RPiError::NotInitialised("OwnedImage".into()))
    }

    /// Create a new image from the given bytes.
    pub async fn put_file<'e>(
        &self,
        path: impl AsRef<Path> + std::fmt::Debug,
    ) -> RPiResult<'e, ()> {
        let bytes = func::fs::read_bytes_from_file(&path).await?;
        self.put_bytes(bytes)
    }

    /// Put the raw image into the instance.
    pub fn put_raw<'e>(&self, value: IR) -> RPiResult<'e, ()> {
        match self.raw.set(value) {
            Ok(_) => Ok(()),
            Err(_) => Err(RPiError::AlreadyInitialised("OwnedImage".into())),
        }
    }

    /// Get the raw image from the instance.
    pub fn raw<'e>(&'i self) -> RPiResult<'e, &IR> {
        match self.raw.get() {
            Some(raw) => Ok(raw),
            None => {
                self.init_from_bytes().and_then(
                    // Try again.
                    |_| self.raw(),
                )
            }
        }
    }

    /// Put the image into the instance.
    pub fn put_image<'e>(&self, value: Image<'i, IR>) -> RPiResult<'e, ()> {
        match self.image.set(value) {
            Ok(_) => Ok(()),
            Err(_) => Err(RPiError::AlreadyInitialised("OwnedImage".into())),
        }
    }

    /// Create a new image from the given instance.
    pub fn init_image_at<'e>(&'i self, position: Point) -> RPiResult<'e, ()> {
        let raw = self.raw()?;
        self.put_image(Image::new(raw, position))
    }

    /// Get the image from the instance. If it does not exist, create it.
    pub fn image<'e>(&'i self) -> RPiResult<'e, &Image<'i, IR>> {
        match self.image.get() {
            Some(image) => Ok(image),
            None => {
                self.init_image_at(Point::new(0, 0)).and_then(
                    // Try again.
                    |_| self.image(),
                )
            }
        }
    }

    /// Get the image from the instance at a given position.
    ///
    /// This will create the base image at origin first, then translate it to the
    /// given position.
    ///
    /// Note that this returns an image that is owned by the caller, albeit bound
    /// to the lifetime of the instance.
    pub fn image_at<'e>(&'i self, position: Point) -> RPiResult<'e, Image<'i, IR>> {
        let base_image = self.image()?;

        Ok(base_image.translate(position))
    }

    /// Create a [`SubImage`] from the instance.
    pub fn subimage<'e>(&'i self, position: Point, size: Size) -> RPiResult<'e, SubImage<'i, IR>> {
        let raw = self.raw()?;

        Ok(func::crop::crop_raw(
            raw,
            position.x,
            position.y,
            size.width,
            size.height,
        ))
    }
}

pub trait OwnsImage<'i, IR>
where
    IR: ImageDrawable,
    Self: Sized + 'i,
{
    /// Create a new instance from the given bytes.
    fn init_from_bytes<'e>(&'i self) -> RPiResult<'e, ()>;
}

/// An [`ImageRaw`] that owns its own data.
pub type OwnedImageRaw<'i, COLOUR, BO> = OwnedImage<'i, ImageRaw<'i, COLOUR, BO>, COLOUR>;

impl<'i, COLOUR, BO> OwnsImage<'i, ImageRaw<'i, COLOUR, BO>> for OwnedImageRaw<'i, COLOUR, BO>
where
    COLOUR: RgbColor + From<<COLOUR as PixelColor>::Raw> + 'i,
    BO: pixelcolor::raw::ByteOrder + 'i,
    embedded_graphics::iterator::raw::RawDataSlice<'i, COLOUR::Raw, BO>:
        IntoIterator<Item = COLOUR::Raw>,
{
    /// Initialise the instance with the given bytes as a raw image.
    ///
    /// A raw image requires a width to be specified, along with the colour space
    /// and byte ordering of the incoming bytes.
    fn init_from_bytes<'e>(&'i self) -> RPiResult<'e, ()> {
        let bytes = self.bytes()?;
        let width = match self.size.get() {
            Some(size) => size.width,
            None => {
                return Err(RPiError::InvalidInput(
                    "OwnedImage".into(),
                    "`width` must be specified when creating a raw image. \
                Try creating an instance with `with_size()` instead."
                        .into(),
                ))
            }
        };

        let raw = ImageRaw::new(bytes, width);
        self.put_raw(raw)
    }
}

#[cfg(feature = "bmp")]
/// An [`Bmp`] that owns its own data.
pub type OwnedBmp<'i, COLOUR> = OwnedImage<'i, Bmp<'i, COLOUR>, COLOUR>;

#[cfg(feature = "bmp")]
impl<'i, COLOUR> OwnsImage<'i, Bmp<'i, COLOUR>> for OwnedBmp<'i, COLOUR>
where
    COLOUR: PixelColor
        + From<pixelcolor::Rgb555>
        + From<pixelcolor::Rgb565>
        + From<pixelcolor::Rgb888>
        + From<<COLOUR as PixelColor>::Raw>
        + 'i,
{
    /// Initialise the instance with the given bytes as a BMP image.
    ///
    /// A BMP image contains its own size, so this requires the size of the image
    /// to NOT be set.
    fn init_from_bytes<'e>(&'i self) -> RPiResult<'e, ()>
    where
        Self: Sized,
    {
        let bytes = self.bytes()?;

        let raw = func::bmp::bmp_from_bytes(bytes)?;

        self.put_raw(raw).and_then(|_| self.put_size(raw.size()))
    }
}
