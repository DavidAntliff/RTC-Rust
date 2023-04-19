// Improved Perlin Noise, Ken Perlin
// Based on http://adrianb.io/2014/08/09/perlinnoise.html
// Original paper: https://mrl.cs.nyu.edu/~perlin/paper445.pdf
// Original code: https://mrl.cs.nyu.edu/~perlin/noise/
// C++ implementation:
// https://github.com/DavidAntliff/RTC-CPP/blob/main/src/lib/include/ray_tracer_challenge/perlin_noise.h

// Hash lookup table as defined by Ken Perlin. This is a randomly
// arranged array of all numbers from 0-255 inclusive, repeated once.
#[rustfmt::skip]
static PERMUTATION: [usize; 512] = [
        151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233,
        7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23,
        190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
        203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174,
        20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27,
        166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
        220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25,
        63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169,
        200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
        186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118,
        126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182,
        189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
        70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19,
        98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246,
        97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
        241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181,
        199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150,
        254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128,
        195, 78, 66, 215, 61, 156, 180,
        /* repeat */
        151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233,
        7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23,
        190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
        203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174,
        20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27,
        166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
        220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25,
        63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169,
        200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
        186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118,
        126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182,
        189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
        70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19,
        98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246,
        97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
        241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181,
        199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150,
        254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128,
        195, 78, 66, 215, 61, 156, 180
];

pub fn grad(hash: usize, x: f64, y: f64, z: f64) -> f64 {
    // Take the hashed value and take the first 4 bits of it (15 == 0b1111)
    let h = hash & 0b1111;

    // If the most significant bit (MSB) of the hash is 0 then set u = x. Otherwise y.
    let u = if h < 0b1000 { x } else { y };

    // In Ken Perlin's original implementation this was another conditional operator (?:).
    // Expand it for readability.
    let v: f64;

    if h < 0b0100 {
        // If the first and second significant bits are 0 set v = y
        v = y;
    } else if h == 0b1100 || h == 0b1110 {
        // If the first and second significant bits are 1 set v = x
        v = x;
    } else {
        // If the first and second significant bits are not equal (0/1, 1/0) set v = z
        v = z;
    }

    // Use the last 2 bits to decide if u and v are positive or negative. Then return their addition.
    let m = if (h & 1) == 0 { u } else { -u };
    let n = if (h & 2) == 0 { v } else { -v };
    m + n
}

fn fade(t: f64) -> f64 {
    // Fade function as defined by Ken Perlin.  This eases coordinate values
    // so that they will "ease" towards integral values.  This ends up smoothing
    // the final output.
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0) // 6t^5 - 15t^4 + 10t^3
}

fn lerp(a: f64, b: f64, x: f64) -> f64 {
    a + x * (b - a)
}

pub fn perlin(x: f64, y: f64, z: f64) -> f64 {
    perlin_impl(x, y, z, 0)
}

pub fn perlin_with_repeat(x: f64, y: f64, z: f64, repeat: i32) -> f64 {
    perlin_impl(x, y, z, repeat)
}

