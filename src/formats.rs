#![cfg_attr(feature = "strict", deny(missing_docs))]
#![cfg_attr(feature = "strict", deny(warnings))]

#[derive(Debug, Copy, Clone)]
pub enum PixelFormat {
    I420,
    Yv12,
    Yuy2,
    Uyvy,
    Ayuv,
    Vuya,
    Rgbx,
    Bgrx,
    Xrgb,
    Xbgr,
    Rgba,
    Bgra,
    Argb,
    Abgr,
    Rgb,
    Bgr,
    Y41b,
    Y42b,
    Yvyu,
    Y444,
    V210,
    V216,
    Y210,
    Y410,
    Nv12,
    Nv21,
    Gray8,
    Gray16Be,
    Gray16Le,
    V308,
    Rgb16,
    Bgr16,
    Rgb15,
    Bgr15,
    Uyvp,
    A420,
    Rgb8P,
    Yuv9,
    Yvu9,
    Iyu1,
    Argb64,
    Ayuv64,
    R210,
    I420_10Be,
    I420_10Le,
    I422_10Be,
    I422_10lE,
    Y444_10Be,
    Y444_10Le,
    Gbr,
    Gbr10Be,
    Gbr10Le,
    Nv16,
    Nv24,
    Nv12_64Z32,
    A420_10Be,
    A420_10Le,
    A422_10Be,
    A422_10Le,
    A444_10Be,
    A444_10Le,
    Nv61,
    P010_10Be,
    P010_10Le,
    Iyu2,
    Vyuy,
    Gbra,
    Gbra10Be,
    Gbra10Le,
    Bgr10A2Le,
    Rgb10A2Le,
    Gbr12Be,
    Gbr12Le,
    Gbra12Be,
    Gbra12Le,
    I420_12Be,
    I420_12Le,
    I422_12Be,
    I422_12Le,
    Y444_12Be,
    Y444_12Le,
    Gray10Le32,
    Nv12_10Le32,
    Nv16_10Le32,
    Nv12_10Le40,
}

impl PixelFormat {
    pub fn to_gst_format(&self) -> &str {
        match self {
            PixelFormat::I420 => "I420",
            PixelFormat::Yv12 => "YV12",
            PixelFormat::Yuy2 => "YUY2",
            PixelFormat::Uyvy => "UYVY",
            PixelFormat::Ayuv => "AYUV",
            PixelFormat::Vuya => "VUYA",
            PixelFormat::Rgbx => "RGBx",
            PixelFormat::Bgrx => "BGRx",
            PixelFormat::Xrgb => "xRGB",
            PixelFormat::Xbgr => "xBGR",
            PixelFormat::Rgba => "RGBA",
            PixelFormat::Bgra => "BGRA",
            PixelFormat::Argb => "ARGB",
            PixelFormat::Abgr => "ABGR",
            PixelFormat::Rgb => "RGB",
            PixelFormat::Bgr => "BGR",
            PixelFormat::Y41b => "Y41B",
            PixelFormat::Y42b => "Y42B",
            PixelFormat::Yvyu => "YVYU",
            PixelFormat::Y444 => "Y444",
            PixelFormat::V210 => "v210",
            PixelFormat::V216 => "v216",
            PixelFormat::Y210 => "Y210",
            PixelFormat::Y410 => "Y410",
            PixelFormat::Nv12 => "NV12",
            PixelFormat::Nv21 => "NV21",
            PixelFormat::Gray8 => "GRAY8",
            PixelFormat::Gray16Be => "GRAY16_BE",
            PixelFormat::Gray16Le => "GRAY16_LE",
            PixelFormat::V308 => "v308",
            PixelFormat::Rgb16 => "RGB16",
            PixelFormat::Bgr16 => "BGR16",
            PixelFormat::Rgb15 => "RGB15",
            PixelFormat::Bgr15 => "BGR15",
            PixelFormat::Uyvp => "UYVP",
            PixelFormat::A420 => "A420",
            PixelFormat::Rgb8P => "RGB8P",
            PixelFormat::Yuv9 => "YUV9",
            PixelFormat::Yvu9 => "YVU9",
            PixelFormat::Iyu1 => "IYU1",
            PixelFormat::Argb64 => "ARGB64",
            PixelFormat::Ayuv64 => "AYUV64",
            PixelFormat::R210 => "r210",
            PixelFormat::I420_10Be => "I420_10BE",
            PixelFormat::I420_10Le => "I420_10LE",
            PixelFormat::I422_10Be => "I422_10BE",
            PixelFormat::I422_10lE => "I422_10LE",
            PixelFormat::Y444_10Be => "Y444_10BE",
            PixelFormat::Y444_10Le => "Y444_10LE",
            PixelFormat::Gbr => "GBR",
            PixelFormat::Gbr10Be => "GBR_10BE",
            PixelFormat::Gbr10Le => "GBR_10LE",
            PixelFormat::Nv16 => "NV16",
            PixelFormat::Nv24 => "NV24",
            PixelFormat::Nv12_64Z32 => "NV12_64Z32",
            PixelFormat::A420_10Be => "A420_10BE",
            PixelFormat::A420_10Le => "A420_10LE",
            PixelFormat::A422_10Be => "A422_10BE",
            PixelFormat::A422_10Le => "A422_10LE",
            PixelFormat::A444_10Be => "A444_10BE",
            PixelFormat::A444_10Le => "A444_10LE",
            PixelFormat::Nv61 => "NV61",
            PixelFormat::P010_10Be => "P010_10BE",
            PixelFormat::P010_10Le => "P010_10LE",
            PixelFormat::Iyu2 => "IYU2",
            PixelFormat::Vyuy => "VYUY",
            PixelFormat::Gbra => "GBRA",
            PixelFormat::Gbra10Be => "GBRA_10BE",
            PixelFormat::Gbra10Le => "GBRA_10LE",
            PixelFormat::Bgr10A2Le => "BGR10A2_LE",
            PixelFormat::Rgb10A2Le => "RGB10A2_LE",
            PixelFormat::Gbr12Be => "GBR_12BE",
            PixelFormat::Gbr12Le => "GBR_12LE",
            PixelFormat::Gbra12Be => "GBRA_12BE",
            PixelFormat::Gbra12Le => "GBRA_12LE",
            PixelFormat::I420_12Be => "I420_12BE",
            PixelFormat::I420_12Le => "I420_12LE",
            PixelFormat::I422_12Be => "I422_12BE",
            PixelFormat::I422_12Le => "I422_12LE",
            PixelFormat::Y444_12Be => "Y444_12BE",
            PixelFormat::Y444_12Le => "Y444_12LE",
            PixelFormat::Gray10Le32 => "GRAY10_LE32",
            PixelFormat::Nv12_10Le32 => "NV12_10LE32",
            PixelFormat::Nv16_10Le32 => "NV16_10LE32",
            PixelFormat::Nv12_10Le40 => "NV12_10LE40",
        }
    }
}