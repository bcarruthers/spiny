// Adapted for Rust from Stefan Gustavson code
// Simplex noise demystified:
// https://weber.itn.liu.se/~stegu/simplexnoise/simplexnoise.pdf

// Original license:
// sdnoise1234, Simplex noise with true analytic
// derivative in 1D to 4D.
//
// Copyright © 2003-2012, Stefan Gustavson
//
// Contact: stefan.gustavson@gmail.com
//
// This library is public domain software, released by the author
// into the public domain in February 2011. You may do anything
// you like with it. You may even remove all attributions,
// but of course I'd appreciate it if you kept my name somewhere.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// This is an implementation of Perlin "simplex noise" over one
// dimension (x), two dimensions (x,y), three dimensions (x,y,z)
// and four dimensions (x,y,z,w). The analytic derivative is
// returned, to make it possible to do lots of fun stuff like
// flow animations, curl noise, analytic antialiasing and such.
//
// Visually, this noise is exactly the same as the plain version of
// simplex noise provided in the file "snoise1234.c". It just returns
// all partial derivatives in addition to the scalar noise value.

const GRAD3: [[f32; 3]; 12] = [
    [1.0f32, 1.0f32, 0.0f32],
    [-1.0f32, 1.0f32, 0.0f32],
    [1.0f32, -1.0f32, 0.0f32],
    [-1.0f32, -1.0f32, 0.0f32],
    [1.0f32, 0.0f32, 1.0f32],
    [-1.0f32, 0.0f32, 1.0f32],
    [1.0f32, 0.0f32, -1.0f32],
    [-1.0f32, 0.0f32, -1.0f32],
    [0.0f32, 1.0f32, 1.0f32],
    [0.0f32, -1.0f32, 1.0f32],
    [0.0f32, 1.0f32, -1.0f32],
    [0.0f32, -1.0f32, -1.0f32],
];

// Permutation table. This is just a random jumble of all numbers 0-255,
// repeated twice to avoid wrapping the index at 255 for each lookup.
const PERM: [u8; 512] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180, 151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194,
    233, 7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234,
    75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174,
    20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83,
    111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25,
    63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188,
    159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147,
    118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
    213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253,
    19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193,
    238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31,
    181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93,
    222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
];

const SQRT3: f32 = 1.7320508075688772935274463415059f32;
const F2: f32 = 0.5f32 * (SQRT3 - 1.0f32);
const G2: f32 = (3.0f32 - SQRT3) / 6.0f32;

fn dot2(gi: u8, x: f32, y: f32) -> f32 {
    let gi = gi as usize;
    GRAD3[gi][0] * x + GRAD3[gi][1] * y
}

