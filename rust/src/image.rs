/*
 * Contour tracing library
 * https://github.com/STPR/contour_tracing
 *
 * Copyright (c) 2022, STPR - https://github.com/STPR
 *
 * SPDX-License-Identifier: EUPL-1.2
 */

use ::image::{ImageBuffer, Luma};

const O_VERTEX_NO_BORDER: [(i8, i8); 7] = [(0, 1), (0, 0), (0, 0), (0, 0), (1, 0), (0, 0), (1, 1)]; // Bottom left coordinates without a border
const H_VERTEX_NO_BORDER: [(i8, i8); 7] = [(1, 1), (0, 0), (0, 1), (0, 0), (0, 0), (0, 0), (1, 0)]; // Bottom right coordinates without a border
const O_VALUE_FOR_UNSIGNED: [i8; 7] = [-1, 0, -2, 0, -4, 0, -8]; // Value to add into an image buffer (using unsigned integers)
const H_VALUE_FOR_UNSIGNED: [i8; 7] = [4, 0, 8, 0, 1, 0, 2]; // (idem)

/// A function that takes an image buffer, an 8-bit luminance value and an option as input and return a string of SVG Path commands as output.
/// # Examples
/// ```ignore
/// use image::{GrayImage, Luma};
/// use contour_tracing::image::single_l8_to_paths;
/// ```
/// - A simple example with the **closepaths option** set to **true**:
///
/// ```edition2018
/// # use image::{GrayImage, Luma};
/// # use contour_tracing::image::single_l8_to_paths;
/// let mut image_buffer = GrayImage::new(3, 3);
/// let foreground_color: image::Luma<u8> = Luma([1]);
///
/// image_buffer.put_pixel(0, 0, foreground_color);
/// image_buffer.put_pixel(1, 1, foreground_color);
/// image_buffer.put_pixel(2, 2, foreground_color);
///
/// println!("{}", single_l8_to_paths(&mut image_buffer, foreground_color, true));
/// ```
pub fn single_l8_to_paths(buffer: &mut ImageBuffer<Luma<u8>, Vec<u8>>, luma: Luma<u8>, closepaths: bool) -> String {
    for p in buffer.pixels_mut() {
        if p == &luma {
            *p = Luma([31]);
        }
        else {
            *p = Luma([33]);
        }
    }
    let mut paths = String::new();
    let mut ol: usize;
    let mut hl: usize;
    let mut integer_from_luma: i8;
    for cursor_y in 0..buffer.height() {
        ol = 0;
        hl = 0;
        for cursor_x in 0..buffer.width() {
            if ol == hl && buffer[(cursor_x, cursor_y)][0] == 31 {
                trace_single_l8(true, cursor_x, cursor_y, [2, 3, 4, 5, 6, 7, 0, 1], 2, (7, 1, 0), O_VERTEX_NO_BORDER, O_VALUE_FOR_UNSIGNED, buffer, &mut paths, closepaths);
            }
            else if ol > hl && buffer[(cursor_x, cursor_y)][0] == 33 {
                trace_single_l8(false, cursor_x, cursor_y, [4, 5, 6, 7, 0, 1, 2, 3], -2, (1, 7, 6), H_VERTEX_NO_BORDER, H_VALUE_FOR_UNSIGNED, buffer, &mut paths, closepaths);
            }
            integer_from_luma = 32i8 - buffer[(cursor_x, cursor_y)][0] as i8;
            match integer_from_luma.abs() {
                2 |   4 |  10 |  12 => if integer_from_luma > 0 { ol += 1 } else { hl += 1 },
                5 |   7 |  13 |  15 => if integer_from_luma > 0 { ol -= 1 } else { hl -= 1 },
                _ => ()
            }
        }
    }
    paths
}

