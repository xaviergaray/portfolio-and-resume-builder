use std::fs;
use std::path::Path;

pub fn output_latex(latex: &str, output_path: &str, filename: &str) {    
    fs::write(format!("{output_path}/{filename}.tex"), latex).expect("failed to write latex file");

    let status = std::process::Command::new("pdflatex")
        .arg("-interaction=nonstopmode")
        .arg("-output-directory")
        .arg(output_path.clone())
        .arg(format!("{filename}.tex"))
        .status().expect("failed to execute pdflatex");

    if status.success() {
        println!("PDF successfully compiled to {output_path}/{filename}.pdf");
    } else {
        eprintln!("Failed to compile LaTeX.");
    }
}

pub fn copy_file(src: impl AsRef<Path>, dst: impl AsRef<Path>) {
    let src = src.as_ref();
    let dst = dst.as_ref();

    if let Some(parent) = dst.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("failed to create parent directory");
        }
    }

    fs::copy(src, dst).expect(&format!("failed to copy file from {} to {}", src.display(), dst.display()));
}

pub fn copy_dir_recursive(src: impl AsRef<Path>, dst: impl AsRef<Path>) {
    let src = src.as_ref();
    let dst = dst.as_ref();

    if !dst.exists() {
        fs::create_dir_all(dst).expect("failed to create destination directory");
    }

    for entry in fs::read_dir(src).expect("failed to read directory") {
        let entry = entry.expect("failed to read directory entry");
        let file_type = entry.file_type().expect("failed to get file type");
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursive(src_path, dst_path);
        } else {
            copy_file(&src_path, &dst_path);
        }
    }
}