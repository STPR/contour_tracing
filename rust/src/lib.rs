/*
 * Contour tracing library
 * https://github.com/STPR/contour_tracing
 *
 * Copyright (c) 2022, STPR - https://github.com/STPR
 *
 * SPDX-License-Identifier: EUPL-1.2
 */

//! A 2D library to trace contours.
//!
//! # Features
//! Core features:
//! - Trace contours using the Theo Pavlidis' algorithm (connectivity: 4-connected)
//! - Trace **outlines** in **clockwise direction**
//! - Trace **holes** in **counterclockwise direction**
//! - Input format: an image buffer or a 2D array of bits
//! - Output format: a string of SVG Path commands
//!
//! Manual parameters:
//! - User can specify to close or not the paths (with the SVG Path **Z** command)
//! 
//! # Examples
//! Have a look at the different functions below.

#![cfg_attr(docsrs, feature(doc_cfg))]

#[allow(unused_imports)]
use std;

#[cfg(feature = "image")]
use image::{ImageBuffer, Luma};

const MN: [(i8, i8); 8] = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)]; // Moore neighborhood

const O_VERTEX_WITH_BORDER: [(i8, i8); 7] = [(-1, 0), (0, 0), (-1, -1), (0, 0), (0, -1), (0, 0), (0, 0)]; // Bottom left coordinates with a border
const H_VERTEX_WITH_BORDER: [(i8, i8); 7] = [(0, 0), (0, 0), (-1, 0), (0, 0), (-1, -1), (0, 0), (0, -1)]; // Bottom right coordinates with a border
const O_VALUE_FOR_SIGNED: [i8; 7] = [1, 0, 2, 0, 4, 0, 8]; // Value to add into an array of contours (using signed integers)
const H_VALUE_FOR_SIGNED: [i8; 7] = [-4, 0, -8, 0, -1, 0, -2]; // (idem)

#[cfg(feature = "image")]
const O_VERTEX_NO_BORDER: [(i8, i8); 7] = [(0, 1), (0, 0), (0, 0), (0, 0), (1, 0), (0, 0), (1, 1)]; // Bottom left coordinates without a border
#[cfg(feature = "image")]
const H_VERTEX_NO_BORDER: [(i8, i8); 7] = [(1, 1), (0, 0), (0, 1), (0, 0), (0, 0), (0, 0), (1, 0)]; // Bottom right coordinates without a border
#[cfg(feature = "image")]
const O_VALUE_FOR_UNSIGNED: [i8; 7] = [-1, 0, -2, 0, -4, 0, -8]; // Value to add into an image buffer (using unsigned integers)
#[cfg(feature = "image")]
const H_VALUE_FOR_UNSIGNED: [i8; 7] = [4, 0, 8, 0, 1, 0, 2]; // (idem)

/*
 buffer: an image buffer
 contours: an array of contours
 ol: outlines level
 hl: holes level
 rn: reachable neighbor - for the outlines: 0: none, 1: front left,  2: front, 3: front right
                        - for the holes:    0: none, 1: front right, 2: front, 3: front left
 o: orientation, e.g. to the east:

          N
    ┏━━━━━━━━━━━┓
    ┃ 7   0   1 ┃
  W ┃ 6   o > 2 ┃ E   o = [2, 3, 4, 5, 6, 7, 0, 1]
    ┃ 5   4   3 ┃
    ┗━━━━━━━━━━━┛
          S
*/

