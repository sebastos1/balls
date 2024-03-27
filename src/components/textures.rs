use crate::*;
use bevy::{
    render::{
        texture::ImageSampler,
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

pub fn grainy_texture(color: Color, height: usize, width: usize) -> Image {
    let mut rng = StdRng::seed_from_u64(636345654);
    let mut texture_data = vec![0; height * width * 4];

    for y in 0..height {
        for x in 0..width {
            let index = (y * width + x) * 4;
            let random_factor: f32 = rng.gen_range(0.1..=0.9);

            texture_data[index] = (color.r() * random_factor * 255.0) as u8;
            texture_data[index + 1] = (color.g() * random_factor * 255.0) as u8;
            texture_data[index + 2] = (color.b() * random_factor * 255.0) as u8;
            texture_data[index + 3] = 255;
        }
    }

    let mut image = Image::new_fill(
        Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::nearest();
    image
}

pub fn noisy_texture(color: Color) -> Image {
    const TEXTURE_SIZE: usize = 256;
    let perlin = Perlin::new(4231432142);
    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];

    for y in 0..TEXTURE_SIZE {
        for x in 0..TEXTURE_SIZE {
            let index = (y * TEXTURE_SIZE + x) * 4;

            let noise_scale = 0.05;
            let noise = perlin.get([x as f64 * noise_scale, y as f64 * noise_scale, 0.0]);
            let normalized_noise = noise * 0.5 + 0.5;
            let dampened_noise = (normalized_noise * 0.5 + 0.25) as f32;

            texture_data[index] = (color.r() * dampened_noise * 255.0) as u8;
            texture_data[index + 1] = (color.g() * dampened_noise * 255.0) as u8;
            texture_data[index + 2] = (color.b() * dampened_noise * 255.0) as u8;
            texture_data[index + 3] = 255;
        }
    }

    let mut image = Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::nearest();
    image
}

pub fn striped_texture(stripe_color: Color) -> Image {
    const TEXTURE_SIZE: usize = 4;
    const STRIPE_THICKNESS: usize = 2;

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];

    for y in 0..TEXTURE_SIZE {
        for x in 0..TEXTURE_SIZE {
            let index = (y * TEXTURE_SIZE + x) * 4;

            let is_stripe = y >= TEXTURE_SIZE / 2 - STRIPE_THICKNESS / 2
                && y < TEXTURE_SIZE / 2 + STRIPE_THICKNESS / 2;

            let color = if is_stripe {
                stripe_color
            } else {
                Color::WHITE
            };

            texture_data[index] = (color.r() * 255.0) as u8;
            texture_data[index + 1] = (color.g() * 255.0) as u8;
            texture_data[index + 2] = (color.b() * 255.0) as u8;
            texture_data[index + 3] = 255;
        }
    }

    let mut image = Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::nearest();
    image
}
