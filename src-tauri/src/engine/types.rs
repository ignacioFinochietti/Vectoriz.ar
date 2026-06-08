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
