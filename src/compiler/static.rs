use crate::{compiler::route::Route, indexer::FileSlot};
use anyhow::Result;
use http::HeaderValue;

pub fn compile(slot: &FileSlot) -> Result<Route> {
    let mime = mime_guess::from_ext(&slot.ext).first_or_text_plain();

    let cache_control = (mime.type_() == "font")
        .then(|| HeaderValue::from_static("public, max-age=31536000, immutable"));

    Ok(Route::from_bytes_precompressed(
        slot.file.to_vec(),
        mime,
        cache_control,
    ))
}
