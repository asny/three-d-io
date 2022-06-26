use crate::{io::Deserialize, Error, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

///
/// Contains raw assets which are usually generated by either the [load](crate::io::load)/[load_async](crate::io::load_async) functions or the [Serialize::serialize](crate::io::Serialize::serialize) function.
/// Can also be constructed manually using the [RawAssets::new] and [RawAssets::insert].
///
/// Use the [RawAssets::remove] or [RawAssets::get] function to extract the raw byte array for the assets
/// or [RawAssets::deserialize] to deserialize an asset or [RawAssets::save] to save the assets.
///
#[derive(Default)]
pub struct RawAssets(HashMap<PathBuf, Vec<u8>>);

impl RawAssets {
    ///
    /// Constructs a new empty set of raw assets.
    ///
    pub fn new() -> Self {
        Self::default()
    }

    ///
    /// Remove and returns the raw byte array for the resource at the given path.
    ///
    /// ```
    /// # use three_d_asset::io::*;
    /// let mut assets = load(&["test_data/test.png"]).unwrap();
    /// let png_bytes = assets.remove("test.png").unwrap();
    /// ```
    ///
    /// **Note:** If no asset has been inserted with the exact same path given as input, then the first asset which path that contains the path given as input is returned.
    /// This means the above example can be reduced to:
    /// ```
    /// # use three_d_asset::io::*;
    /// let png_bytes = load(&["test_data/test.png"]).unwrap().remove("").unwrap();
    /// ```
    ///
    pub fn remove(&mut self, path: impl AsRef<Path>) -> Result<Vec<u8>> {
        Ok(self.0.remove(&self.match_path(path)?).unwrap())
    }

    ///
    /// Returns a reference to the raw byte array for the resource at the given path.
    ///
    /// ```
    /// # use three_d_asset::io::*;
    /// let mut assets = load(&["test_data/test.png"]).unwrap();
    /// let png_bytes = assets.get("test.png").unwrap();
    /// ```
    ///
    /// **Note:** If no asset has been inserted with the exact same path given as input, then the first asset which path that contains the path given as input is returned.
    /// This means the above example can be reduced to:
    /// ```
    /// # use three_d_asset::io::*;
    /// let png_bytes = load(&["test_data/test.png"]).unwrap().get("").unwrap();
    /// ```
    ///
    pub fn get(&self, path: impl AsRef<Path>) -> Result<&[u8]> {
        Ok(self.0.get(&self.match_path(path)?).unwrap())
    }

    pub(crate) fn match_path(&self, path: impl AsRef<Path>) -> Result<PathBuf> {
        let path: PathBuf = path.as_ref().to_str().unwrap().replace("\\", "/").into();
        if self.0.contains_key(&path) {
            Ok(path)
        } else {
            let p = path.to_str().unwrap();
            let p2 = if p.ends_with(".jpeg") {
                p[0..p.len() - 2].to_string()
            } else if p.ends_with(".jpg") {
                p[0..p.len() - 1].to_string()
            } else {
                p.to_owned()
            };
            self.0
                .iter()
                .find(|(k, _)| k.to_str().unwrap().contains(&p2))
                .map(|(k, _)| k.clone())
                .ok_or(Error::NotLoaded(p.to_owned()))
        }
    }

    ///
    /// Inserts the given bytes into the set of raw assets.
    /// This is useful if you want to add data from an unsuported source and want to use either the [RawAssets::deserialize] functionality or [RawAssets::save] functionality.
    ///
    /// ```
    /// # use three_d_asset::io::*;
    /// # use three_d_asset::Texture2D;
    /// # let png_bytes = include_bytes!("../../test_data/test.png").to_vec();
    /// let mut assets = RawAssets::new();
    /// assets.insert("test.png", png_bytes);
    /// let texture: Texture2D = assets.deserialize("test.png").unwrap();
    /// ```
    ///
    pub fn insert(&mut self, path: impl AsRef<Path>, bytes: Vec<u8>) -> &mut Self {
        let key = path.as_ref().to_str().unwrap().replace("\\", "/");
        self.0.insert(key.into(), bytes);
        self
    }

    ///
    /// Inserts all of the given raw assets into this set of raw assets.
    ///
    pub fn extend(&mut self, mut raw_assets: Self) -> &mut Self {
        for (k, v) in raw_assets.0.drain() {
            self.insert(k, v);
        }
        self
    }

    ///
    /// Deserialize the asset with the given path into a type that implements the [Deserialize] trait.
    ///
    /// ```
    /// # use three_d_asset::io::*;
    /// # use three_d_asset::Texture2D;
    /// let mut assets = load(&["test_data/test.png"]).unwrap();
    /// let texture: Texture2D = assets.deserialize("test.png").unwrap();
    /// ```
    ///
    /// **Note:** If no asset has been inserted with the exact same path given as input, then the first asset which path that contains the path given as input is deserialized.
    /// This means the above example can be reduced to:
    /// ```
    /// # use three_d_asset::io::*;
    /// # use three_d_asset::Texture2D;
    /// let texture: Texture2D = load(&["test_data/test.png"]).unwrap().deserialize("").unwrap();
    /// ```
    pub fn deserialize<T: Deserialize>(&mut self, path: impl AsRef<Path>) -> Result<T> {
        T::deserialize(path, self)
    }

    ///
    /// Saves all of the raw assets to files.
    ///
    #[cfg_attr(docsrs, doc(not(target_arch = "wasm32")))]
    #[cfg(not(target_arch = "wasm32"))]
    pub fn save(&mut self) -> Result<()> {
        crate::io::save(self)
    }

    ///
    /// An iterator visiting all pairs of [PathBuf] (indicating the source or destination path for the raw asset) and `Vec<u8>` which are the raw serialized bytes.
    ///
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, PathBuf, Vec<u8>> {
        self.0.iter()
    }
}

impl std::fmt::Debug for RawAssets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("RawAssets");
        for (key, value) in self.0.iter() {
            d.field("path", key);
            d.field("byte length", &value.len());
        }
        d.finish()
    }
}