pub fn sample2(x: f32, y: f32) -> f32 {
    //float n0, n1, n2; // Noise contributions from the three corners
    // Skew the input space to determine which simplex cell we're in
    let s = (x + y) * F2; // Hairy factor for 2D

    //int i = Floor(x + s);
    //int j = Floor(y + s);
    let real_i = x + s;
    let real_j = y + s;
    let i = (if real_i > 0.0f32 {
        real_i
    } else {
        real_i - 1.0f32
    }) as i32;
    let j = (if real_j > 0.0f32 {
        real_j
    } else {
        real_j - 1.0f32
    }) as i32;

    let t = (i + j) as f32 * G2;
    let x0 = i as f32 - t; // Unskew the cell origin back to (x,y) space
    let y0 = j as f32 - t;
    let x0 = x - x0; // The x,y distances from the cell origin
    let y0 = y - y0;

    // For the 2D case, the simplex shape is an equilateral triangle.
    // Determine which simplex we are in.
    //let i1, j1 =                // Offsets for second (middle) corner of simplex in (i,j) coords
    //    if (x0 > y0) { 1, 0  // lower triangle, XY order: (0,0)->(1,0)->(1,1)
    //    else 0, 1               // upper triangle, YX order: (0,0)->(0,1)->(1,1)
    let i1 = if x0 > y0 { 1 } else { 0 };
    let j1 = 1 - i1;

    // A step of (1,0) in (i,j) means a step of (1-c,-c) in (x,y), and
    // a step of (0,1) in (i,j) means a step of (-c,1-c) in (x,y), where
    // c = (3-sqrt(3))/6
    let x1 = x0 - i1 as f32 + G2; // Offsets for middle corner in (x,y) unskewed coords
    let y1 = y0 - j1 as f32 + G2;
    let x2 = x0 - 1.0f32 + 2.0f32 * G2; // Offsets for last corner in (x,y) unskewed coords
    let y2 = y0 - 1.0f32 + 2.0f32 * G2;
    // Work out the hashed gradient indices of the three simplex corners
    let ii = (i & 255) as usize;
    let jj = (j & 255) as usize;

    let gi0 = PERM[ii + PERM[jj] as usize] % 12;
    let gi1 = PERM[ii + i1 + PERM[jj + j1] as usize] % 12;
    let gi2 = PERM[ii + 1 + PERM[jj + 1] as usize] % 12;

    //(n * (n * n * 15731 + 789221) + 1376312589)

    // Calculate the contribution from the three corners
    let t0 = 0.5f32 - x0 * x0 - y0 * y0;
    let n0 = if t0 < 0.0f32 {
        0.0f32
    } else {
        let t02 = t0 * t0;
        t02 * t02 * dot2(gi0, x0, y0) // (x,y) of grad3 used for 2D gradient
    };

    let t1 = 0.5f32 - x1 * x1 - y1 * y1;
    let n1 = if t1 < 0.0f32 {
        0.0f32
    } else {
        let t12 = t1 * t1;
        t12 * t12 * dot2(gi1, x1, y1)
    };

    let t2 = 0.5f32 - x2 * x2 - y2 * y2;
    let n2 = if t2 < 0.0f32 {
        0.0f32
    } else {
        let t22 = t2 * t2;
        t22 * t22 * dot2(gi2, x2, y2)
    };

    // Add contributions from each corner to get the final noise value.
    // The result is scaled to return values in the interval [-1,1].
    70.0f32 * (n0 + n1 + n2)
}

const F3: f32 = 1.0f32 / 3.0f32;
const G3: f32 = 1.0f32 / 6.0f32; // Very nice and simple unskew factor, too

fn dot3(gi: u8, x: f32, y: f32, z: f32) -> f32 {
    let gi = gi as usize;
    GRAD3[gi][0] * x + GRAD3[gi][1] * y + GRAD3[gi][2] * z
}

