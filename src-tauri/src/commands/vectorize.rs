use crate::engine::contour;
use crate::engine::types::{BatchImageInput, BatchResultItem, VectorizeMode, VectorizeParams, VectorizeResult};
use crate::image::load;
use base64::Engine;

fn process_single(data_base64: &str, params: &VectorizeParams) -> BatchResultItem {
    let name = String::new();
    let bytes = match base64::engine::general_purpose::STANDARD.decode(data_base64) {
        Ok(b) => b,
        Err(e) => {
            return BatchResultItem {
                name,
                result: VectorizeResult { svg: String::new(), width: 0, height: 0, processing_time_ms: 0 },
                error: Some(format!("Base64 decode error: {}", e)),
            };
        }
    };

    let mut img = match load::load_from_bytes(&bytes) {
        Ok(i) => i,
        Err(e) => {
            return BatchResultItem {
                name,
                result: VectorizeResult { svg: String::new(), width: 0, height: 0, processing_time_ms: 0 },
                error: Some(format!("Image load error: {}", e)),
            };
        }
    };

    if params.blur_radius > 0.0 {
        img = load::apply_gaussian_blur(&img, params.blur_radius);
    }

    let result = match params.mode {
        VectorizeMode::Contour => contour::trace_contour(&img, params),
        VectorizeMode::Centerline => crate::engine::centerline::trace_centerline(&img, params),
    };

    BatchResultItem { name, result, error: None }
}

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

#[tauri::command]
pub fn vectorize_batch(images: Vec<BatchImageInput>) -> Vec<BatchResultItem> {
    use rayon::prelude::*;

    images
        .into_par_iter()
        .map(|img| {
            let mut result = process_single(&img.data_base64, &img.params);
            result.name = img.name;
            result
        })
        .collect()
}
