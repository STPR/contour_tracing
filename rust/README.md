# Contour tracing library [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url]

A 2D library to trace contours.  
How it works in a live demo: https://stpr.github.io/contour_tracing/

## Features

Core features:
- Trace contours using the Theo Pavlidis' algorithm (connectivity: 4-connected)
- Trace **outlines** in **clockwise direction**
- Trace **holes** in **counterclockwise direction**
- Input format: a 2D array of bits or an image buffer
- Output format: a string of SVG Path commands

Manual parameters:
- User can specify to close or not the paths (with the SVG Path **Z** command)

## An example with an array of bits

1. Add the following line to your **Cargo.toml** file in the **dependencies** section:
```
contour_tracing = { version = "*", features = ["array"] }
```

2. Then use the library:
```rust
use contour_tracing::array::bits_to_paths;

fn main() {
    let bits = vec![vec![ 1,0,0 ],
                    vec![ 0,1,0 ],
                    vec![ 0,0,1 ]];

    println!("{}", bits_to_paths(bits, true));
}
```

## An example with an image buffer

1. Add the following line to your **Cargo.toml** file in the **dependencies** section:
```
contour_tracing = { version = "*", features = ["image"] }
```

2. Then use the library:
```rust
use image::{GrayImage, Luma};
use contour_tracing::image::single_l8_to_paths;

fn main() {
    let mut image_buffer = GrayImage::new(3, 3);
    let foreground_color: image::Luma<u8> = Luma([1]);

    image_buffer.put_pixel(0, 0, foreground_color);
    image_buffer.put_pixel(1, 1, foreground_color);
    image_buffer.put_pixel(2, 2, foreground_color);

    println!("{}", single_l8_to_paths(&mut image_buffer, foreground_color, true));
}
```

Both examples should print: `M0 0H1V1H0ZM1 1H2V2H1ZM2 2H3V3H2Z`

For more examples, have a look at the documentation: [![Documentation][documentation-img]][documentation-url]

## License

Contour tracing library
https://github.com/STPR/contour_tracing

Copyright (c) 2022, STPR - https://github.com/STPR

SPDX-License-Identifier: EUPL-1.2

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.txt](LICENSE.txt).

[package-img]: https://img.shields.io/crates/v/contour_tracing.svg
[package-url]: https://crates.io/crates/contour_tracing
[documentation-img]: https://docs.rs/contour_tracing/badge.svg
[documentation-url]: https://docs.rs/contour_tracing
