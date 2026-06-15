use crate::engine::types::{VectorizeParams, VectorizeResult};
use image::{DynamicImage, GrayImage, Luma, GenericImageView};
use std::collections::VecDeque;
use std::time::Instant;

const NEIGHBORS: [(i32, i32); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

pub fn trace_centerline(img: &DynamicImage, params: &VectorizeParams) -> VectorizeResult {
    let start = Instant::now();
    let (width, height) = img.dimensions();

    let gray = img.to_luma8();
    let thresholded = binarize(&gray, params.threshold);
    let despeckled = remove_small_components(&thresholded, params.despeckle);
    let skeleton = morphological_thin(&despeckled);
    let paths = trace_skeleton(&skeleton);
    let simplified = simplify_paths(&paths, params.node_optimization);
    let svg = paths_to_svg(&simplified, width, height);

    VectorizeResult {
        svg,
        width,
        height,
        processing_time_ms: start.elapsed().as_millis() as u64,
    }
}

fn binarize(gray: &GrayImage, threshold: u8) -> GrayImage {
    GrayImage::from_fn(gray.width(), gray.height(), |x, y| {
        if gray.get_pixel(x, y)[0] > threshold {
            Luma([0u8])
        } else {
            Luma([255u8])
        }
    })
}

fn remove_small_components(img: &GrayImage, min_size: u32) -> GrayImage {
    if min_size == 0 {
        return img.clone();
    }

    let (w, h) = img.dimensions();
    let mut visited = vec![false; (w * h) as usize];
    let mut output = GrayImage::new(w, h);

    for y in 0..h {
        for x in 0..w {
            let idx = (y * w + x) as usize;
            if visited[idx] || img.get_pixel(x, y)[0] == 0 {
                continue;
            }

            let mut component: Vec<(u32, u32)> = Vec::new();
            let mut queue: VecDeque<(u32, u32)> = VecDeque::new();
            queue.push_back((x, y));
            visited[idx] = true;

            while let Some((cx, cy)) = queue.pop_front() {
                component.push((cx, cy));
                for (dx, dy) in &NEIGHBORS {
                    let nx = cx.wrapping_add_signed(*dx);
                    let ny = cy.wrapping_add_signed(*dy);
                    if nx < w && ny < h {
                        let nidx = (ny * w + nx) as usize;
                        if !visited[nidx] && img.get_pixel(nx, ny)[0] > 0 {
                            visited[nidx] = true;
                            queue.push_back((nx, ny));
                        }
                    }
                }
            }

            if component.len() >= min_size as usize {
                for &(px, py) in &component {
                    output.put_pixel(px, py, Luma([255u8]));
                }
            }
        }
    }
    output
}

fn morphological_thin(img: &GrayImage) -> GrayImage {
    let (w, h) = img.dimensions();
    let mut current = img.clone();
    let mut temp = GrayImage::new(w, h);

    loop {
        let mut changed = false;

        for pass in 0..2 {
            for y in 0..h {
                for x in 0..w {
                    temp.put_pixel(x, y, *current.get_pixel(x, y));
                }
            }

            for y in 1..h - 1 {
                for x in 1..w - 1 {
                    if current.get_pixel(x, y)[0] == 0 {
                        continue;
                    }
                    let p = get_pixel9(&current, x, y);
                    if thin_condition(p, pass == 0) {
                        temp.put_pixel(x, y, Luma([0u8]));
                        changed = true;
                    }
                }
            }
            std::mem::swap(&mut current, &mut temp);
        }

        if !changed {
            break;
        }
    }
    current
}

fn get_pixel9(img: &GrayImage, x: u32, y: u32) -> [u8; 9] {
    let p2 = img.get_pixel(x, y - 1)[0] / 255;
    let p3 = img.get_pixel(x + 1, y - 1)[0] / 255;
    let p4 = img.get_pixel(x + 1, y)[0] / 255;
    let p5 = img.get_pixel(x + 1, y + 1)[0] / 255;
    let p6 = img.get_pixel(x, y + 1)[0] / 255;
    let p7 = img.get_pixel(x - 1, y + 1)[0] / 255;
    let p8 = img.get_pixel(x - 1, y)[0] / 255;
    let p9 = img.get_pixel(x - 1, y - 1)[0] / 255;
    [0, p2, p3, p4, p5, p6, p7, p8, p9]
}

fn thin_condition(p: [u8; 9], first_pass: bool) -> bool {
    let count = p[1] + p[2] + p[3] + p[4] + p[5] + p[6] + p[7] + p[8];
    if count < 2 || count > 6 {
        return false;
    }

    let mut transitions = 0;
    for i in 1..=7 {
        if p[i] == 0 && p[i + 1] == 1 {
            transitions += 1;
        }
    }
    if p[8] == 0 && p[1] == 1 {
        transitions += 1;
    }
    if transitions != 1 {
        return false;
    }

    if first_pass {
        p[1] * p[3] * p[5] == 0 && p[3] * p[5] * p[7] == 0
    } else {
        p[1] * p[3] * p[7] == 0 && p[1] * p[5] * p[7] == 0
    }
}

fn count_skeleton_neighbors(x: u32, y: u32, w: u32, h: u32, skeleton: &GrayImage) -> usize {
    let mut count = 0;
    for &(dx, dy) in &NEIGHBORS {
        let nx = x.wrapping_add_signed(dx);
        let ny = y.wrapping_add_signed(dy);
        if nx < w && ny < h {
            if skeleton.get_pixel(nx, ny)[0] > 0 {
                count += 1;
            }
        }
    }
    count
}

fn get_next_steps(cx: u32, cy: u32, w: u32, h: u32, skeleton: &GrayImage, visited: &[bool]) -> Vec<(u32, u32)> {
    let mut steps = Vec::new();
    for &(dx, dy) in &NEIGHBORS {
        let nx = cx.wrapping_add_signed(dx);
        let ny = cy.wrapping_add_signed(dy);
        if nx < w && ny < h {
            if skeleton.get_pixel(nx, ny)[0] > 0 {
                let nidx = (ny * w + nx) as usize;
                let is_junc = count_skeleton_neighbors(nx, ny, w, h, skeleton) > 2;
                if !visited[nidx] || is_junc {
                    steps.push((nx, ny));
                }
            }
        }
    }
    steps
}

fn trace_skeleton(skeleton: &GrayImage) -> Vec<Vec<(u32, u32)>> {
    let (w, h) = skeleton.dimensions();
    let mut visited = vec![false; (w * h) as usize];
    let mut paths: Vec<Vec<(u32, u32)>> = Vec::new();

    // 1. Find all endpoints
    let mut endpoints = Vec::new();
    for y in 0..h {
        for x in 0..w {
            if skeleton.get_pixel(x, y)[0] > 0 {
                let n_count = count_skeleton_neighbors(x, y, w, h, skeleton);
                if n_count <= 1 {
                    endpoints.push((x, y));
                }
            }
        }
    }

    // 2. Trace from endpoints
    for &(ex, ey) in &endpoints {
        let eidx = (ey * w + ex) as usize;
        if visited[eidx] || skeleton.get_pixel(ex, ey)[0] == 0 {
            continue;
        }

        let mut path = Vec::new();
        let mut cx = ex;
        let mut cy = ey;
        visited[eidx] = true;
        path.push((cx, cy));

        loop {
            let neighbors = get_next_steps(cx, cy, w, h, skeleton, &visited);
            if neighbors.is_empty() {
                break;
            }

            let mut next_pixel = None;
            for &(nx, ny) in &neighbors {
                if count_skeleton_neighbors(nx, ny, w, h, skeleton) > 2 {
                    next_pixel = Some((nx, ny, true));
                    break;
                }
            }

            if next_pixel.is_none() {
                let unvisited: Vec<(u32, u32)> = neighbors.into_iter()
                    .filter(|&(nx, ny)| !visited[(ny * w + nx) as usize])
                    .collect();
                if !unvisited.is_empty() {
                    next_pixel = Some((unvisited[0].0, unvisited[0].1, false));
                }
            }

            if let Some((nx, ny, is_junc)) = next_pixel {
                let nidx = (ny * w + nx) as usize;
                visited[nidx] = true;
                path.push((nx, ny));
                if is_junc {
                    break;
                }
                cx = nx;
                cy = ny;
            } else {
                break;
            }
        }

        if path.len() >= 2 {
            paths.push(path);
        }
    }

    // 3. Trace remaining loops / circles
    for y in 0..h {
        for x in 0..w {
            let idx = (y * w + x) as usize;
            if visited[idx] || skeleton.get_pixel(x, y)[0] == 0 {
                continue;
            }

            let mut path = Vec::new();
            let mut cx = x;
            let mut cy = y;
            visited[idx] = true;
            path.push((cx, cy));

            loop {
                let neighbors = get_next_steps(cx, cy, w, h, skeleton, &visited);
                if neighbors.is_empty() {
                    break;
                }

                let mut next_pixel = None;
                for &(nx, ny) in &neighbors {
                    if count_skeleton_neighbors(nx, ny, w, h, skeleton) > 2 {
                        next_pixel = Some((nx, ny, true));
                        break;
                    }
                }

                if next_pixel.is_none() {
                    let unvisited: Vec<(u32, u32)> = neighbors.into_iter()
                        .filter(|&(nx, ny)| !visited[(ny * w + nx) as usize])
                        .collect();
                    if !unvisited.is_empty() {
                        next_pixel = Some((unvisited[0].0, unvisited[0].1, false));
                    }
                }

                if let Some((nx, ny, is_junc)) = next_pixel {
                    let nidx = (ny * w + nx) as usize;
                    visited[nidx] = true;
                    path.push((nx, ny));
                    if is_junc {
                        break;
                    }
                    cx = nx;
                    cy = ny;
                } else {
                    break;
                }
            }

            if path.len() >= 2 {
                paths.push(path);
            }
        }
    }

    paths
}

fn simplify_paths(paths: &[Vec<(u32, u32)>], node_optimization: f64) -> Vec<Vec<(u32, u32)>> {
    let epsilon = (10.0 - node_optimization) * 0.5 + 0.1;
    paths.iter().map(|p| rdp_simplify(p, epsilon)).collect()
}

fn rdp_simplify(points: &[(u32, u32)], epsilon: f64) -> Vec<(u32, u32)> {
    if points.len() <= 2 {
        return points.to_vec();
    }

    let mut max_dist = 0.0;
    let mut max_idx = 0;
    let first = points[0];
    let last = points[points.len() - 1];

    for i in 1..points.len() - 1 {
        let dist = perpendicular_distance(&first, &last, &points[i]);
        if dist > max_dist {
            max_dist = dist;
            max_idx = i;
        }
    }

    if max_dist > epsilon {
        let mut left = rdp_simplify(&points[..=max_idx], epsilon);
        let right = rdp_simplify(&points[max_idx..], epsilon);
        left.pop();
        left.extend(right);
        left
    } else {
        vec![first, last]
    }
}

fn perpendicular_distance(a: &(u32, u32), b: &(u32, u32), p: &(u32, u32)) -> f64 {
    let (ax, ay) = (a.0 as f64, a.1 as f64);
    let (bx, by) = (b.0 as f64, b.1 as f64);
    let (px, py) = (p.0 as f64, p.1 as f64);

    let dx = bx - ax;
    let dy = by - ay;
    let length_sq = dx * dx + dy * dy;

    if length_sq == 0.0 {
        return ((px - ax).powi(2) + (py - ay).powi(2)).sqrt();
    }

    let t = ((px - ax) * dx + (py - ay) * dy) / length_sq;
    let t = t.clamp(0.0, 1.0);
    let proj_x = ax + t * dx;
    let proj_y = ay + t * dy;
    ((px - proj_x).powi(2) + (py - proj_y).powi(2)).sqrt()
}

fn paths_to_svg(paths: &[Vec<(u32, u32)>], width: u32, height: u32) -> String {
    let mut svg = String::new();
    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="{}" height="{}">"#,
        width, height, width, height
    ));

    for path in paths {
        if path.len() < 2 {
            continue;
        }
        svg.push_str(r##"<path fill="none" stroke="#000" stroke-width="1.5" d=""##);
        svg.push_str(&format!("M {} {}", path[0].0, path[0].1));
        for p in &path[1..] {
            svg.push_str(&format!(" L {} {}", p.0, p.1));
        }
        svg.push_str("\"/>");
    }

    svg.push_str("</svg>");
    svg
}
