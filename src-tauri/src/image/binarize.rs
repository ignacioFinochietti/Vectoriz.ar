use image::{DynamicImage, GrayImage, Luma};

pub fn otsu_threshold(histogram: &[u32; 256], total: u32) -> u8 {
    let mut best_threshold = 0;
    let mut max_variance = 0.0;

    let mut sum_bg: u64 = 0;
    let mut weight_bg: u64 = 0;
    let mut sum_total: u64 = 0;

    for (i, &count) in histogram.iter().enumerate() {
        sum_total += (i as u64) * (count as u64);
    }

    for t in 0..256 {
        weight_bg += histogram[t] as u64;
        if weight_bg == 0 {
            continue;
        }

        sum_bg += (t as u64) * (histogram[t] as u64);
        let weight_fg = total as u64 - weight_bg;
        if weight_fg == 0 {
            break;
        }

        let mean_bg = sum_bg as f64 / weight_bg as f64;
        let mean_fg = (sum_total - sum_bg) as f64 / weight_fg as f64;

        let variance = weight_bg as f64 * weight_fg as f64 * (mean_bg - mean_fg).powi(2);
        if variance > max_variance {
            max_variance = variance;
            best_threshold = t as u8;
        }
    }
    best_threshold
}

pub fn binarize(img: &DynamicImage, threshold: u8, use_otsu: bool) -> GrayImage {
    let gray = img.to_luma8();
    let threshold = if use_otsu {
        let mut histogram = [0u32; 256];
        for p in gray.pixels() {
            histogram[p[0] as usize] += 1;
        }
        let total = gray.width() * gray.height();
        otsu_threshold(&histogram, total)
    } else {
        threshold
    };

    GrayImage::from_fn(gray.width(), gray.height(), |x, y| {
        if gray.get_pixel(x, y)[0] > threshold {
            Luma([255u8])
        } else {
            Luma([0u8])
        }
    })
}
