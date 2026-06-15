use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VectorizeMode {
    #[serde(rename = "contour")]
    Contour,
    #[serde(rename = "centerline")]
    Centerline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorizeParams {
    pub mode: VectorizeMode,
    pub threshold: u8,
    pub blur_radius: f64,
    pub despeckle: u32,
    pub sparsity: f64,
    pub node_optimization: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorizeResult {
    pub svg: String,
    pub width: u32,
    pub height: u32,
    pub processing_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchImageInput {
    pub data_base64: String,
    pub name: String,
    pub params: VectorizeParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResultItem {
    pub name: String,
    pub result: VectorizeResult,
    pub error: Option<String>,
}