/// A function that takes a 2D array of bits and an option as input and return a string of SVG Path commands as output.
/// # Examples
/// ```
/// use contour_tracing::bits_to_paths;
/// ```
/// - A simple example with the **closepaths option** set to **false**:
///
/// ```
/// # use contour_tracing::bits_to_paths;
/// let bits = vec![vec![ 0,1,1,1,0,0,1,1,1,1,1 ],
///                 vec![ 1,0,0,0,1,0,1,0,0,0,1 ],
///                 vec![ 1,0,0,0,1,0,1,0,1,0,1 ],
///                 vec![ 1,0,0,0,1,0,1,0,0,0,1 ],
///                 vec![ 0,1,1,1,0,0,1,1,1,1,1 ]];
///
/// # assert_eq!(bits_to_paths(bits.to_vec(), false), "M1 0H4V1H1M6 0H11V5H6M0 1H1V4H0M4 1H5V4H4M7 1V4H10V1M8 2H9V3H8M1 4H4V5H1");
/// println!("{}", bits_to_paths(bits, false));
/// ```
/// - When the **closepaths option** is set to **true**, each path is closed with the SVG Path **Z** command:
///
/// ```
/// # use contour_tracing::bits_to_paths;
/// # let bits = vec![vec![ 0,1,1,1,0,0,1,1,1,1,1 ],
/// #                 vec![ 1,0,0,0,1,0,1,0,0,0,1 ],
/// #                 vec![ 1,0,0,0,1,0,1,0,1,0,1 ],
/// #                 vec![ 1,0,0,0,1,0,1,0,0,0,1 ],
/// #                 vec![ 0,1,1,1,0,0,1,1,1,1,1 ]];
/// # assert_eq!(bits_to_paths(bits.to_vec(), true), "M1 0H4V1H1ZM6 0H11V5H6ZM0 1H1V4H0ZM4 1H5V4H4ZM7 1V4H10V1ZM8 2H9V3H8ZM1 4H4V5H1Z");
/// println!("{}", bits_to_paths(bits, true));
/// ```
/// - If you plan to reuse the array of bits after using this function, use the `to_vec()` method like this:
///
/// ```
/// # use contour_tracing::bits_to_paths;
/// let bits = vec![vec![ 1,0,0 ],
///                 vec![ 0,1,0 ],
///                 vec![ 0,0,1 ]];
///
/// # assert_eq!(bits_to_paths(bits.to_vec(), true), "M0 0H1V1H0ZM1 1H2V2H1ZM2 2H3V3H2Z");
/// println!("{}", bits_to_paths(bits.to_vec(), true));
/// println!("{:?}", bits);
/// ```
pub fn bits_to_paths(bits: Vec<Vec<i8>>, closepaths: bool) -> String {
    let rows: usize = bits.len();
    let cols: usize = bits[0].len();

    let mut contours = vec![vec![0i8; cols + 2]; rows + 2]; // Add a border of 1 bit to prevent out-of-bounds error
    for r in 0..=rows - 1 as usize {
        for c in 0..=cols - 1 as usize {
            contours[r + 1][c + 1] = if bits[r][c] == 1 { 1 } else { -1 };
        }
    }

    let mut paths = String::new();
    let mut ol: usize;
    let mut hl: usize;
    for cursor_y in 1..=rows as usize {
        ol = 0;
        hl = 0;
        for cursor_x in 1..=cols as usize {
            if ol == hl && contours[cursor_y][cursor_x] == 1 {
                trace_bits(true, cursor_x, cursor_y, [2, 3, 4, 5, 6, 7, 0, 1], 2, (7, 1, 0), O_VERTEX_WITH_BORDER, O_VALUE_FOR_SIGNED, &mut contours, &mut paths, closepaths);
            }
            else if ol > hl && contours[cursor_y][cursor_x] == -1 {
                trace_bits(false, cursor_x, cursor_y, [4, 5, 6, 7, 0, 1, 2, 3], -2, (1, 7, 6), H_VERTEX_WITH_BORDER, H_VALUE_FOR_SIGNED, &mut contours, &mut paths, closepaths);
            }
            match contours[cursor_y][cursor_x].abs() {
                   2 |   4 |  10 |  12 => if contours[cursor_y][cursor_x] > 0 { ol += 1 } else { hl += 1 },
                   5 |   7 |  13 |  15 => if contours[cursor_y][cursor_x] > 0 { ol -= 1 } else { hl -= 1 },
                _ => ()
            }
        }
    }
    paths
}

