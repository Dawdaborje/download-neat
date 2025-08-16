mod file_handler;
mod config;

use dirs;
use rust_search::SearchBuilder;
use file_handler::{ handle_video_files, handle_image_files, handle_audio_files };
use std::{ path::{ Path, PathBuf }, sync::mpsc };
use notify::{ Event, RecursiveMode, Result, Watcher };

const VIDEO_EXTENSIONS: [&str; 12] = [
    "mp4",
    "mov",
    "avi",
    "mkv",
    "wmv",
    "flv",
    "webm",
    "m4v",
    "m4p",
    "m4b",
    "m4r",
    "m4a",
];

const IMAGE_EXTENSIONS: [&str; 10] = [
    "jpg",
    "jpeg",
    "png",
    "gif",
    "bmp",
    "tiff",
    "tif",
    "webp",
    "heic",
    "heif",
];

const AUDIO_EXTENSIONS: [&str; 4] = ["mp3", "m4a", "m4b", "m4r"];

fn process_downloads(download_dir: &Path) {
    let search_results = SearchBuilder::default().location(download_dir).depth(1).build();

    let videos_folder = download_dir.join("Videos");
    let images_folder = download_dir.join("Images");
    let audio_folder = download_dir.join("Audio");

    let mut videos = Vec::new();
    let mut images = Vec::new();
    let mut audio = Vec::new();

    for result in search_results {
        if let Some(ext) = result.split('.').last() {
            let ext = ext.to_lowercase();
            if VIDEO_EXTENSIONS.contains(&ext.as_str()) {
                videos.push(result.clone());
            } else if IMAGE_EXTENSIONS.contains(&ext.as_str()) {
                images.push(result.clone());
            } else if AUDIO_EXTENSIONS.contains(&ext.as_str()) {
                audio.push(result.clone());
            }
        }
    }

    if !videos.is_empty() {
        handle_video_files(videos, videos_folder);
    }
    if !images.is_empty() {
        handle_image_files(images, images_folder);
    }
    if !audio.is_empty() {
        handle_audio_files(audio, audio_folder);
    }
}

fn main() -> notify::Result<()> {
    let download_dir = dirs::download_dir().unwrap();

    process_downloads(&download_dir);

    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(&download_dir, RecursiveMode::Recursive)?;

    println!("Watching Downloads folder for new files...");

    for res in rx {
        match res {
            Ok(event) => {
                println!("event: {:?}", event);

                if event.kind.is_create() {
                    process_downloads(&download_dir);
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
