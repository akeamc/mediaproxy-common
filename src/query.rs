use base64::encode_config;
use custom_error::custom_error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ImageProcessingOutput {
    #[serde(rename = "jpeg")]
    #[serde(alias = "jpg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "webp")]
    WebP,
    #[serde(rename = "gif")]
    Gif,
}

/// The default URL-safe base64 configuration.
fn b64_config() -> base64::Config {
    base64::Config::new(base64::CharacterSet::UrlSafe, false)
}

custom_error! {pub QueryFingerprintConversionError
  JsonError{source: serde_json::Error} = "Something went wrong when (de)serializing JSON.",
  Base64Error{source: base64::DecodeError} = "Something went wrong when decoding Base64.",
  UnicodeError{source: std::str::Utf8Error} = "Could not convert byte array to string!"
}

/// The default object that clients use to make requests to Media Proxy.
#[derive(Serialize, Deserialize)]
pub struct Query {
    /// URL of the source image. Input formats currently supported are the same as those of the [image] crate.
    pub source: String,

    /// The width of the processed image.
    pub width: Option<u32>,

    /// The height of the processed image.
    pub height: Option<u32>,

    /// Output format of the image.
    pub format: ImageProcessingOutput,
}

impl Query {
    /// Convert a `Query` to a fingerprint.
    ///
    /// In reality, this just serializes the `Query` to JSON and encodes it in base64.
    pub fn to_fingerprint(self: &Self) -> String {
        let json = serde_json::to_string(&self).unwrap();
        encode_config(json, b64_config())
    }

    /// Create a `Query` from a base64 encoded fingerprint. Fingerprints can be created with `Query.to_fingerprint()`.
    ///
    /// # Arguments
    ///
    /// * `fingerprint` - A base64 encoded `Query` in JSON.
    ///
    /// # Example
    ///
    /// ```
    /// use mediaproxy_common::query::Query;
    /// let query = Query::from_fingerprint("eyJzb3VyY2UiOiJodHRwczovL2R1bW15aW1hZ2UuY29tLzYwMHg0MDAvMDAwL2ZmZiIsIndpZHRoIjpudWxsLCJoZWlnaHQiOm51bGwsImZvcm1hdCI6ImpwZWcifQ".to_string());
    /// ```
    pub fn from_fingerprint(fingerprint: String) -> Result<Query, QueryFingerprintConversionError> {
        let bytes = base64::decode_config(fingerprint, b64_config())?;
        let json = std::str::from_utf8(&bytes)?;
        let query: Query = serde_json::from_str(json)?;
        Ok(query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_to_fingerprint() {
        let query = Query {
            source: String::from("https://dummyimage.com/600x400/000/fff"),
            format: ImageProcessingOutput::Jpeg,
            width: None,
            height: None,
        };

        assert_eq!(query.to_fingerprint(), String::from("eyJzb3VyY2UiOiJodHRwczovL2R1bW15aW1hZ2UuY29tLzYwMHg0MDAvMDAwL2ZmZiIsIndpZHRoIjpudWxsLCJoZWlnaHQiOm51bGwsImZvcm1hdCI6ImpwZWcifQ"));
    }

    #[test]
    fn fingerprint_to_query() {
        let fingerprint = String::from("eyJzb3VyY2UiOiJodHRwczovL2R1bW15aW1hZ2UuY29tLzYwMHg0MDAvMDAwL2ZmZiIsIndpZHRoIjpudWxsLCJoZWlnaHQiOm51bGwsImZvcm1hdCI6ImpwZWcifQ");
        let query = Query::from_fingerprint(fingerprint).unwrap();
        assert_eq!(query.source, "https://dummyimage.com/600x400/000/fff");
    }

    #[test]
    fn invalid_fingerprint() {
        let fingerprint = String::from("bruh"); // Perfectly fine base 64, not so fine JSON.
        let query = Query::from_fingerprint(fingerprint);
        assert_eq!(query.is_err(), true);
    }
}
