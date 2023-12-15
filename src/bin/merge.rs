use std::{fs, io};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use image::{DynamicImage, ImageError, ImageFormat, imageops, RgbaImage};
use rayon::prelude::*;
use walkdir::WalkDir;

const CSV_NOT_FOUND: &str = "
Failed to find vcglist.csv in any subdirectory,
you should extract system.dat and other resources into this folder,
then rerun this program.";
const IMAGE_NOT_FOUND: &str = "
Missing image file,
did you well extracted all the images?";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let processed_images = AtomicUsize::new(0);
    let folder = PathBuf::from("../..");
    let csv_file_path = WalkDir::new(&folder)
        .into_iter()
        .filter_map(Result::ok)
        .find(|e| e.file_name() == "vcglist.csv")
        .ok_or(CSV_NOT_FOUND)?
        .into_path();
    let cg_list = fs::read_to_string(csv_file_path)?;
    let total_images = cg_list.lines().count();
    println!("Total images: {}", total_images);
    let output_path = folder.join("merged");
    fs::create_dir_all(&output_path)?;
    let image_cache: Arc<dashmap::DashMap<String, DynamicImage>> = Arc::new(dashmap::DashMap::new());

    // Process images
    cg_list.par_lines().try_for_each(|line| -> Result<(), ImageError> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let cg_name = parts[0];
        let images: Result<Vec<DynamicImage>, ImageError> = parts.iter().skip(1)
            .map(|&part| {
                let file_name = format!("{}.webp", part);
                let cache = image_cache.clone();
                if let Some(image) = cache.get(part) {
                    return Ok(image.clone());
                }
                let image = find_and_open_image(&folder, &file_name)?;
                cache.insert(part.to_string(), image.clone());
                Ok(image)
            })
            .collect();

        // Combine images
        match images {
            Ok(images) if !images.is_empty() => {
                let combined_image = combine_images(&images)?;
                let image_path = output_path.join(format!("{}.{:?}", cg_name, ImageFormat::Png));
                combined_image.save_with_format(&image_path, ImageFormat::Png)?;
            }
            _ => eprintln!("Failed to combine images for {}", cg_name),
        }

        // Print progress every 10 images
        let processed = processed_images.fetch_add(1, Ordering::SeqCst) + 1;
        if processed % 10 == 0 {
            print_progress(processed, total_images, start_time);
        }

        Ok(())
    })?;

    // Print benchmark
    println!("Processed {} images, total images: {}", processed_images.load(Ordering::SeqCst), total_images);
    println!();
    println!("All done. 🏳️‍⚧️🦀");

    Ok(())
}

fn find_and_open_image(folder: &Path, file_name: &str) -> Result<DynamicImage, ImageError> {
    let file_path = WalkDir::new(folder)
        .into_iter()
        .filter_map(Result::ok)
        .find(|e| e.file_name() == file_name)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, IMAGE_NOT_FOUND))?
        .into_path();
    image::open(file_path)
}

fn combine_images(images: &[DynamicImage]) -> Result<RgbaImage, ImageError> {
    images.iter().try_fold(None, |combined, image| {
        let watermark = image.to_rgba8();
        match combined {
            Some(mut base) => {
                imageops::overlay(&mut base, &watermark, 0, 0);
                Ok(Some(base))
            }
            None => Ok(Some(watermark)),
        }
    }).map(|combined| combined.expect(IMAGE_NOT_FOUND))
}

fn print_progress(processed: usize, total_images: usize, start_time: Instant) {
    let elapsed_time = start_time.elapsed().as_secs_f64();
    let images_per_second = processed as f64 / elapsed_time;
    let percent_complete = (processed as f64 / total_images as f64) * 100.0;
    println!(
        "Processed: {}/{} ({:.2}%), Time: {:.2}s, Rate: {:.2} images/s, Remaining: {:.0}m:{:.0}s",
        processed, total_images, percent_complete, elapsed_time, images_per_second,
        ((total_images - processed) as f64 / images_per_second) / 60.0,
        ((total_images - processed) as f64 / images_per_second) % 60.0,
    );
}
