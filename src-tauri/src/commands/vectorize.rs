use crate::engine::contour;
use crate::engine::types::{VectorizeMode, VectorizeParams, VectorizeResult};
use crate::image::load;
use base64::Engine;

#[tauri::command]
pub fn vectorize_image(data_base64: String, params: VectorizeParams) -> Result<VectorizeResult, String> {
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(&data_base64)
        .map_err(|e| format!("Base64 decode error: {}", e))?;

    let mut img = load::load_from_bytes(&bytes)
        .map_err(|e| format!("Image load error: {}", e))?;

    if params.blur_radius > 0.0 {
        img = load::apply_gaussian_blur(&img, params.blur_radius);
    }

    match params.mode {
        VectorizeMode::Contour => Ok(contour::trace_contour(&img, &params)),
        VectorizeMode::Centerline => Ok(crate::engine::centerline::trace_centerline(&img, &params)),
    }
}
