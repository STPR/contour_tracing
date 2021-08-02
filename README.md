# Contour tracing library

A 2D library to trace contours.  
How it works in a live demo: https://stpr.github.io/contour_tracing/

## Features

Core features:
- Trace contours using the Theo Pavlidis' algorithm (connectivity: 4-connected)
- Trace **outlines** in **clockwise direction**
- Trace **holes** in **counterclockwise direction**
- Input format: a 2D array of bits
- Output format: a string of SVG Path commands

Manual parameters:
- User can specify to close or not the paths (with the SVG Path **Z** command)

## Examples

Rust language:
```rust
extern crate contour_tracing;
use contour_tracing::bits_to_paths;

let bits = vec![vec![ 1,0,0 ],
                vec![ 0,1,0 ],
                vec![ 0,0,1 ]];

println!("{}", bits_to_paths(bits, true));
```
Should print: `M0 0H1V1H0ZM1 1H2V2H1ZM2 2H3V3H2Z`

For more Rust examples, have a look at the documentation: [![Documentation][documentation-img]][documentation-url]

## License

Contour tracing library
https://github.com/STPR/contour_tracing

Copyright (c) 2021, STPR - https://github.com/STPR

SPDX-License-Identifier: EUPL-1.2

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.txt](LICENSE.txt).

[documentation-img]: https://docs.rs/contour_tracing/badge.svg
[documentation-url]: https://docs.rs/contour_tracing
