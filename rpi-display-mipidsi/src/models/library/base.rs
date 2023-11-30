//!
//!
use std::{hash::Hash, sync::Arc};

use crate::foreign_types::*;
use gxhash::GxHashMap;

/// Prune a [`GxHashMap`] of any entries not in the keep [`GxHashMap`] and with a
/// reference count less than or equal to the given minimum.
///
/// # Returns
///
/// The number of entries removed.
fn prune_map<H, T, U>(
    map: &mut GxHashMap<H, Arc<T>>,
    keep: &GxHashMap<H, Arc<U>>,
    min_ref_count: usize,
) -> usize
where
    H: Clone + Hash + PartialEq + Eq,
{
    let orphans: Vec<_> = map
        .iter()
        .filter_map(|(key, value)| {
            if (!keep.contains_key(key)) || Arc::strong_count(value) <= min_ref_count {
                Some(key.clone())
            } else {
                None
            }
        })
        .collect();

    let orphan_count = orphans.len();
    for orphan in orphans {
        map.remove(&orphan);
    }

    orphan_count
}

#[derive(Debug)]
pub struct ImageLibrary<'a, H, COLOUR>
where
    H: Clone + Hash + PartialEq + Eq,
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    for<'i> ImageRaw<'i, COLOUR>: ImageDrawable,
{
    images: GxHashMap<H, Arc<Image<'a, ImageRaw<'a, COLOUR>>>>,
    raws: GxHashMap<H, Arc<ImageRaw<'a, COLOUR>>>,
    bytes: GxHashMap<H, Arc<Vec<u8>>>,
}

impl<'a, H, COLOUR> Default for ImageLibrary<'a, H, COLOUR>
where
    H: Clone + Hash + PartialEq + Eq,
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    for<'i> ImageRaw<'i, COLOUR>: ImageDrawable,
{
    /// Create a new, empty [`ImageLibrary`].
    fn default() -> Self {
        Self {
            images: GxHashMap::default(),
            raws: GxHashMap::default(),
            bytes: GxHashMap::default(),
        }
    }
}

impl<'a, H, COLOUR> ImageLibrary<'a, H, COLOUR>
where
    H: Clone + Hash + PartialEq + Eq,
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    for<'i> ImageRaw<'i, COLOUR>: ImageDrawable,
{
    /// Create a new, empty [`ImageLibrary`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an [`Image`] via its bytes to the [`ImageLibrary`].
    ///
    /// Returns an [`Arc`] reference to the [`Image`].
    ///
    /// # Note
    ///
    /// This function will silently replace the [`Image`] if it already exists in the
    /// [`ImageLibrary`].
    ///
    /// TODO This currently hashes the same value 6 times in order to guarantee lifetimes.
    /// Is there a better way?
    pub fn insert_image_by_bytes(
        &'a mut self,
        hash: H,
        bytes: &'a [u8],
        width: u32,
        x: i32,
        y: i32,
    ) -> Arc<Image<'a, ImageRaw<'a, COLOUR>>> {
        let bytes = Arc::new(bytes.to_vec());
        self.bytes.insert(hash.clone(), Arc::clone(&bytes));

        let bytes = self.bytes.get(&hash).unwrap();

        let raw: Arc<ImageRaw<'a, COLOUR>> = Arc::new(ImageRaw::new(&bytes, width));
        self.raws.insert(hash.clone(), Arc::clone(&raw));

        let raw = self.raws.get(&hash).unwrap();

        let image = Arc::new(Image::new(raw.as_ref(), Point::new(x, y)));
        self.images.insert(hash.clone(), Arc::clone(&image));

        Arc::clone(self.images.get(&hash).unwrap())
    }

    /// Prune the [`ImageLibrary`] of any dangling data not required by images.
    ///
    /// This should not strictly be necessary, but it is a good idea to have this
    /// function available to avoid memory leaks.
    pub fn prune(&mut self) -> usize {
        const MIN_REF_COUNT: usize = 1;

        prune_map(&mut self.raws, &self.images, MIN_REF_COUNT)
            + prune_map(&mut self.bytes, &self.raws, MIN_REF_COUNT)
    }
}
