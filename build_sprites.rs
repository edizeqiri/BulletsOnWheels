// build.rs - fixed sprite extraction using flood fill approach - Claude VIBE CODED
use std::error::Error;
use std::fs;
use std::path::Path;
use std::collections::HashSet;

use image::{open, GenericImageView};

fn main() -> Result<(), Box<dyn Error>> {
    // Input sprite sheet (put your sheet in assets/)
    let input_path = "assets/sprites.png";
    let output_dir = "assets/extracted";

    // Skip if no input file exists (avoids build failure if missing)
    if !Path::new(input_path).exists() {
        println!("cargo:warning=No sprite sheet found at {}", input_path);
        return Ok(());
    }

    dump_sprites_to_assets(input_path, output_dir)?;
    println!("cargo:rerun-if-changed={}", input_path); // rebuild if sheet changes
    Ok(())
}

#[derive(Debug)]
struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

fn dump_sprites_to_assets(input_path: &str, output_dir: &str) -> Result<(), Box<dyn Error>> {
    let img = open(input_path)?.to_rgba8();
    let (width, height) = img.dimensions();

    let mut visited = HashSet::new();
    let mut sprites = Vec::new();

    // Scan through the image looking for unvisited non-transparent pixels
    for y in 0..height {
        for x in 0..width {
            if visited.contains(&(x, y)) {
                continue;
            }

            let pixel = img.get_pixel(x, y);
            if pixel[3] > 0 { // Non-transparent pixel found
                // Find the bounding box of this sprite using flood fill
                let sprite_rect = find_sprite_bounds(&img, x, y, &mut visited);
                sprites.push(sprite_rect);
            }
        }
    }

    fs::create_dir_all(output_dir)?;

    for (i, rect) in sprites.iter().enumerate() {
        let sub_img = image::imageops::crop_imm(
            &img,
            rect.x,
            rect.y,
            rect.width,
            rect.height,
        ).to_image();

        let file_path = format!("{}/sprite_{:03}.png", output_dir, i);
        sub_img.save(&file_path)?;
        println!("cargo:warning=Saved {} ({}x{} at {},{}) ",
                 file_path, rect.width, rect.height, rect.x, rect.y);
    }

    println!("cargo:warning=Extracted {} sprites total", sprites.len());
    Ok(())
}

fn find_sprite_bounds(
    img: &image::RgbaImage,
    start_x: u32,
    start_y: u32,
    visited: &mut HashSet<(u32, u32)>
) -> Rect {
    let (width, height) = img.dimensions();
    let mut to_visit = vec![(start_x, start_y)];
    let mut sprite_pixels = HashSet::new();

    // Flood fill to find all connected non-transparent pixels
    while let Some((x, y)) = to_visit.pop() {
        if visited.contains(&(x, y)) || sprite_pixels.contains(&(x, y)) {
            continue;
        }

        if x >= width || y >= height {
            continue;
        }

        let pixel = img.get_pixel(x, y);
        if pixel[3] == 0 { // Transparent pixel
            continue;
        }

        sprite_pixels.insert((x, y));
        visited.insert((x, y));

        // Add neighbors to visit
        if x > 0 { to_visit.push((x - 1, y)); }
        if x + 1 < width { to_visit.push((x + 1, y)); }
        if y > 0 { to_visit.push((x, y - 1)); }
        if y + 1 < height { to_visit.push((x, y + 1)); }
    }

    // Find bounding box of all sprite pixels
    let min_x = sprite_pixels.iter().map(|(x, _)| *x).min().unwrap_or(start_x);
    let max_x = sprite_pixels.iter().map(|(x, _)| *x).max().unwrap_or(start_x);
    let min_y = sprite_pixels.iter().map(|(_, y)| *y).min().unwrap_or(start_y);
    let max_y = sprite_pixels.iter().map(|(_, y)| *y).max().unwrap_or(start_y);

    Rect {
        x: min_x,
        y: min_y,
        width: max_x - min_x + 1,
        height: max_y - min_y + 1,
    }
}