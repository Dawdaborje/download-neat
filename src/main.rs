mod config;
mod file_handler;

use dirs;
use file_handler::{
    handle_audio_files, handle_document_files, handle_image_files, handle_video_files,
    handle_windows_executables,
};
use notify::{Event, RecursiveMode, Watcher};
use rust_search::SearchBuilder;
use std::{fs, path::Path, sync::mpsc};

const VIDEO_EXTENSIONS: [&str; 12] = [
    "mp4", "mov", "avi", "mkv", "wmv", "flv", "webm", "m4v", "m4p", "m4b", "m4r", "m4a",
];

const IMAGE_EXTENSIONS: [&str; 10] = [
    "jpg", "jpeg", "png", "gif", "bmp", "tiff", "tif", "webp", "heic", "heif",
];

const AUDIO_EXTENSIONS: [&str; 4] = ["mp3", "m4a", "m4b", "m4r"];

const DOCUMENT_EXTENSIONS: [&str; 9] = [
    "pdf", "doc", "docx", "txt", "csv", "xls", "xlsx", "ppt", "pptx",
];

const WINDOWS_EXECUTABLE_EXTENSIONS: [&str; 2] = ["exe", "msi"];

fn process_downloads(download_dir: &Path) {
    let search_results = SearchBuilder::default()
        .location(download_dir)
        .depth(1)
        .build();

    let videos_folder = download_dir.join("Videos");
    let images_folder = download_dir.join("Images");
    let audio_folder = download_dir.join("Audio");
    let documents_folder = download_dir.join("Documents Files");
    let windows_executables_folder = download_dir.join("Windows Executables");

    let mut videos = Vec::new();
    let mut images = Vec::new();
    let mut audio = Vec::new();
    let mut documents = Vec::new();
    let mut windows_executables = Vec::new();

    for result in search_results {
        let full_path = download_dir.join(&result);
        let file_path_str = full_path.to_string_lossy().to_string();

        // First try extension-based detection
        if let Some(ext) = full_path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            if VIDEO_EXTENSIONS.contains(&ext.as_str()) {
                videos.push(file_path_str.clone());
                continue;
            } else if IMAGE_EXTENSIONS.contains(&ext.as_str()) {
                images.push(file_path_str.clone());
                continue;
            } else if AUDIO_EXTENSIONS.contains(&ext.as_str()) {
                audio.push(file_path_str.clone());
                continue;
            } else if DOCUMENT_EXTENSIONS.contains(&ext.as_str()) {
                documents.push(file_path_str.clone());
                continue;
            } else if WINDOWS_EXECUTABLE_EXTENSIONS.contains(&ext.as_str()) {
                windows_executables.push(file_path_str.clone());
                continue;
            }
        }

        // If no extension match, try content-based detection
        if let Ok(buffer) = fs::read(&full_path) {
            if let Some(kind) = infer::get(&buffer) {
                match kind.mime_type() {
                    mime if mime.starts_with("image/") => {
                        images.push(file_path_str);
                    }
                    mime if mime.starts_with("video/") => {
                        videos.push(file_path_str);
                    }
                    mime if mime.starts_with("audio/") => {
                        audio.push(file_path_str);
                    }
                    mime if mime.contains("pdf")
                        || mime.contains("document")
                        || mime.contains("spreadsheet") =>
                    {
                        documents.push(file_path_str);
                    }
                    mime if mime.contains("executable") || mime.contains("application/x-msi") => {
                        windows_executables.push(file_path_str);
                    }
                    _ => {}
                }
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
    if !documents.is_empty() {
        handle_document_files(documents, documents_folder);
    }
    if !windows_executables.is_empty() {
        handle_windows_executables(windows_executables, windows_executables_folder);
    }
}

fn main() -> notify::Result<()> {
    let download_dir = match dirs::download_dir() {
        Some(path) => path,
        None => {
            println!("Failed to find the Downloads directory");
            return Ok(());
        }
    };

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
