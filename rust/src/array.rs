/*
 * Contour tracing library
 * https://github.com/STPR/contour_tracing
 *
 * Copyright (c) 2022, STPR - https://github.com/STPR
 *
 * SPDX-License-Identifier: EUPL-1.2
 */

const O_VERTEX_WITH_BORDER: [(i8, i8); 7] = [(-1, 0), (0, 0), (-1, -1), (0, 0), (0, -1), (0, 0), (0, 0)]; // Bottom left coordinates with a border
const H_VERTEX_WITH_BORDER: [(i8, i8); 7] = [(0, 0), (0, 0), (-1, 0), (0, 0), (-1, -1), (0, 0), (0, -1)]; // Bottom right coordinates with a border
const O_VALUE_FOR_SIGNED:   [i8; 7]       = [1, 0, 2, 0, 4, 0, 8];     // Value to add into an array of contours (using signed integers)
const H_VALUE_FOR_SIGNED:   [i8; 7]       = [-4, 0, -8, 0, -1, 0, -2]; // (idem)

/// A function that takes a 2D array of bits and an option as input and return a string of SVG Path commands as output.
/// # Examples
/// ```ignore
/// use contour_tracing::array::bits_to_paths;
/// ```
/// - A simple example with the **closepaths option** set to **false**:
///
/// ```edition2018
/// # use contour_tracing::array::bits_to_paths;
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
/// ```edition2018
/// # use contour_tracing::array::bits_to_paths;
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
/// ```edition2018
/// # use contour_tracing::array::bits_to_paths;
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
    for r in 0..=rows - 1_usize {
        for c in 0..=cols - 1_usize {
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
        neighbors = [
            contours[tracer_y - 1][tracer_x    ],
            contours[tracer_y - 1][tracer_x + 1],
            contours[tracer_y    ][tracer_x + 1],
            contours[tracer_y + 1][tracer_x + 1],
            contours[tracer_y + 1][tracer_x    ],
            contours[tracer_y + 1][tracer_x - 1],
            contours[tracer_y    ][tracer_x - 1],
            contours[tracer_y - 1][tracer_x - 1]
        ];
        rn =
            if outline {
                if      neighbors[o[7]] > 0 && neighbors[o[0]] > 0 { 1 }
                else if neighbors[o[0]] > 0                        { 2 }
                else if neighbors[o[1]] > 0 && neighbors[o[2]] > 0 { 3 }
                else { 0 }
            }
            else if neighbors[o[1]] < 0 && neighbors[o[0]] < 0 { 1 }
            else if neighbors[o[0]] < 0                        { 2 }
            else if neighbors[o[7]] < 0 && neighbors[o[6]] < 0 { 3 }
            else { 0 };
        match rn {
            1 => {
                contours[tracer_y][tracer_x] += value[o[0]];
                tracer_x = tracer_x.wrapping_add(super::MN[o[viv.0]].0 as usize);
                tracer_y = tracer_y.wrapping_add(super::MN[o[viv.0]].1 as usize);
                o.rotate_right(rot.rem_euclid(8) as usize); // Rotate 90 degrees, counterclockwise for the outlines (rot = 2) or clockwise for the holes (rot = -2)
                vertices_nbr += 1;
                if o[0] == 0 || o[0] == 4 { paths.push_str(&format!("H{}", tracer_x.wrapping_add(vertex[o[0]].0 as usize))); } else { paths.push_str(&format!("V{}", tracer_y.wrapping_add(vertex[o[0]].1 as usize))); }
            }
            2 => {
                contours[tracer_y][tracer_x] += value[o[0]];
                tracer_x = tracer_x.wrapping_add(super::MN[o[0]].0 as usize);
                tracer_y = tracer_y.wrapping_add(super::MN[o[0]].1 as usize);
            }
            3 => {
                contours[tracer_y][tracer_x] += value[o[0]];
                o.rotate_left(rot.rem_euclid(8) as usize); // Rotate 90 degrees, clockwise for the outlines (rot = 2) or counterclockwise for the holes (rot = -2)
                contours[tracer_y][tracer_x] += value[o[0]];
                vertices_nbr += 1;
                if o[0] == 0 || o[0] == 4 { paths.push_str(&format!("H{}", tracer_x.wrapping_add(vertex[o[0]].0 as usize))); } else { paths.push_str(&format!("V{}", tracer_y.wrapping_add(vertex[o[0]].1 as usize))); }
                o.rotate_right(rot.rem_euclid(8) as usize);
                tracer_x = tracer_x.wrapping_add(super::MN[o[viv.1]].0 as usize);
                tracer_y = tracer_y.wrapping_add(super::MN[o[viv.1]].1 as usize);
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
    if closepaths { paths.push('Z'); }
}
