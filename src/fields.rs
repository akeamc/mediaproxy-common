use serde::{Deserialize, Serialize};

/// Output format of the processed image.
#[derive(Serialize, Deserialize)]
pub enum OutputFormat {
    /// Output the media as JPEG.
    #[serde(rename = "jpeg")]
    #[serde(alias = "jpg")]
    Jpeg,

    /// Output the media as a PNG file.
    #[serde(rename = "png")]
    Png,

    /// Encode the media with WebP.
    #[serde(rename = "webp")]
    WebP,

    /// Encode the media as a GIF.
    #[serde(rename = "gif")]
    Gif,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Jpeg
    }
}

/// Options for how to crop the media if the new resolution has a different aspect ratio.
#[derive(Serialize, Deserialize)]
pub enum ResizeStrategy {
    /// The image is scaled to the maximum possible size that fits within the new resolution.
    #[serde(rename = "contain")]
    Contain,

    /// Crop the image to match the aspect ratio and resolution. Doesn't stretch.
    #[serde(rename = "crop")]
    Crop,

    /// Stretches the media to the exact resolution.
    #[serde(rename = "stretch")]
    Stretch,
}

impl Default for ResizeStrategy {
  fn default() -> Self {
    ResizeStrategy::Crop
  }
}