fn trace_single_l8(outline: bool, cursor_x: u32, cursor_y: u32, mut o: [usize; 8], rot: i8, viv: (usize, usize, usize), vertex: [(i8, i8); 7], value: [i8; 7], buffer: &mut ImageBuffer<Luma<u8>, Vec<u8>>, paths: &mut String, closepaths: bool) {
    let mut tracer_x = cursor_x;
    let mut tracer_y = cursor_y;
    let max_x = buffer.width() - 1;
    let max_y = buffer.height() - 1;
    let mut vertices_nbr: usize = 1;
    paths.push_str(&format!("M{} {}", tracer_x.wrapping_add(vertex[o[0]].0 as u32), tracer_y.wrapping_add(vertex[o[0]].1 as u32)));
    let mut neighbors: [u8; 8];
    let mut rn: u8;
    loop {
        neighbors = [
            if                      tracer_y == 0     { 32 } else { buffer[(tracer_x    , tracer_y - 1)][0] },
            if tracer_x == max_x || tracer_y == 0     { 32 } else { buffer[(tracer_x + 1, tracer_y - 1)][0] },
            if tracer_x == max_x                      { 32 } else { buffer[(tracer_x + 1, tracer_y    )][0] },
            if tracer_x == max_x || tracer_y == max_y { 32 } else { buffer[(tracer_x + 1, tracer_y + 1)][0] },
            if                      tracer_y == max_y { 32 } else { buffer[(tracer_x    , tracer_y + 1)][0] },
            if tracer_x == 0     || tracer_y == max_y { 32 } else { buffer[(tracer_x - 1, tracer_y + 1)][0] },
            if tracer_x == 0                          { 32 } else { buffer[(tracer_x - 1, tracer_y    )][0] },
            if tracer_x == 0     || tracer_y == 0     { 32 } else { buffer[(tracer_x - 1, tracer_y - 1)][0] }
        ];
        rn =
            if outline {
                if neighbors[o[7]] < 32 && neighbors[o[0]] < 32 { 1 }
                else if neighbors[o[0]] < 32 { 2 }
                else if neighbors[o[1]] < 32 && neighbors[o[2]] < 32 { 3 }
                else { 0 }
            }
            else if neighbors[o[1]] > 32 && neighbors[o[0]] > 32 { 1 }
            else if neighbors[o[0]] > 32 { 2 }
            else if neighbors[o[7]] > 32 && neighbors[o[6]] > 32 { 3 }
            else { 0 };
        match rn {
            1 => {
                buffer.put_pixel(tracer_x, tracer_y, Luma([(buffer[(tracer_x, tracer_y)][0] as i8).wrapping_add(value[o[0]]) as u8]));
                tracer_x = tracer_x.wrapping_add(super::MN[o[viv.0]].0 as u32);
                tracer_y = tracer_y.wrapping_add(super::MN[o[viv.0]].1 as u32);
                o.rotate_right(rot.rem_euclid(8) as usize); // Rotate 90 degrees, counterclockwise for the outlines (rot = 2) or clockwise for the holes (rot = -2)
                vertices_nbr += 1;
                if o[0] == 0 || o[0] == 4 { paths.push_str(&format!("H{}", tracer_x.wrapping_add(vertex[o[0]].0 as u32))); } else { paths.push_str(&format!("V{}", tracer_y.wrapping_add(vertex[o[0]].1 as u32))); }
            }
            2 => {
                buffer.put_pixel(tracer_x, tracer_y, Luma([(buffer[(tracer_x, tracer_y)][0] as i8).wrapping_add(value[o[0]]) as u8]));
                tracer_x = tracer_x.wrapping_add(super::MN[o[0]].0 as u32);
                tracer_y = tracer_y.wrapping_add(super::MN[o[0]].1 as u32);
            }
            3 => {
                buffer.put_pixel(tracer_x, tracer_y, Luma([(buffer[(tracer_x, tracer_y)][0] as i8).wrapping_add(value[o[0]]) as u8]));
                o.rotate_left(rot.rem_euclid(8) as usize); // Rotate 90 degrees, clockwise for the outlines (rot = 2) or counterclockwise for the holes (rot = -2)
                buffer.put_pixel(tracer_x, tracer_y, Luma([(buffer[(tracer_x, tracer_y)][0] as i8).wrapping_add(value[o[0]]) as u8]));
                vertices_nbr += 1;
                if o[0] == 0 || o[0] == 4 { paths.push_str(&format!("H{}", tracer_x.wrapping_add(vertex[o[0]].0 as u32))); } else { paths.push_str(&format!("V{}", tracer_y.wrapping_add(vertex[o[0]].1 as u32))); }
                o.rotate_right(rot.rem_euclid(8) as usize);
                tracer_x = tracer_x.wrapping_add(super::MN[o[viv.1]].0 as u32);
                tracer_y = tracer_y.wrapping_add(super::MN[o[viv.1]].1 as u32);
                vertices_nbr += 1;
                if o[0] == 0 || o[0] == 4 { paths.push_str(&format!("H{}", tracer_x.wrapping_add(vertex[o[0]].0 as u32))); } else { paths.push_str(&format!("V{}", tracer_y.wrapping_add(vertex[o[0]].1 as u32))); }
            }
            _ => {
                buffer.put_pixel(tracer_x, tracer_y, Luma([(buffer[(tracer_x, tracer_y)][0] as i8).wrapping_add(value[o[0]]) as u8]));
                o.rotate_left(rot.rem_euclid(8) as usize);
                vertices_nbr += 1;
                if o[0] == 0 || o[0] == 4 { paths.push_str(&format!("H{}", tracer_x.wrapping_add(vertex[o[0]].0 as u32))); } else { paths.push_str(&format!("V{}", tracer_y.wrapping_add(vertex[o[0]].1 as u32))); }
            }
        }
        if tracer_x == cursor_x && tracer_y == cursor_y && vertices_nbr > 2 {
            break;
        }
    }
    loop {
        buffer.put_pixel(tracer_x, tracer_y, Luma([(buffer[(tracer_x, tracer_y)][0] as i8).wrapping_add(value[o[0]]) as u8]));
        if o[0] == viv.2 {
            break;
        }
        o.rotate_left(rot.rem_euclid(8) as usize);
        vertices_nbr += 1;
        if o[0] == 0 || o[0] == 4 { paths.push_str(&format!("H{}", tracer_x.wrapping_add(vertex[o[0]].0 as u32))); } else { paths.push_str(&format!("V{}", tracer_y.wrapping_add(vertex[o[0]].1 as u32))); }
    }
    if closepaths { paths.push('Z'); }
}
