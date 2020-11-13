fn main() {
    println!("cargo:rerun-if-file-changed=shaders/main.glsl");
    println!("cargo:rerun-if-file-changed=shaders/geometry.glsl");
    println!("cargo:rerun-if-file-changed=shaders/random.glsl");
}
