use crate::engine::types::{VectorizeParams, VectorizeResult};
use image::{DynamicImage, GenericImageView};
use std::time::Instant;

pub fn trace_contour(img: &DynamicImage, params: &VectorizeParams) -> VectorizeResult {
    let start = Instant::now();
    let (width, height) = img.dimensions();

    let rgba = img.to_rgba8();
    let raw: Vec<u8> = rgba.into_raw();

    let vtracer_img = vtracer::ColorImage {
        pixels: raw,
        width: width as usize,
        height: height as usize,
    };

    let mut config = vtracer::Config::default();
    config.hierarchical = vtracer::Hierarchical::Stacked;
    config.filter_speckle = params.despeckle as usize;
    config.color_precision = 6;
    config.layer_difference = 249;
    config.corner_threshold = params.sparsity as i32;
    config.length_threshold = 4.0;
    config.max_iterations = 10;
    config.splice_threshold = 45;
    config.path_precision = Some((params.node_optimization * 10.0) as u32);

    let svg_file = vtracer::convert(vtracer_img, config)
        .unwrap_or_else(|e| {
            eprintln!("VTracer error: {}", e);
            vtracer::SvgFile::new(
                width as usize,
                height as usize,
                Some((params.node_optimization * 10.0) as u32),
            )
        });

    VectorizeResult {
        svg: svg_file.to_string(),
        width,
        height,
        processing_time_ms: start.elapsed().as_millis() as u64,
    }
}
