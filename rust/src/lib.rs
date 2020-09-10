/*
 * Contour tracing library (Rust)
 * https://github.com/STPR/contour_tracing
 *
 * Copyright (c) 2020, STPR - https://github.com/STPR
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
//! - Input format: a 2D array of bits
//! - Output format: a string of SVG Path commands
//!
//! Manual parameters:
//! - User can specify to close or not the paths (with the SVG Path **Z** command)
//! 
//! # Examples
//! For examples, have a look at the **bits_to_paths** function below.

#[allow(unused_imports)]
use std;

static T: [(i8, i8); 8] = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];
static O_VERTEX: [(i8, i8); 7] = [(-1, 0), (0, 0), (-1, -1), (0, 0), (0, -1), (0, 0), (0, 0)]; // Vertex coordinates for the outlines (bottom left) according to the orientation
static H_VERTEX: [(i8, i8); 7] = [(0, 0), (0, 0), (-1, 0), (0, 0), (-1, -1), (0, 0), (0, -1)]; // Vertex coordinates for the holes (bottom right) according to the orientation
static O_VALUE: [i8; 7] = [1, 0, 2, 0, 4, 0, 8]; // Value to add into the array of contours for the outlines
static H_VALUE: [i8; 7] = [-4, 0, -8, 0, -1, 0, -2]; // Value to add into the array of contours for the holes

/*
 contours: an array of contours
 ol: outlines level
 hl: holes level
 rn: reachable neighbor - For outlines -> 0: none, 1: front left neighbor,  2: front neighbor, 3: front right neighbor
                        - For holes    -> 0: none, 1: front right neighbor, 2: front neighbor, 3: front left neighbor
 o: orientation:

            North
        ┌───────────┐
        │ 7   0   1 │
   West │ 6   o   2 │ East
        │ 5   4   3 │
        └───────────┘
            South

 - To the north, o = {0, 1, 2, 3, 4, 5, 6, 7}
 - To the east,  o = {2, 3, 4, 5, 6, 7, 0, 1}
 - To the south, o = {4, 5, 6, 7, 0, 1, 2, 3}
 - To the west,  o = {6, 7, 0, 1, 2, 3, 4, 5}
*/

/// A function that takes a 2D array of bits and an option as input and return a string of SVG Path commands as output.
/// # Examples
/// ```
/// extern crate contour_tracing;
/// use contour_tracing::bits_to_paths;
/// ```
/// - A simple example with the **closepaths option** set to **false**:
/// ```
/// # extern crate contour_tracing;
/// # use contour_tracing::bits_to_paths;
/// let bits = vec![vec![ 0,0,0,0,0,0,0,0,0,0,0,0,0 ],
///                 vec![ 0,0,1,1,1,0,0,1,1,1,1,1,0 ],
///                 vec![ 0,1,0,0,0,1,0,1,0,0,0,1,0 ],
///                 vec![ 0,1,0,0,0,1,0,1,0,1,0,1,0 ],
///                 vec![ 0,1,0,0,0,1,0,1,0,0,0,1,0 ],
///                 vec![ 0,0,1,1,1,0,0,1,1,1,1,1,0 ],
///                 vec![ 0,0,0,0,0,0,0,0,0,0,0,0,0 ]];
///
/// # assert_eq!(bits_to_paths(bits.to_vec(), false), "M2 1H5V2H2M7 1H12V6H7M1 2H2V5H1M5 2H6V5H5M8 2V5H11V2M9 3H10V4H9M2 5H5V6H2");
/// println!("{}", bits_to_paths(bits, false));
/// ```
/// - When the **closepaths option** is set to **true**, each path is closed with the SVG Path **Z** command:
/// ```
/// # extern crate contour_tracing;
/// # use contour_tracing::bits_to_paths;
/// # let bits = vec![vec![ 0,0,0,0,0,0,0,0,0,0,0,0,0 ],
/// #                 vec![ 0,0,1,1,1,0,0,1,1,1,1,1,0 ],
/// #                 vec![ 0,1,0,0,0,1,0,1,0,0,0,1,0 ],
/// #                 vec![ 0,1,0,0,0,1,0,1,0,1,0,1,0 ],
/// #                 vec![ 0,1,0,0,0,1,0,1,0,0,0,1,0 ],
/// #                 vec![ 0,0,1,1,1,0,0,1,1,1,1,1,0 ],
/// #                 vec![ 0,0,0,0,0,0,0,0,0,0,0,0,0 ]];
/// # assert_eq!(bits_to_paths(bits.to_vec(), true), "M2 1H5V2H2ZM7 1H12V6H7ZM1 2H2V5H1ZM5 2H6V5H5ZM8 2V5H11V2ZM9 3H10V4H9ZM2 5H5V6H2Z");
/// println!("{}", bits_to_paths(bits, true));
/// ```
/// - If you plan to reuse the array of bits after using this function, use the `to_vec()` method like this:
///
/// ```
/// # extern crate contour_tracing;
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

    let mut contours = vec![vec![0i8; cols + 2]; rows + 2]; // The array of contours needs a border of 1 bit
    for y in 0..=rows - 1 as usize {
        for x in 0..=cols - 1 as usize {
            contours[y + 1][x + 1] = bits[y][x];
        }
    }

    let mut paths = String::new();
    let mut ol: usize;
    let mut hl: usize;
    for y in 1..=rows as usize {
        ol = 1;
        hl = 1;
        for x in 1..=cols as usize {
            if ol == hl && contours[y][x] == 1 {
                trace(true, x, y, [2, 3, 4, 5, 6, 7, 0, 1], 2, (7, 1, 0), O_VERTEX, O_VALUE, &mut contours, &mut paths, closepaths);
            }
            else if ol > hl && contours[y][x] == 0 {
                trace(false, x, y, [4, 5, 6, 7, 0, 1, 2, 3], -2, (1, 7, 6), H_VERTEX, H_VALUE, &mut contours, &mut paths, closepaths);
            }
            match contours[y][x] {
                2 | 4 | 10 | 12 => ol += 1,
                5 | 7 | 13 | 15 => ol -= 1,
                -1 | -3 | -9 | -11 => hl += 1,
                -4 | -6 | -12 | -14 => hl -= 1,
                _ => ()
            }
        }
    }
    paths
}

