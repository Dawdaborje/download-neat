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

pub fn handle_windows_executables(
    windows_executables: Vec<String>,
    windows_executables_folder_to_tranfer: PathBuf
) {
    println!("Found {} windows executables", windows_executables.len());
    let folder = Path::new(&windows_executables_folder_to_tranfer);

    if folder.exists() {
        println!("Windows executables folder already exists: {}", folder.display());
    } else {
        fs::create_dir_all(folder).unwrap();
        println!("Windows executables folder created: {}", folder.display());
    }

    for windows_executable in windows_executables {
        let old_file_name = get_file_name(&windows_executable);

        fs::rename(
            windows_executable,
            windows_executables_folder_to_tranfer.join(&old_file_name)
        ).unwrap();
        println!(
            "Windows executable: {} -> {}",
            &old_file_name,
            windows_executables_folder_to_tranfer.join(&old_file_name).display()
        );
    }
}

pub fn handle_document_files(documents: Vec<String>, document_folder_to_tranfer: PathBuf) {
    println!("Found {} documents", documents.len());
    let folder = Path::new(&document_folder_to_tranfer);

    if folder.exists() {
        println!("Document folder already exists: {}", folder.display());
    } else {
        fs::create_dir_all(folder).unwrap();
        println!("Document folder created: {}", folder.display());
    }

    let mut pdfs = Vec::new();
    let mut word_files = Vec::new();
    let mut csv_files = Vec::new();
    let mut txt_files = Vec::new();

    for document in documents {
        if let Some(ext) = document.split('.').last() {
            let ext = ext.to_lowercase();
            if ext == "pdf" {
                pdfs.push(document.clone());
            } else if ext == "doc" || ext == "docx" {
                word_files.push(document.clone());
            } else if ext == "csv" || ext == "xls" || ext == "xlsx" {
                csv_files.push(document.clone());
            } else if ext == "txt" {
                txt_files.push(document.clone());
            }
        }
    }

    if !pdfs.is_empty() {
        handle_pdf_files(pdfs, &document_folder_to_tranfer);
    }

    if !word_files.is_empty() {
        handle_word_files(word_files, &document_folder_to_tranfer);
    }

    if !csv_files.is_empty() {
        handle_csv_files(csv_files, &document_folder_to_tranfer);
    }

    if !txt_files.is_empty() {
        handle_txt_files(txt_files, &document_folder_to_tranfer);
    }
}

pub fn handle_csv_files(csv_files: Vec<String>, csv_folder_to_tranfer: &PathBuf) {
    println!("Found {} csv files", csv_files.len());
    let folder = Path::new(&csv_folder_to_tranfer).join("CSV Files");

    if folder.exists() {
        println!("CSV folder already exists: {}", &folder.display());
    } else {
        fs::create_dir_all(&folder).unwrap();
        println!("CSV folder created: {}", &folder.display());
    }

    for csv_file in csv_files {
        let old_file_name = get_file_name(&csv_file);

        fs::rename(csv_file, folder.join(&old_file_name)).unwrap();
        println!("CSV: {} -> {}", &old_file_name, folder.join(&old_file_name).display());
    }
}

pub fn handle_txt_files(txt_files: Vec<String>, txt_folder_to_tranfer: &PathBuf) {
    println!("Found {} txt files", txt_files.len());
    let folder = Path::new(&txt_folder_to_tranfer).join("Text Files");

    if folder.exists() {
        println!("TXT folder already exists: {}", folder.display());
    } else {
        fs::create_dir_all(&folder).unwrap();
        println!("TXT folder created: {}", &folder.display());
    }

    for txt_file in txt_files {
        let old_file_name = get_file_name(&txt_file);

        fs::rename(txt_file, folder.join(&old_file_name)).unwrap();
        println!("TXT: {} -> {}", &old_file_name, folder.join(&old_file_name).display());
    }
}

fn handle_pdf_files(pdfs: Vec<String>, pdf_folder_to_tranfer: &PathBuf) {
    println!("Found {} pdfs", pdfs.len());
    let folder = Path::new(&pdf_folder_to_tranfer).join("PDF Files");

    if folder.exists() {
        println!("PDF folder already exists: {}", &folder.display());
    } else {
        fs::create_dir_all(&folder).unwrap();
        println!("PDF folder created: {}", &folder.display());
    }

    for pdf in pdfs {
        let old_file_name = get_file_name(&pdf);

        fs::rename(pdf, folder.join(&old_file_name)).unwrap();
        println!("PDF: {} -> {}", &old_file_name, folder.join(&old_file_name).display());
    }
}

fn handle_word_files(word_files: Vec<String>, word_folder_to_tranfer: &PathBuf) {
    println!("Found {} word files", word_files.len());
    let folder = Path::new(&word_folder_to_tranfer).join("Word Files");

    if folder.exists() {
        println!("Word folder already exists: {}", folder.display());
    } else {
        fs::create_dir_all(&folder).unwrap();
        println!("Word folder created: {}", &folder.display());
    }

    for word_file in word_files {
        let old_file_name = get_file_name(&word_file);

        fs::rename(word_file, folder.join(&old_file_name)).unwrap();
        println!("Word file: {} -> {}", &old_file_name, folder.join(&old_file_name).display());
    }
}
