fn main() {
    // Add library search paths for both macOS and Linux
    println!("cargo:rustc-link-search=/usr/local/lib");
    println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-search=/lib");
    println!("cargo:rustc-link-search=/lib/aarch64-linux-gnu");
    println!("cargo:rustc-link-search=/usr/lib/aarch64-linux-gnu");
    
    // Add the path where we found the ARM64 Vosk library
    println!("cargo:rustc-link-search=/home/jgnoonan/rustroi2/vosk-linux-aarch64-0.3.45");
    
    // Link against vosk dynamically
    println!("cargo:rustc-link-lib=dylib=vosk");

    // Add rpath for dynamic libraries
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/local/lib");
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib");
    println!("cargo:rustc-link-arg=-Wl,-rpath,/lib/aarch64-linux-gnu");
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/aarch64-linux-gnu");
    println!("cargo:rustc-link-arg=-Wl,-rpath,/home/jgnoonan/rustroi2/vosk-linux-aarch64-0.3.45");
    
    // Rerun this script if the libraries change
    // For macOS
    println!("cargo:rerun-if-changed=/usr/local/lib/libvosk.dylib");
    println!("cargo:rerun-if-changed=/usr/lib/libvosk.dylib");
    // For Linux
    println!("cargo:rerun-if-changed=/usr/local/lib/libvosk.so");
    println!("cargo:rerun-if-changed=/usr/lib/libvosk.so");
    println!("cargo:rerun-if-changed=/lib/aarch64-linux-gnu/libvosk.so");
    println!("cargo:rerun-if-changed=/usr/lib/aarch64-linux-gnu/libvosk.so");
    println!("cargo:rerun-if-changed=/home/jgnoonan/rustroi2/vosk-linux-aarch64-0.3.45/libvosk.so");
}