pub fn sample3(x: f32, y: f32, z: f32) -> f32 {
    // Skew the input space to determine which simplex cell we're in
    let s = (x + y + z) * F3; // Very nice and simple skew factor for 3D
    let i = (x + s).floor() as i32;
    let j = (y + s).floor() as i32;
    let k = (z + s).floor() as i32;
    let t = (i + j + k) as f32 * G3;
    let x0 = i as f32 - t; // Unskew the cell origin back to (x,y,z) space
    let y0 = j as f32 - t;
    let z0 = k as f32 - t;
    let x0 = x - x0; // The x,y,z distances from the cell origin
    let y0 = y - y0;
    let z0 = z - z0;
    // For the 3D case, the simplex shape is a slightly irregular tetrahedron.
    // Determine which simplex we are in.
    // Offsets for second corner of simplex in (i,j,k) coords
    // Offsets for third corner of simplex in (i,j,k) coords
    let (i1, j1, k1, i2, j2, k2) = if x0 >= y0 {
        if y0 >= z0 {
            (1, 0, 0, 1, 1, 0)
        }
        // X Y Z order
        else if x0 >= z0 {
            (1, 0, 0, 1, 0, 1)
        }
        // X Z Y order
        else {
            (0, 0, 1, 1, 0, 1)
        } // Z X Y order
    } else {
        if y0 < z0 {
            (0, 0, 1, 0, 1, 1)
        }
        // Z Y X order
        else if x0 < z0 {
            (0, 1, 0, 0, 1, 1)
        }
        // Y Z X order
        else {
            (0, 1, 0, 1, 1, 0)
        } // Y X Z order
    };

    // A step of (1,0,0) in (i,j,k) means a step of (1-c,-c,-c) in (x,y,z),
    // a step of (0,1,0) in (i,j,k) means a step of (-c,1-c,-c) in (x,y,z), and
    // a step of (0,0,1) in (i,j,k) means a step of (-c,-c,1-c) in (x,y,z), where
    // c = 1/6.
    let x1 = x0 - i1 as f32 + G3; // Offsets for second corner in (x,y,z) coords
    let y1 = y0 - j1 as f32 + G3;
    let z1 = z0 - k1 as f32 + G3;
    let x2 = x0 - i2 as f32 + 2.0f32 * G3; // Offsets for third corner in (x,y,z) coords
    let y2 = y0 - j2 as f32 + 2.0f32 * G3;
    let z2 = z0 - k2 as f32 + 2.0f32 * G3;
    let x3 = x0 - 1.0f32 + 3.0f32 * G3; // Offsets for last corner in (x,y,z) coords
    let y3 = y0 - 1.0f32 + 3.0f32 * G3;
    let z3 = z0 - 1.0f32 + 3.0f32 * G3;
    // Work out the hashed gradient indices of the four simplex corners
    let ii = (i & 255) as usize;
    let jj = (j & 255) as usize;
    let kk = (k & 255) as usize;
    let gi0 = PERM[ii + PERM[jj + PERM[kk] as usize] as usize] % 12;
    let gi1 = PERM[ii + i1 + PERM[jj + j1 + PERM[kk + k1] as usize] as usize] % 12;
    let gi2 = PERM[ii + i2 + PERM[jj + j2 + PERM[kk + k2] as usize] as usize] % 12;
    let gi3 = PERM[ii + 1 + PERM[jj + 1 + PERM[kk + 1] as usize] as usize] % 12;
    // Calculate the contribution from the four corners
    let t0 = 0.6f32 - x0 * x0 - y0 * y0 - z0 * z0;
    let n0 = if t0 < 0.0f32 {
        0.0f32
    } else {
        let t02 = t0 * t0;
        t02 * t02 * dot3(gi0, x0, y0, z0)
    };

    let t1 = 0.6f32 - x1 * x1 - y1 * y1 - z1 * z1;
    let n1 = if t1 < 0.0f32 {
        0.0f32
    } else {
        let t12 = t1 * t1;
        t12 * t12 * dot3(gi1, x1, y1, z1)
    };

    let t2 = 0.6f32 - x2 * x2 - y2 * y2 - z2 * z2;
    let n2 = if t2 < 0.0f32 {
        0.0f32
    } else {
        let t22 = t2 * t2;
        t22 * t22 * dot3(gi2, x2, y2, z2)
    };

    let t3 = 0.6f32 - x3 * x3 - y3 * y3 - z3 * z3;
    let n3 = if t3 < 0.0f32 {
        0.0f32
    } else {
        let t32 = t3 * t3;
        t32 * t32 * dot3(gi3, x3, y3, z3)
    };

    // Add contributions from each corner to get the final noise value.
    // The result is scaled to stay just inside [-1,1]
    32.0f32 * (n0 + n1 + n2 + n3)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_simplex_noise() {
        assert_eq!(super::sample3(0.0, 0.0, 0.0), 0.0);
        assert_eq!(super::sample3(10.0, 0.0, 0.0), -0.6522211);
        assert_eq!(super::sample3(10.0, 10.0, 0.0), 0.76009965);
        assert_eq!(super::sample3(10.0, 10.0, 20.0), -0.10788002);
    }

    #[test]
    fn test_noise_perf() {
        use std::time::Instant;
        let start = Instant::now();
        for _ in 0..1 {
            for z in 0..10 {
                for y in 0..100 {
                    for x in 0..100 {
                        super::sample3(x as f32, y as f32, z as f32);
                    }
                }
            }
        }
        let duration = start.elapsed();
        println!("Elapsed: {:?}", duration);
    }
}