fn trace_bits(outline: bool, cursor_x: usize, cursor_y: usize, mut o: [usize; 8], rot: i8, viv: (usize, usize, usize), vertex: [(i8, i8); 7], value: [i8; 7], contours: &mut Vec<Vec<i8>>, paths: &mut String, closepaths: bool) {
    let mut tracer_x = cursor_x;
    let mut tracer_y = cursor_y;
    let mut vertices_nbr: usize = 1;
    paths.push_str(&format!("M{} {}", tracer_x.wrapping_add(vertex[o[0]].0 as usize), tracer_y.wrapping_add(vertex[o[0]].1 as usize)));
    let mut neighbors: [i8; 8];
    let mut rn: u8;
    loop {
        neighbors = [contours[tracer_y - 1][tracer_x], contours[tracer_y - 1][tracer_x + 1], contours[tracer_y][tracer_x + 1], contours[tracer_y + 1][tracer_x + 1], contours[tracer_y + 1][tracer_x], contours[tracer_y + 1][tracer_x - 1], contours[tracer_y][tracer_x - 1], contours[tracer_y - 1][tracer_x - 1]];
        rn =
            if outline {
                if neighbors[o[7]] > 0 && neighbors[o[0]] > 0 { 1 }
                else if neighbors[o[0]] > 0 { 2 }
                else if neighbors[o[1]] > 0 && neighbors[o[2]] > 0 { 3 }
                else { 0 }
            }
            else if neighbors[o[1]] < 0 && neighbors[o[0]] < 0 { 1 }
            else if neighbors[o[0]] < 0 { 2 }
            else if neighbors[o[7]] < 0 && neighbors[o[6]] < 0 { 3 }
            else { 0 };
        match rn {
            1 => {
                contours[tracer_y][tracer_x] += value[o[0]];
                tracer_x = tracer_x.wrapping_add(MN[o[viv.0]].0 as usize);
                tracer_y = tracer_y.wrapping_add(MN[o[viv.0]].1 as usize);
                o.rotate_right(rot.rem_euclid(8) as usize); // Rotate 90 degrees, counterclockwise for the outlines (rot = 2) or clockwise for the holes (rot = -2)
                vertices_nbr += 1;
                if o[0] == 0 || o[0] == 4 { paths.push_str(&format!("H{}", tracer_x.wrapping_add(vertex[o[0]].0 as usize))); } else { paths.push_str(&format!("V{}", tracer_y.wrapping_add(vertex[o[0]].1 as usize))); }
            }
            2 => {
                contours[tracer_y][tracer_x] += value[o[0]];
                tracer_x = tracer_x.wrapping_add(MN[o[0]].0 as usize);
                tracer_y = tracer_y.wrapping_add(MN[o[0]].1 as usize);
            }
            3 => {
                contours[tracer_y][tracer_x] += value[o[0]];
                o.rotate_left(rot.rem_euclid(8) as usize); // Rotate 90 degrees, clockwise for the outlines (rot = 2) or counterclockwise for the holes (rot = -2)
                contours[tracer_y][tracer_x] += value[o[0]];
                vertices_nbr += 1;
                if o[0] == 0 || o[0] == 4 { paths.push_str(&format!("H{}", tracer_x.wrapping_add(vertex[o[0]].0 as usize))); } else { paths.push_str(&format!("V{}", tracer_y.wrapping_add(vertex[o[0]].1 as usize))); }
                o.rotate_right(rot.rem_euclid(8) as usize);
                tracer_x = tracer_x.wrapping_add(MN[o[viv.1]].0 as usize);
                tracer_y = tracer_y.wrapping_add(MN[o[viv.1]].1 as usize);
                vertices_nbr += 1;
                if o[0] == 0 || o[0] == 4 { paths.push_str(&format!("H{}", tracer_x.wrapping_add(vertex[o[0]].0 as usize))); } else { paths.push_str(&format!("V{}", tracer_y.wrapping_add(vertex[o[0]].1 as usize))); }
            }
            _ => {
                contours[tracer_y][tracer_x] += value[o[0]];
                o.rotate_left(rot.rem_euclid(8) as usize);
                vertices_nbr += 1;
                if o[0] == 0 || o[0] == 4 { paths.push_str(&format!("H{}", tracer_x.wrapping_add(vertex[o[0]].0 as usize))); } else { paths.push_str(&format!("V{}", tracer_y.wrapping_add(vertex[o[0]].1 as usize))); }
            }
        }
        if tracer_x == cursor_x && tracer_y == cursor_y && vertices_nbr > 2 {
            break;
        }
    }
    loop {
        contours[tracer_y][tracer_x] += value[o[0]];
        if o[0] == viv.2 {
            break;
        }
        o.rotate_left(rot.rem_euclid(8) as usize);
        vertices_nbr += 1;
        if o[0] == 0 || o[0] == 4 { paths.push_str(&format!("H{}", tracer_x.wrapping_add(vertex[o[0]].0 as usize))); } else { paths.push_str(&format!("V{}", tracer_y.wrapping_add(vertex[o[0]].1 as usize))); }
    }
    if closepaths { paths.push_str("Z"); }
}

