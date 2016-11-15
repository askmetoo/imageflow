#![feature(plugin, custom_derive)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;

use std::ascii::AsciiExt;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}


mod nodes {
    #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
    pub struct Decode {
        pub io_id: i32,
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
    pub enum Encoder {
        Png,
        Png24,
        Png8,
        Jpeg,
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
    pub struct Encode {
        pub io_id: i32,
        pub encoder: Option<Encoder>,
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
    pub enum AnyNode {
        Decode(Decode),
        Encode(Encode),
    }

}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum MNode {
    Decode { io_id: i32 },
    Encode {
        io_id: i32,
        encoder: Option<nodes::Encoder>,
    },
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum PixelFormat {
    Bgra32,
    Bgr24,
    Gray8,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Encoder {
    Png,
    Jpeg,
}



#[repr(C)]
#[derive(Copy, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Debug)]
pub enum Filter {
    RobidouxFast = 1,
    Robidoux = 2,
    RobidouxSharp = 3,
    Ginseng = 4,
    GinsengSharp = 5,
    Lanczos = 6,
    LanczosSharp = 7,
    Lanczos2 = 8,
    Lanczos2Sharp = 9,
    CubicFast = 10,
    Cubic = 11,
    CubicSharp = 12,
    CatmullRom = 13,
    Mitchell = 14,

    CubicBSpline = 15,
    Hermite = 16,
    Jinc = 17,
    RawLanczos3 = 18,
    RawLanczos3Sharp = 19,
    RawLanczos2 = 20,
    RawLanczos2Sharp = 21,
    Triangle = 22,
    Linear = 23,
    Box = 24,
    CatmullRomFast = 25,
    CatmullRomFastSharp = 26,

    Fastest = 27,

    MitchellFast = 28,
    NCubic = 29,
    NCubicSharp = 30,
}
impl FromStr for Filter {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_ascii_lowercase() {
            "robidouxfast" => Ok(Filter::RobidouxFast),
            "robidoux" => Ok(Filter::Robidoux),
            "robidouxsharp" => Ok(Filter::RobidouxSharp),
            "ginseng" => Ok(Filter::Ginseng),
            "ginsengsharp" => Ok(Filter::GinsengSharp),
            "lanczos" => Ok(Filter::Lanczos),
            "lanczossharp" => Ok(Filter::LanczosSharp),
            "lanczos2" => Ok(Filter::Lanczos2),
            "lanczos2sharp" => Ok(Filter::Lanczos2Sharp),
            "cubicfast" => Ok(Filter::CubicFast),
            "cubic_0_1" => Ok(Filter::Cubic),
            "cubicsharp" => Ok(Filter::CubicSharp),
            "catmullrom" => Ok(Filter::CatmullRom),
            "catrom" => Ok(Filter::CatmullRom),
            "mitchell" => Ok(Filter::Mitchell),
            "cubicbspline" => Ok(Filter::CubicBSpline),
            "bspline" => Ok(Filter::CubicBSpline),
            "hermite" => Ok(Filter::Hermite),
            "jinc" => Ok(Filter::Jinc),
            "rawlanczos3" => Ok(Filter::RawLanczos3),
            "rawlanczos3sharp" => Ok(Filter::RawLanczos3Sharp),
            "rawlanczos2" => Ok(Filter::RawLanczos2),
            "rawlanczos2sharp" => Ok(Filter::RawLanczos2Sharp),
            "triangle" => Ok(Filter::Triangle),
            "linear" => Ok(Filter::Linear),
            "box" => Ok(Filter::Box),
            "catmullromfast" => Ok(Filter::CatmullRomFast),
            "catmullromfastsharp" => Ok(Filter::CatmullRomFastSharp),
            "fastest" => Ok(Filter::Fastest),
            "mitchellfast" => Ok(Filter::MitchellFast),
            "ncubic" => Ok(Filter::NCubic),
            "ncubicsharp" => Ok(Filter::NCubicSharp),
            _ => Err("no match"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum PngBitDepth{
    Png32,
    Png24,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum EncoderPreset {
    LibjpegTurbo { quality: Option<i32> },
    Libpng {  depth: Option<PngBitDepth>, matte: Option<Color>, zlib_compression: Option<i32>}
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum ColorSrgb {
    Hex(String),
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Color {
    Transparent,
    Black,
    Srgb(ColorSrgb),
}

impl Color {
    pub fn to_u32_bgra(self) -> std::result::Result<u32, std::num::ParseIntError> {
        self.to_u32_rgba().and_then(|u| Ok(u.swap_bytes().rotate_left(8)))
    }
    pub fn to_u32_rgba(self) -> std::result::Result<u32, std::num::ParseIntError> {
        match self {
            Color::Srgb(srgb) => {
                match srgb {
                    ColorSrgb::Hex(hex_srgb) => {
                        u32::from_str_radix(hex_srgb.as_str(), 16)
                            .and_then(|value| if hex_srgb.len() <= 6 {
                                Ok(value.checked_shl(8).unwrap() | 0xFF)
                            } else {
                                Ok(value)
                            })
                    }
                }
            },
            Color::Black => Ok(0x000000FF),
            Color::Transparent => Ok(0),
        }
    }
}


#[test]
fn test_color() {

    assert_eq!(Color::Srgb(ColorSrgb::Hex("FFAAEEDD".to_owned())).to_u32_rgba().unwrap(),
               0xFFAAEEDD);
    assert_eq!(Color::Srgb(ColorSrgb::Hex("FFAAEE".to_owned())).to_u32_rgba().unwrap(),
               0xFFAAEEFF);
}

#[test]
fn test_bgra() {

    assert_eq!(Color::Srgb(ColorSrgb::Hex("FFAAEEDD".to_owned())).to_u32_bgra().unwrap(),
               0xEEAAFFDD);
    assert_eq!(Color::Srgb(ColorSrgb::Hex("FFAAEE".to_owned())).to_u32_bgra().unwrap(),
               0xEEAAFFFF);
    assert_eq!(Color::Srgb(ColorSrgb::Hex("000000FF".to_owned())).to_u32_bgra().unwrap(),
               0x000000FF);


}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ResampleHints{
    #[serde(rename="sharpenPercent")]
    pub sharpen_percent: Option<f32>,

    #[serde(rename="prefer1dTwice")]
    pub prefer_1d_twice: Option<bool>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Node {
    #[serde(rename="flipV")]
    FlipV,
    #[serde(rename="flipH")]
    FlipH,
    #[serde(rename="crop")]
    Crop { x1: u32, y1: u32, x2: u32, y2: u32 },
    #[serde(rename="createCanvas")]
    CreateCanvas {
        format: PixelFormat,
        w: usize,
        h: usize,
        color: Color,
    },
    #[serde(rename="copyRectToCanvas")]
    CopyRectToCanvas {
        #[serde(rename="fromX")]
        from_x: u32,
        #[serde(rename="fromY")]
        from_y: u32,
        width: u32,
        height: u32,
        x: u32,
        y: u32,
    },
    #[serde(rename="decode")]
    Decode {
        #[serde(rename="ioId")]
        io_id: i32,
    },
    #[serde(rename="encode")]
    Encode {
        #[serde(rename="ioId")]
        io_id: i32,
        preset: EncoderPreset
    },
    #[serde(rename="fillRect")]
    FillRect {
        x1: u32,
        y1: u32,
        x2: u32,
        y2: u32,
        color: Color,
    },
    #[serde(rename="expandCanvas")]
    ExpandCanvas {
        left: u32,
        top: u32,
        right: u32,
        bottom: u32,
        color: Color,
    },
    #[serde(rename="transpose")]
    Transpose,
    #[serde(rename="rotate90")]
    Rotate90,
    #[serde(rename="rotate180")]
    Rotate180,
    #[serde(rename="rotate270")]
    Rotate270,
    #[serde(rename="applyOrientation")]
    ApplyOrientation { flag: i32 },
    #[serde(rename="resample2d")]
    Resample2D {
        w: usize,
        h: usize,
        #[serde(rename="downFilter")]
        down_filter: Option<Filter>,
        #[serde(rename="upFilter")]
        up_filter: Option<Filter>,
        hints: Option<ResampleHints>
    },

    #[serde(rename="resample1d")]
    Resample1D {
        scale_to_width: usize,
        transpose_on_write: bool,
        interpolation_filter: Option<Filter>,
    },
    // TODO: Block use except from FFI/unit test use
    #[serde(rename="flowBitmapBgraPtr")]
    FlowBitmapBgraPtr {
        #[serde(rename="ptrToFlowBitmapBgraPtr")]
        ptr_to_flow_bitmap_bgra_ptr: usize,
    },
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum EdgeKind {
    #[serde(rename="input")]
    Input,
    #[serde(rename="canvas")]
    Canvas,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Edge {
    pub from: i32,
    pub to: i32,
    pub kind: EdgeKind,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Graph {
    pub nodes: std::collections::HashMap<String, Node>,
    pub edges: Vec<Edge>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum TestEnum {
    A,
    B { c: i32 },
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum IoDirection {
    #[serde(rename="output")]
    Output = 8,
    #[serde(rename="input")]
    Input = 4,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum IoEnum {
    #[serde(rename="bytesHex")]
    BytesHex(String),
    #[serde(rename="byteArray")]
    ByteArray(Vec<u8>),
    #[serde(rename="file")]
    Filename(String),
    #[serde(rename="url")]
    Url(String),
    #[serde(rename="outputBuffer")]
    OutputBuffer,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]

pub enum IoChecksum {
    #[serde(rename="djb2Hex")]
    Djb2Hex(String),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct IoObject {
    #[serde(rename="ioId")]
    pub io_id: i32,
    pub direction: IoDirection,
    pub io: IoEnum,
    pub checksum: Option<IoChecksum>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Framewise {
    #[serde(rename="graph")]
    Graph(Graph),
    #[serde(rename="steps")]
    Steps(Vec<Node>),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Build001GraphRecording {
    pub record_graph_versions: Option<bool>,
    pub record_frame_images: Option<bool>,
    pub render_last_graph: Option<bool>,
    pub render_graph_versions: Option<bool>,
    pub render_animated_graph: Option<bool>,
}

impl  Build001GraphRecording {
    pub fn debug_defaults() -> Build001GraphRecording {
        Build001GraphRecording {
            record_graph_versions: Some(true),
            record_frame_images: Some(true),
            render_last_graph: Some(true),
            render_animated_graph: Some(false),
            render_graph_versions: Some(false),
        }
    }
    pub fn off() -> Build001GraphRecording {
        Build001GraphRecording {
            record_graph_versions: Some(false),
            record_frame_images: Some(false),
            render_last_graph: Some(false),
            render_animated_graph: Some(false),
            render_graph_versions: Some(false),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Build001Config {
    #[serde(rename="enableJpegBlockScaling")]
    pub enable_jpeg_block_scaling: Option<bool>,
    #[serde(rename="processAllGifFrames")]
    pub process_all_gif_frames: Option<bool>,
    #[serde(rename="graphRecording")]
    pub graph_recording: Option<Build001GraphRecording>,
    #[serde(rename="noGammaCorrection")]
    pub no_gamma_correction: bool,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Build001 {
    #[serde(rename="builderConfig")]
    pub builder_config: Option<Build001Config>,
    pub io: Vec<IoObject>,
    pub framewise: Framewise,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Execute001 {
    #[serde(rename="noGammaCorrection")]
    pub no_gamma_correction: Option<bool>,
    #[serde(rename="graphRecording")]
    pub graph_recording: Option<Build001GraphRecording>,
    pub framewise: Framewise,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct GetImageInfo001 {
    #[serde(rename="ioId")]
    pub io_id: i32,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct JpegIDCTDownscaleHints {
    pub width: i64,
    pub height: i64,
    #[serde(rename="scaleLumaSpatially")]
    pub scale_luma_spatially: Option<bool>,
    #[serde(rename="gammaCorrectForSrgbDuringSpatialLumaScaling")]
    pub gamma_correct_for_srgb_during_spatial_luma_scaling: Option<bool>
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum TellDecoderWhat {
    JpegDownscaleHints(JpegIDCTDownscaleHints)
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TellDecoder001 {
    #[serde(rename="ioId")]
    pub io_id: i32,
    pub command: TellDecoderWhat
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ImageInfo {
    #[serde(rename="preferredMimeType")]
    pub preferred_mime_type: String,
    #[serde(rename="preferredExtension")]
    pub preferred_extension: String,
    //Warning, one cannot count frames in a GIF without scanning the whole thing.
    #[serde(rename="frameCount")]
    pub frame_count: usize,
    #[serde(rename="currentFrameIndex")]
    pub current_frame_index: i64,
    #[serde(rename="frame0Width")]
    pub frame0_width: i32,
    #[serde(rename="frame0Height")]
    pub frame0_height: i32,
    #[serde(rename="frame0PostDecodeFormat")]
    pub frame0_post_decode_format: PixelFormat,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum ResponsePayload {
    ImageInfo(ImageInfo),
    None,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Response001 {
    pub code: i64,
    pub success: bool,
    pub message: Option<String>,
    pub data: ResponsePayload
}


#[test]
fn test_roundtrip() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    assert_eq!(serialized, r#"{"x":1,"y":2}"#);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized, Point { x: 1, y: 2 });
}


#[test]
fn test_decode_node() {
    let text = r#"{"Decode": { "io_id": 1 } }"#;

    let obj: nodes::AnyNode = serde_json::from_str(&text).unwrap();

    assert_eq!(obj, nodes::AnyNode::Decode(nodes::Decode { io_id: 1 }));
}


#[test]
fn test_decode_mnode() {
    let text = r#"[{"Decode": { "io_id": 1 } }, {"Encode": { "io_id": 2 } }]"#;

    let obj: Vec<MNode> = serde_json::from_str(&text).unwrap();

    assert_eq!(obj,
               vec![MNode::Decode { io_id: 1 },
                    MNode::Encode {
                        io_id: 2,
                        encoder: None,
                    }]);
}

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[test]
fn decode_graph() {
    let text = r#"{
        "nodes": {
            "0": {"decode": { "ioId": 1 } },
            "1": {"rotate90" : null}

        },
        "edges": [
            {"from": 0, "to": 1, "kind": "input"}
        ]
    }"#;

    let obj: Graph = serde_json::from_str(&text).unwrap();
    let expected = Graph {
        nodes: hashmap![ "0".to_owned() => Node::Decode{ io_id: 1 },
                         "1".to_owned() => Node::Rotate90
        ],
        edges: vec![Edge {
                        from: 0,
                        to: 1,
                        kind: EdgeKind::Input,
                    }],
    };

    assert_eq!(obj, expected);
}

#[test]
fn error_from_string() {
    let text = r#"{ "B": { "c": "hi" } }"#;

    let val: Result<TestEnum, serde_json::Error> = serde_json::from_str(text);

    let (code, line, chr) = match val {
        Err(e) => {
            match e {
                serde_json::Error::Syntax(code, line, char) => (code, line, char),
                _ => {
                    assert!(false);
                    unreachable!()
                }
            }
        }
        _ => {
            assert!(false);
            unreachable!()
        }
    };

    assert_eq!(code,
               serde_json::ErrorCode::InvalidType(serde::de::Type::Str));
    assert_eq!(line, 1);
    assert_eq!(chr, 18);
}

#[test]
fn error_from_value() {

    let text = r#"{ "B": { "c": "hi" } }"#;

    let val: serde_json::Value = serde_json::from_str(text).unwrap();

    let x: Result<TestEnum, serde_json::Error> = serde_json::from_value(val);

    let (code, line, chr) = match x {
        Err(e) => {
            match e {
                serde_json::Error::Syntax(code, line, char) => (code, line, char),
                _ => {
                    assert!(false);
                    unreachable!()
                }
            }
        }
        _ => {
            assert!(false);
            unreachable!()
        }
    };

    assert_eq!(code,
               serde_json::ErrorCode::InvalidType(serde::de::Type::Str));
    assert_eq!(line, 0);
    assert_eq!(chr, 0);
    // When parsing from a value, we cannot tell which line or character caused it. I suppose we
    // must serialize/deserialize again, in order to inject an indicator into the text?
    // We cannot recreate the original location AFAICT

}
