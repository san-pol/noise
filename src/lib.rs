#![allow(non_snake_case)] //suppresses some spammy compiler warnings

pub mod noise{

    pub fn hash(seed:u32) -> u32{
        let mut h = seed;
        h = h.wrapping_sub(h << 6);
        h = h^(h >> 17);
        h = h.wrapping_sub(h << 9);
        h = h^(h << 4);
        h = h.wrapping_sub(h << 3);
        h = h^(h << 10);
        h = h^(h >> 15);
        return h;
    }

    pub struct Noise2D{
        pub nx:usize,
        pub ny:usize,
        pub data:Vec<f32>,
    }

    impl Noise2D{
        pub fn new(nx: usize, ny:usize) -> Self{
            let vec_init = vec![0f32; nx*ny];
            Self{
                nx,
                ny,
                data: vec_init,
            }
        }

        pub fn white_noise(nx: usize, ny:usize, seed:u32) -> Self{
            let vec_init: Vec<f32> = Noise2D::gen_white_noise_vec(nx*ny, seed);
            Self{
                nx,
                ny,
                data: vec_init,
            }
        }

        fn gen_white_noise_vec(n:usize, seed:u32) -> Vec<f32>{
            let mut noise_vec: Vec<f32> = Vec::with_capacity(n);
            let mut prev_hash = seed;
            for _i in 0..n{
                let new_hash = hash(prev_hash);
                noise_vec.push(new_hash as f32 / u32::MAX as f32);
                prev_hash = new_hash;
            }
            return noise_vec;
        }

        pub fn get(&self, i:usize, j:usize) -> f32{
            return self.data[i + j*self.nx];
        }

        pub fn set(&mut self, i:usize, j:usize, v:f32){
            self.data[i + j*self.nx] = v;
        }
    }

    pub fn make_white_noise_img(nx:usize, ny:usize, seed:u32, path:String){
        use image::ImageBuffer;
        let mut imgbuf = ImageBuffer::new(nx as u32, ny as u32);
        let noise2d = Noise2D::white_noise(nx, ny, seed);

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let s = (256f32 * noise2d.get(x as usize, y as usize)) as u8;
            *pixel = image::Rgb([s, s, s]);
        }
        // Save the image as “fractal.png”, the format is deduced from the path
        imgbuf.save(path).unwrap();
    }

    pub struct NoiseOctaveCollection{
        pub noises:Vec<Noise2D>
    }

    impl NoiseOctaveCollection{

        pub fn fractal_noise(n_octaves: u32, seed:u32) -> Self{
            let mut noise_coll: Vec<Noise2D> = Vec::new();
            for i in 0..n_octaves{
                let oct_dim = 2usize.pow(i) + 1;
                noise_coll.push(Noise2D::white_noise(oct_dim, oct_dim, hash(seed) + i));
            }
            Self{
                noises: noise_coll,
            }
        }

        pub fn interpolate(i: f32, j:f32, lox_loy: f32, 
                        lox_hiy: f32, hix_loy: f32, hix_hiy: f32) -> f32{
            assert!(i >= 0f32); assert!(i <= 1f32);
            assert!(j >= 0f32); assert!(j <= 1f32);
            return (1f32-i)*(1f32-j)*lox_loy + (1f32-i)*j*lox_hiy
                + i*(1f32-j)*hix_loy + i*j*hix_hiy;
        }

        pub fn sample(&self, i:usize, j:usize) -> Vec<f32>{
            let mut components: Vec<f32> = Vec::new();
            let max_dim = 2usize.pow((self.noises.len() - 1) as u32);
            assert!(i <= max_dim); assert!(j <= max_dim);
            for octave in &self.noises{
                let n_side = octave.nx - 1;
                let sz = max_dim / n_side;
                let ni = i / sz;
                let nj = j / sz;
                let ri = (i % sz) as f32 / sz as f32;
                let rj = (j % sz) as f32 / sz as f32;
                use std::cmp::min;
                let lox_loy = octave.get(ni, nj);
                let hix_loy = octave.get(min(ni + 1, n_side), nj);
                let lox_hiy = octave.get(ni, min(nj + 1, n_side));
                let hix_hiy = octave.get(min(ni + 1, n_side),
                                min(nj + 1, n_side));
                let sample_value = NoiseOctaveCollection::interpolate(ri, rj,
                                lox_loy, lox_hiy, hix_loy, hix_hiy);
                components.push(sample_value);
            }
            return components;
        }

        pub fn create_fractal_map(&self, base_amp: f32, persi: f32) -> Noise2D{
            let max_dim = 2usize.pow((self.noises.len() - 1) as u32) + 1;
            let mut fractal_map: Noise2D = Noise2D::new(max_dim, max_dim);
            for i in 0..max_dim{
                for j in 0..max_dim{
                    let comp_sample = self.sample(i, j);
                    let mut sum = 0f32;
                    let mut amp = base_amp;
                    for comp in comp_sample{
                        sum += comp * amp;
                        amp = persi * amp;
                    }
                    fractal_map.set(i, j, sum);
                }
            }
            return fractal_map;
        }
    }

    pub fn make_fractal_noise_img(n_octaves:u32, seed:u32, path:String){
        use image::ImageBuffer;
        let max_dim = 2usize.pow((n_octaves - 1) as u32) + 1;
        let mut imgbuf = ImageBuffer::new(max_dim as u32, max_dim as u32);
        let noise_coll = NoiseOctaveCollection::fractal_noise(n_octaves, seed);
        let fractal_map = noise_coll.create_fractal_map(0.5, 0.5);

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let s = (256f32 * fractal_map.get(x as usize, y as usize)) as u8;
            *pixel = image::Rgb([s, s, s]);
        }
        // Save the image as “fractal.png”, the format is deduced from the path
        imgbuf.save(path).unwrap();
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_hash_collisions(){
        use crate::noise::hash;
        let mut hashes = Vec::new();
        let first_hash = hash(0);
        hashes.push(first_hash);
        for i in 1..1_000{
            let new_hash = hash(i);
            hashes.push(new_hash);
            for j in 0..i-1{
                //println!("At i {} and j {}", i, j);
                assert_ne!(hashes[i as usize], hashes[j as usize]);
            }
        }
    }

    #[test]
    fn test_hash_speed(){
        //only opt-level = 0 or 1 produce a non-0 time result
        //about 100 million hashes per second on AMD RX 3600
        use crate::noise::hash;
        use std::time::Instant;
        let before = Instant::now();
        let n = 10_000_000u64;
        let mut stored_hash = 0u32;
        for _i in 0..n{
            stored_hash = hash(stored_hash);
        }
        let t_elapsed = before.elapsed();
        println!("Running {} hashes took {} seconds.", n, t_elapsed.as_secs());
    }

    #[test]
    fn create_white_noise_img(){
        use crate::noise::make_white_noise_img;
        make_white_noise_img(100, 100, 1, "white_noise.png".to_string());
    }

    #[test]
    fn create_fractal_noise_img(){
        use crate::noise::make_fractal_noise_img;
        make_fractal_noise_img(11, 4, "fractal_noise.png".to_string());
    }
}
