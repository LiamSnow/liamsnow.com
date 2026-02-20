use brotli::{BrotliCompress, enc::BrotliEncoderParams};
use bytes::Bytes;
use http::HeaderValue;

pub struct Route {
    pub content_br: Option<Bytes>,
    pub content_identity: Bytes,
    pub content_type: HeaderValue,
    pub cache_control: Option<HeaderValue>,
}

impl Route {
    pub fn from_bytes(
        content: Vec<u8>,
        mime: impl ToString,
        cache_control: Option<HeaderValue>,
    ) -> Self {
        Self {
            content_br: Some(compress_brotli(&content)),
            content_identity: Bytes::from(content),
            content_type: HeaderValue::from_str(&mime.to_string()).unwrap(),
            cache_control,
        }
    }

    pub fn from_bytes_precompressed(
        content: Vec<u8>,
        mime: impl ToString,
        cache_control: Option<HeaderValue>,
    ) -> Self {
        Self {
            content_br: None,
            content_identity: Bytes::from(content),
            content_type: HeaderValue::from_str(&mime.to_string()).unwrap(),
            cache_control,
        }
    }

    pub fn from_string(
        content: String,
        mime: impl ToString,
        cache_control: Option<HeaderValue>,
    ) -> Self {
        let bytes = content.into_bytes();
        Self {
            content_br: Some(compress_brotli(&bytes)),
            content_identity: Bytes::from(bytes),
            content_type: HeaderValue::from_str(&mime.to_string()).unwrap(),
            cache_control,
        }
    }
}

fn compress_brotli(input: &[u8]) -> Bytes {
    let mut output = Vec::new();
    let params = BrotliEncoderParams {
        quality: 11,
        ..Default::default()
    };
    BrotliCompress(&mut &input[..], &mut output, &params).unwrap();
    Bytes::from(output)
}
