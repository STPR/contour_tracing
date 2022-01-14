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
//! - Input format: a 2D array of bits or an image buffer
//! - Output format: a string of SVG Path commands
//!
//! Manual parameters:
//! - User can specify to close or not the paths (with the SVG Path **Z** command)
//! 
//! # Examples
//! Have a look at the different functions below.

#![cfg_attr(docsrs, feature(doc_cfg))]

const MN: [(i8, i8); 8] = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)]; // Moore neighborhood

/*
 contours: an array of contours
 buffer: an image buffer
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

#[cfg(feature = "array")]
pub mod array;

#[cfg(feature = "image")]
#[cfg_attr(docsrs, doc(cfg(feature = "image")))]
pub mod image;
