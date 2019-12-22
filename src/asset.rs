use std::borrow::Cow;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets"]
struct AssetImpl;

/// The RustEmbed wrapper for completion.
pub struct Asset;

impl Asset {
    pub fn get(file_path: &str) -> Option<Cow<'static, [u8]>> {
        AssetImpl::get(file_path)
    }

    pub fn iter() -> impl Iterator<Item = Cow<'static, str>> {
        AssetImpl::iter()
    }
}