#[cfg(feature = "image")]
#[cfg_attr(docsrs, doc(cfg(feature = "image")))]
/// A function that takes an image buffer, an 8-bit luminance value and an option as input and return a string of SVG Path commands as output.
/// # Examples
/// ```
/// use image::{GrayImage, Luma};
/// use contour_tracing::single_l8_to_paths;
/// ```
/// - A simple example with the **closepaths option** set to **true**:
///
/// ```
/// # use image::{GrayImage, Luma};
/// # use contour_tracing::single_l8_to_paths;
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

#[cfg(feature = "image")]
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
        if max_x == 0 && max_y == 0 {
            neighbors = [32, 32, 32, 32, 32, 32, 32, 32];
        }
        else if max_x == 0 && tracer_y == 0 {
            neighbors = [32, 32, 32, 32, buffer[(tracer_x, tracer_y + 1)][0], 32, 32, 32];
        }
        else if max_x == 0 && tracer_y == max_y {
            neighbors = [buffer[(tracer_x, tracer_y - 1)][0], 32, 32, 32, 32, 32, 32, 32];
        }
        else if max_x == 0 {
            neighbors = [buffer[(tracer_x, tracer_y - 1)][0], 32, 32, 32, buffer[(tracer_x, tracer_y + 1)][0], 32, 32, 32];
        }
        else if max_y == 0 && tracer_x == 0 {
            neighbors = [32, 32, buffer[(tracer_x + 1, tracer_y)][0], 32, 32, 32, 32, 32];
        }
        else if max_y == 0 && tracer_x == max_x {
            neighbors = [32, 32, 32, 32, 32, 32, buffer[(tracer_x - 1, tracer_y)][0], 32];
        }
        else if max_y == 0 {
            neighbors = [32, 32, buffer[(tracer_x + 1, tracer_y)][0], 32, 32, 32, buffer[(tracer_x - 1, tracer_y)][0], 32];
        }
        else if tracer_x == 0 && tracer_y == 0 {
            neighbors = [32, 32, buffer[(tracer_x + 1, tracer_y)][0], buffer[(tracer_x + 1, tracer_y + 1)][0], buffer[(tracer_x, tracer_y + 1)][0], 32, 32, 32];
        }
        else if tracer_x == max_x && tracer_y == 0 {
            neighbors = [32, 32, 32, 32, buffer[(tracer_x, tracer_y + 1)][0], buffer[(tracer_x - 1, tracer_y + 1)][0], buffer[(tracer_x - 1, tracer_y)][0], 32];
        }
        else if tracer_x == 0 && tracer_y == max_y {
            neighbors = [buffer[(tracer_x, tracer_y - 1)][0], buffer[(tracer_x + 1, tracer_y - 1)][0], buffer[(tracer_x + 1, tracer_y)][0], 32, 32, 32, 32, 32];
        }
        else if tracer_x == max_x && tracer_y == max_y {
            neighbors = [buffer[(tracer_x, tracer_y - 1)][0], 32, 32, 32, 32, 32, buffer[(tracer_x - 1, tracer_y)][0], buffer[(tracer_x - 1, tracer_y - 1)][0]];
        }
        else if tracer_x == 0 {
            neighbors = [buffer[(tracer_x, tracer_y - 1)][0], buffer[(tracer_x + 1, tracer_y - 1)][0], buffer[(tracer_x + 1, tracer_y)][0], buffer[(tracer_x + 1, tracer_y + 1)][0], buffer[(tracer_x, tracer_y + 1)][0], 32, 32, 32];
        }
        else if tracer_x == max_x {
            neighbors = [buffer[(tracer_x, tracer_y - 1)][0], 32, 32, 32, buffer[(tracer_x, tracer_y + 1)][0], buffer[(tracer_x - 1, tracer_y + 1)][0], buffer[(tracer_x - 1, tracer_y)][0], buffer[(tracer_x - 1, tracer_y - 1)][0]];
        }
        else if tracer_y == 0 {
            neighbors = [32, 32, buffer[(tracer_x + 1, tracer_y)][0], buffer[(tracer_x + 1, tracer_y + 1)][0], buffer[(tracer_x, tracer_y + 1)][0], buffer[(tracer_x - 1, tracer_y + 1)][0], buffer[(tracer_x - 1, tracer_y)][0], 32];
        }
        else if tracer_y == max_y {
            neighbors = [buffer[(tracer_x, tracer_y - 1)][0], buffer[(tracer_x + 1, tracer_y - 1)][0], buffer[(tracer_x + 1, tracer_y)][0], 32, 32, 32, buffer[(tracer_x - 1, tracer_y)][0], buffer[(tracer_x - 1, tracer_y - 1)][0]];
        }
        else {
            neighbors = [buffer[(tracer_x, tracer_y - 1)][0], buffer[(tracer_x + 1, tracer_y - 1)][0], buffer[(tracer_x + 1, tracer_y)][0], buffer[(tracer_x + 1, tracer_y + 1)][0], buffer[(tracer_x, tracer_y + 1)][0], buffer[(tracer_x - 1, tracer_y + 1)][0], buffer[(tracer_x - 1, tracer_y)][0], buffer[(tracer_x - 1, tracer_y - 1)][0]];
        }
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
                tracer_x = tracer_x.wrapping_add(MN[o[viv.0]].0 as u32);
                tracer_y = tracer_y.wrapping_add(MN[o[viv.0]].1 as u32);
                o.rotate_right(rot.rem_euclid(8) as usize); // Rotate 90 degrees, counterclockwise for the outlines (rot = 2) or clockwise for the holes (rot = -2)
                vertices_nbr += 1;
                if o[0] == 0 || o[0] == 4 { paths.push_str(&format!("H{}", tracer_x.wrapping_add(vertex[o[0]].0 as u32))); } else { paths.push_str(&format!("V{}", tracer_y.wrapping_add(vertex[o[0]].1 as u32))); }
            }
            2 => {
                buffer.put_pixel(tracer_x, tracer_y, Luma([(buffer[(tracer_x, tracer_y)][0] as i8).wrapping_add(value[o[0]]) as u8]));
                tracer_x = tracer_x.wrapping_add(MN[o[0]].0 as u32);
                tracer_y = tracer_y.wrapping_add(MN[o[0]].1 as u32);
            }
            3 => {
                buffer.put_pixel(tracer_x, tracer_y, Luma([(buffer[(tracer_x, tracer_y)][0] as i8).wrapping_add(value[o[0]]) as u8]));
                o.rotate_left(rot.rem_euclid(8) as usize); // Rotate 90 degrees, clockwise for the outlines (rot = 2) or counterclockwise for the holes (rot = -2)
                buffer.put_pixel(tracer_x, tracer_y, Luma([(buffer[(tracer_x, tracer_y)][0] as i8).wrapping_add(value[o[0]]) as u8]));
                vertices_nbr += 1;
                if o[0] == 0 || o[0] == 4 { paths.push_str(&format!("H{}", tracer_x.wrapping_add(vertex[o[0]].0 as u32))); } else { paths.push_str(&format!("V{}", tracer_y.wrapping_add(vertex[o[0]].1 as u32))); }
                o.rotate_right(rot.rem_euclid(8) as usize);
                tracer_x = tracer_x.wrapping_add(MN[o[viv.1]].0 as u32);
                tracer_y = tracer_y.wrapping_add(MN[o[viv.1]].1 as u32);
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
    if closepaths { paths.push_str("Z"); }
}