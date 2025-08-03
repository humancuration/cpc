fn main() {
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=static/styles.css");
    println!("cargo:rerun-if-changed=../../shared_packages/feedback_core/src/lib.rs");
    println!("cargo:rerun-if-changed=../../shared_packages/feedback_analysis/src/lib.rs");
    println!("cargo:rerun-if-changed=../../shared_packages/feedback_visualization/src/lib.rs");
    println!("cargo:rerun-if-changed=../../shared_packages/reviews/src/lib.rs");
    println!("cargo:rerun-if-changed=../../shared_packages/survey/src/lib.rs");
}