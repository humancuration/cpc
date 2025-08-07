use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Create the dist directory if it doesn't exist
    let dist_dir = Path::new("dist");
    if !dist_dir.exists() {
        fs::create_dir_all(dist_dir).expect("Failed to create dist directory");
    }
    
    // Copy static assets
    let assets_dir = Path::new("assets");
    if assets_dir.exists() {
        copy_dir_all(assets_dir, &dist_dir.join("assets"))
            .expect("Failed to copy assets");
    }
    
    println!("cargo:rerun-if-changed=assets/");
    println!("cargo:rerun-if-changed=src/");
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if file_type.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    
    Ok(())
}