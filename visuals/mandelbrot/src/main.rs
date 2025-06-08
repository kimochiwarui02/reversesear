use image::{Rgb, RgbImage};
use itertools::Itertools;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
const ITER: u32 = 1000;

fn main() {
    let xmax = 25600;
    let ymax = 14400;
    let img = Arc::new(Mutex::new(RgbImage::new(xmax, ymax)));

    let brot_x_min: f64 = -2.00;
    let brot_x_max: f64 = 0.47;
    let brot_y_min: f64 = -1.12;
    let brot_y_max: f64 = 1.12;
    let x_scale = (brot_x_max - brot_x_min) + brot_x_min;
    let y_scale = (brot_y_max - brot_y_min) + brot_y_min;

    let palette = [
        Rgb([66, 30, 15]),    // Deep brown
        Rgb([45, 20, 10]),    // Darker brown (new)
        Rgb([25, 7, 26]),     // Deep purple
        Rgb([17, 4, 36]),     // Darker purple (new)
        Rgb([9, 1, 47]),      // Dark navy
        Rgb([6, 3, 60]),      // Deeper navy (new)
        Rgb([4, 4, 73]),      // Dark blue
        Rgb([2, 5, 86]),      // Slightly lighter dark blue (new)
        Rgb([0, 7, 100]),     // Deep blue
        Rgb([6, 25, 119]),    // Blue transition (new)
        Rgb([12, 44, 138]),   // Medium blue
        Rgb([18, 63, 157]),   // Brighter medium blue (new)
        Rgb([24, 82, 177]),   // Bright blue
        Rgb([40, 103, 193]),  // Sky blue (new)
        Rgb([57, 125, 209]),  // Light blue
        Rgb([96, 153, 221]),  // Pale blue (new)
        Rgb([134, 181, 229]), // Very light blue
        Rgb([172, 208, 238]), // Near-white blue (new)
        Rgb([211, 236, 248]), // Off-white blue
        Rgb([226, 235, 220]), // Pale cream (new)
        Rgb([241, 233, 191]), // Light yellow
        Rgb([248, 201, 95]),  // Bright yellow
        Rgb([255, 170, 0]),   // Orange
        Rgb([178, 99, 0]),    // Deep orange (new)
        Rgb([153, 87, 0]),    // Burnt orange
        Rgb([106, 52, 3]),    // Dark orange-brown
    ];
    let palette_size = palette.len();
    // Scale x and y to be within the set

    (0..ymax).into_par_iter().for_each(|y| {
        for x in 0..xmax{
            let dx: f64 = (x as f64 / (xmax - 1) as f64) * x_scale;
            let dy: f64 = (y as f64 / (ymax - 1) as f64) * y_scale;

            let mut px: f64 = 0.0;
            let mut py: f64 = 0.0;
            let mut px2: f64 = 0.0;
            let mut py2: f64 = 0.0;
            let mut iteration = 0;

            while (px2 + py2 <= 4.0) && (iteration < ITER) {
                py = 2.0 * px * py + dy;
                px = px2 - py2 + dx;
                px2 = px * px;
                py2 = py * py;

                iteration += 1
            }
            img.lock().unwrap().put_pixel(x, y, {
                if iteration == ITER {
                    Rgb([0, 0, 0])
                } else {
                    palette[iteration as usize % palette_size]
                }
            });
        }
    });

    img.lock().unwrap().save("output.png").unwrap(); // Example: save the image
}