fn trace(outline: bool, x: usize, y: usize, mut o: [usize; 8], rot: i8, viv: (usize, usize, usize), c_vertex: [(i8, i8); 7], c_value: [i8; 7], contours: &mut Vec<Vec<i8>>, paths: &mut String, closepaths: bool) {
    let mut cx = x; // Current x
    let mut cy = y; // Current y
    let mut v: usize = 1; // Number of vertices
    paths.push_str(&format!("M{} {}", cx.wrapping_add(c_vertex[o[0]].0 as usize), cy.wrapping_add(c_vertex[o[0]].1 as usize)));
    let mut rn: u8;
    loop {
        let neighbors: [i8; 8] = [contours[cy - 1][cx], contours[cy - 1][cx + 1], contours[cy][cx + 1], contours[cy + 1][cx + 1], contours[cy + 1][cx], contours[cy + 1][cx - 1], contours[cy][cx - 1], contours[cy - 1][cx - 1]];
        rn =
            if outline {
                if neighbors[o[7]] > 0 && neighbors[o[0]] > 0 { 1 }
                else if neighbors[o[0]] > 0 { 2 }
                else if neighbors[o[1]] > 0 && neighbors[o[2]] > 0 { 3 }
                else { 0 }
            } else {
                if neighbors[o[1]] <= 0 && neighbors[o[0]] <= 0 { 1 }
                else if neighbors[o[0]] <= 0 { 2 }
                else if neighbors[o[7]] <= 0 && neighbors[o[6]] <= 0 { 3 }
                else { 0 }
            };
        match rn {
            1 => {
                contours[cy][cx] += c_value[o[0]];
                cx = cx.wrapping_add(T[o[viv.0]].0 as usize);
                cy = cy.wrapping_add(T[o[viv.0]].1 as usize);
                o.rotate_right(rot.rem_euclid(8) as usize); // Rotate 90 degrees, counterclockwise for the outlines (rot = 2) or clockwise for the holes (rot = -2)
                v += 1;
                if o[0] == 0 || o[0] == 4 { paths.push_str(&format!("H{}", cx.wrapping_add(c_vertex[o[0]].0 as usize))); } else { paths.push_str(&format!("V{}", cy.wrapping_add(c_vertex[o[0]].1 as usize))); }
            }
            2 => {
                contours[cy][cx] += c_value[o[0]];
                cx = cx.wrapping_add(T[o[0]].0 as usize);
                cy = cy.wrapping_add(T[o[0]].1 as usize);
            }
            3 => {
                contours[cy][cx] += c_value[o[0]];
                o.rotate_left(rot.rem_euclid(8) as usize); // Rotate 90 degrees, clockwise for the outlines (rot = 2) or counterclockwise for the holes (rot = -2)
                contours[cy][cx] += c_value[o[0]];
                v += 1;
                if o[0] == 0 || o[0] == 4 { paths.push_str(&format!("H{}", cx.wrapping_add(c_vertex[o[0]].0 as usize))); } else { paths.push_str(&format!("V{}", cy.wrapping_add(c_vertex[o[0]].1 as usize))); }
                o.rotate_right(rot.rem_euclid(8) as usize);
                cx = cx.wrapping_add(T[o[viv.1]].0 as usize);
                cy = cy.wrapping_add(T[o[viv.1]].1 as usize);
                v += 1;
                if o[0] == 0 || o[0] == 4 { paths.push_str(&format!("H{}", cx.wrapping_add(c_vertex[o[0]].0 as usize))); } else { paths.push_str(&format!("V{}", cy.wrapping_add(c_vertex[o[0]].1 as usize))); }
            }
            _ => {
                contours[cy][cx] += c_value[o[0]];
                o.rotate_left(rot.rem_euclid(8) as usize);
                v += 1;
                if o[0] == 0 || o[0] == 4 { paths.push_str(&format!("H{}", cx.wrapping_add(c_vertex[o[0]].0 as usize))); } else { paths.push_str(&format!("V{}", cy.wrapping_add(c_vertex[o[0]].1 as usize))); }
            }
        }
        if cx == x && cy == y && v > 2 {
            break;
        }
    }
    loop {
        contours[cy][cx] += c_value[o[0]];
        if o[0] == viv.2 {
            break;
        }
        o.rotate_left(rot.rem_euclid(8) as usize);
        v += 1;
        if o[0] == 0 || o[0] == 4 { paths.push_str(&format!("H{}", cx.wrapping_add(c_vertex[o[0]].0 as usize))); } else { paths.push_str(&format!("V{}", cy.wrapping_add(c_vertex[o[0]].1 as usize))); }
    }
    if closepaths { paths.push_str("Z"); }
}
