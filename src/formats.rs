use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum PixelFormat {
    /// Stores 8-bit intensity values (each pixel is composed of just one value) along with an
    /// alpha channel. This makes the image look grayscale.
    IntensityA8,
    /// Stores 16-bit color values, but does not save an alpha channel.
    Rgb565,
    /// Stores 16-bit color values, but saves the alpha channel as well.
    Rgb5a3,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum DataFormat {
    /// Stores 4-bit intensity values (each pixel is composed of just one value). This makes the
    /// image look grayscale. This format stores no alpha channel.
    Intensity4,
    /// Stores 8-bit intensity values (each pixel is composed of just one value). This makes the
    /// image look grayscale. This format stores no alpha channel.
    Intensity8,
    /// Stores 4-bit intensity values (each pixel is composed of just one value) along with an
    /// alpha channel. This makes the image look grayscale.
    IntensityA4,
    /// Stores 8-bit intensity values (each pixel is composed of just one value) along with an
    /// alpha channel. This makes the image look grayscale.
    IntensityA8,
    /// Stores 16-bit color values, but does not save an alpha channel.
    Rgb565,
    /// Stores 16-bit color values, but saves the alpha channel as well.
    Rgb5a3,
    /// Stores 24-bit depth true color (1 byte per color). It also stores an 8-bit alpha channel.
    ///
    /// This format is by far the one with the largest filesize, although the most accurate in terms of
    /// color.
    Argb8888,
    /// Stores 4-bit indices into a quantized color palette.
    ///
    /// The color palette can only encode a maximum of 16 colors, which means images with a larger
    /// variety of colors will not look that great.
    Index4,
    /// Stores 8-bit indices into a quantized color palette.
    ///
    /// The color palette can encode a maximum of 256 colors, which means images preserve a decent
    /// amount of their color quality, as opposed to the Index4 format.
    Index8,
    /// Encodes the image using a DXT1 compression algorithm, also known as BC1 (Block Compression 1).
    ///
    /// Works well in environments where the texture cannot be easily viewed (like a 3D model in
    /// motion), but not that well in other cases (like on a 2D menu), as the compression artifacts
    /// can be quite visible at times.
    Dxt1,
}

impl std::fmt::Display for PixelFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IntensityA8 => write!(f, "Intensity 8-bit With Alpha"),
            Self::Rgb565 => write!(f, "RGB565"),
            Self::Rgb5a3 => write!(f, "RGB5A3"),
        }
    }
}

impl From<PixelFormat> for gvrtex::formats::PixelFormat {
    fn from(value: PixelFormat) -> Self {
        match value {
            PixelFormat::IntensityA8 => Self::IntensityA8,
            PixelFormat::Rgb565 => Self::RGB565,
            PixelFormat::Rgb5a3 => Self::RGB5A3,
        }
    }
}

impl std::fmt::Display for DataFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataFormat::Intensity4 => write!(f, "Intensity 4-bit"),
            DataFormat::Intensity8 => write!(f, "Intensity 8-bit"),
            DataFormat::IntensityA4 => write!(f, "Intensity 4-bit With Alpha"),
            DataFormat::IntensityA8 => write!(f, "Intensity 8-bit With Alpha"),
            DataFormat::Rgb565 => write!(f, "RGB565"),
            DataFormat::Rgb5a3 => write!(f, "RGB5A3"),
            DataFormat::Argb8888 => write!(f, "ARGB8888"),
            DataFormat::Index4 => write!(f, "4-bit Indexed"),
            DataFormat::Index8 => write!(f, "8-bit Indexed"),
            DataFormat::Dxt1 => write!(f, "DXT1 Compressed"),
        }
    }
}

impl From<DataFormat> for gvrtex::formats::DataFormat {
    fn from(value: DataFormat) -> Self {
        match value {
            DataFormat::Intensity4 => Self::Intensity4,
            DataFormat::Intensity8 => Self::Intensity8,
            DataFormat::IntensityA4 => Self::IntensityA4,
            DataFormat::IntensityA8 => Self::IntensityA8,
            DataFormat::Rgb565 => Self::Rgb565,
            DataFormat::Rgb5a3 => Self::Rgb5a3,
            DataFormat::Argb8888 => Self::Argb8888,
            DataFormat::Index4 => Self::Index4,
            DataFormat::Index8 => Self::Index8,
            DataFormat::Dxt1 => Self::Dxt1,
        }
    }
}
