use std::fs;
use std::path::{ Path, PathBuf };

fn get_file_name(file_path: &str) -> String {
    let file_name = PathBuf::from(file_path).file_name().unwrap().to_str().unwrap().to_string();
    file_name
}

pub fn handle_video_files(videos: Vec<String>, video_folder_to_tranfer: PathBuf) {
    println!("Found {} videos", videos.len());
    let folder = Path::new(&video_folder_to_tranfer);

    if folder.exists() {
        println!("Video folder already exists: {}", folder.display());
    } else {
        fs::create_dir_all(folder).unwrap();
        println!("Video folder created: {}", folder.display());
    }

    for video in videos {
        let old_file_name = get_file_name(&video);

        fs::rename(video, video_folder_to_tranfer.join(&old_file_name)).unwrap();
        println!(
            "Video: {} -> {}",
            &old_file_name,
            video_folder_to_tranfer.join(&old_file_name).display()
        );
    }
}

pub fn handle_image_files(images: Vec<String>, image_folder_to_tranfer: PathBuf) {
    println!("Found {} images", images.len());
    let folder = Path::new(&image_folder_to_tranfer);

    if folder.exists() {
        println!("Image folder already exists: {}", folder.display());
    } else {
        fs::create_dir_all(folder).unwrap();
        println!("Image folder created: {}", folder.display());
    }

    for image in images {
        let old_file_name = get_file_name(&image);

        fs::rename(image, image_folder_to_tranfer.join(&old_file_name)).unwrap();
        println!(
            "Image: {} -> {}",
            &old_file_name,
            image_folder_to_tranfer.join(&old_file_name).display()
        );
    }
}

pub fn handle_audio_files(audio: Vec<String>, audio_folder_to_tranfer: PathBuf) {
    println!("Found {} audio files", audio.len());
    let folder = Path::new(&audio_folder_to_tranfer);

    if folder.exists() {
        println!("Audio folder already exists: {}", folder.display());
    } else {
        fs::create_dir_all(folder).unwrap();
        println!("Audio folder created: {}", folder.display());
    }

    for audio in audio {
        let old_file_name = get_file_name(&audio);

        fs::rename(audio, audio_folder_to_tranfer.join(&old_file_name)).unwrap();
        println!(
            "Audio: {} -> {}",
            &old_file_name,
            audio_folder_to_tranfer.join(&old_file_name).display()
        );
    }
}
