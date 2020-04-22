use super::texture::Texture;
use super::utils::color::Color;
use super::utils::vec3::Vec3;

use rand::Rng;

const POINT_COUNT: usize = 256;

struct Perlin {
    rnd: Vec<Vec3>,
    x: Vec<usize>,
    y: Vec<usize>,
    z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        Self {
            rnd: (0..POINT_COUNT)
                .map(|_| Vec3::random_range(-1.0, 1.0))
                .collect(),
            x: Self::generate_permutation(),
            y: Self::generate_permutation(),
            z: Self::generate_permutation(),
        }
    }

    pub fn turbulence(&self, point: &Vec3, depth: usize) -> f64 {
        let mut acc = 0.0;
        let mut tmp_point = *point;
        let mut weight = 1.0;

        for _ in 0..depth {
            acc += weight * self.noise(&tmp_point);
            weight *= 0.5;
            tmp_point = tmp_point * 2.0;
        }

        acc.abs()
    }

    pub fn noise(&self, point: &Vec3) -> f64 {
        let i = point.x.floor() as usize;
        let j = point.y.floor() as usize;
        let k = point.z.floor() as usize;

        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();

        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for ii in 0..2 {
            for jj in 0..2 {
                for kk in 0..2 {
                    c[ii][jj][kk] = 
                        self.rnd[self.x[(i + ii) & 255] ^ self.y[(j + jj) & 255] ^ self.z[(k + kk) & 255]];
                }
            }
        }

        Self::perlin_interpolation(&c, u, v, w)
    }

    fn perlin_interpolation(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut acc = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    acc += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                         * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                         * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                         * Vec3::dot_product(c[i][j][k], weight);
                }
            }
        }

        acc
    }

    fn generate_permutation() -> Vec<usize> {
        let mut perm: Vec<usize> = (0..POINT_COUNT).map(|i|{ i }).collect();

        Self::permutate(&mut perm);

        perm
    }

    fn permutate(arr: &mut Vec<usize>) {
        let mut rng = rand::thread_rng();
        for i in (1..POINT_COUNT).rev() {
            let j = rng.gen_range(0, i);
            let tmp = arr[j];
            arr[j] = arr[i];
            arr[i] = tmp;
        }
    }
}

pub struct NoiseTexture {
    perlin: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            perlin: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn color(&self, _u: f64, _v: f64, point: &Vec3) -> Color {
        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (self.scale * point.z + self.perlin.turbulence(point, 7) * 10.0).sin())
    }
}