#[rustfmt::skip]
fn perlin_impl(x: f64, y: f64, z: f64, repeat: i32) -> f64 {
    let mut x = x;
    let mut y = y;
    let mut z = z;

    // If we have any repeat, change the coordinates to their "local" repetitions
    if repeat > 0 {
        x %= repeat as f64;
        y %= repeat as f64;
        z %= repeat as f64;
    }

    // Calculate the "unit cube" that the point asked will be located in
    // The left bound is ( |_x_|,|_y_|,|_z_| ) and the right bound is that
    // plus 1.  Next we calculate the location (from 0.0 to 1.0) in that cube.
    let xi: usize = (x.floor() as i32 & 255) as usize;
    let yi: usize = (y.floor() as i32 & 255) as usize;
    let zi: usize = (z.floor() as i32 & 255) as usize;

    // We also fade the location to smooth the result.
    let xf = x - x.floor();
    let yf= y - y.floor();
    let zf = z - z.floor();

    let u = fade(xf);
    let v = fade(yf);
    let w = fade(zf);

    let p = &PERMUTATION;

    let inc = |mut num: usize| -> usize {
        num += 1;
        if repeat > 0 {
            num %= repeat as usize;
        }
        num
    };

    let aaa = p[(p[(p[    xi ] +     yi)]  +     zi)];
    let aba = p[(p[(p[    xi ] + inc(yi))] +     zi)];
    let aab = p[(p[(p[    xi ] +     yi)]  + inc(zi))];
    let abb = p[(p[(p[    xi ] + inc(yi))] + inc(zi))];
    let baa = p[(p[(p[inc(xi)] +     yi)]  +     zi)];
    let bba = p[(p[(p[inc(xi)] + inc(yi))] +     zi)];
    let bab = p[(p[(p[inc(xi)] +     yi)]  + inc(zi))];
    let bbb = p[(p[(p[inc(xi)] + inc(yi))] + inc(zi))];

    // The gradient function calculates the dot product between a pseudorandom
    // gradient vector and the vector from the input coordinate to the 8
    // surrounding points in its unit cube.
    // This is all then lerped together as a sort of weighted average based on the faded (u,v,w)
    // values we made earlier.
    let x1 = lerp(grad (aaa, xf  , yf  , zf),
                       grad (baa, xf-1.0, yf  , zf),
                       u);
    let x2 = lerp(grad (aba, xf  , yf-1.0, zf),
                       grad (bba, xf-1.0, yf-1.0, zf),
                       u);
    let y1 = lerp(x1, x2, v);

    let x1 = lerp(grad (aab, xf  , yf  , zf-1.0),
                       grad (bab, xf-1.0, yf  , zf-1.0),
                       u);
    let x2 = lerp(grad (abb, xf  , yf-1.0, zf-1.0),
                       grad (bbb, xf-1.0, yf-1.0, zf-1.0),
                       u);
    let y2 = lerp (x1, x2, v);

    // For convenience we bound it to 0 - 1 (theoretical min/max before is -1 - 1)
    (lerp(y1, y2, w) + 1.0) / 2.0
}

pub fn octave_perlin(x: f64, y: f64, z: f64, octaves: u32, persistence: f64) -> f64 {
    let mut total = 0.0;
    let mut frequency = 1.0;
    let mut amplitude = 1.0;
    let mut max_value = 0.0; // Used for normalizing result to 0.0 - 1.0

    for _ in 0..octaves {
        total += perlin(x * frequency, y * frequency, z * frequency) * amplitude;
        max_value += amplitude;
        amplitude *= persistence;
        frequency *= 2.0;
    }

    total / max_value
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::canvas;
    use crate::colors::color;

    #[test]
    fn generate_octave_perlin_noise_image() {
        let mut image = canvas(512, 512);
        let coordinate_scale = 4.0;
        let value_scale = 2.4; // use to keep min and max between [0.0, 1.0]
        let mut min_vv = 1.0;
        let mut max_vv = 0.0;
        for x in 0..image.width {
            for y in 0..image.height {
                let dx = coordinate_scale * x as f64 / image.width as f64;
                let dy = coordinate_scale * y as f64 / image.height as f64;
                let v = octave_perlin(dx, dy, 0.0, 8, 0.6);
                let vv = (v - 0.5) * value_scale + 0.5;
                min_vv = f64::min(min_vv, vv);
                max_vv = f64::max(max_vv, vv);
                let pixel = color(vv, vv, vv);
                image.write_pixel(x, y, &pixel);
            }
        }
        image.to_ppm_file("test_perlin_noise.ppm");
        println!("min {}, max {}", min_vv, max_vv);

        assert_eq!(0.5, perlin(0.0, 0.0, 0.0));
    }

    #[test]
    fn debug() {
        let x = -3.3273440166393975;
        let y = 2.1870538633277339;
        let z = -0.38154583713770984;
        let x = octave_perlin(x, y, z, 3, 0.8);
        println!("{}", x);
    }
}
