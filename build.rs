fn main() {
    // Add library search paths
    println!("cargo:rustc-link-search=/usr/local/lib");
    println!("cargo:rustc-link-search=/usr/lib");
    
    // Link against vosk dynamically
    println!("cargo:rustc-link-lib=dylib=vosk");

    // Add rpath for dynamic libraries
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/local/lib");
    
    // Rerun this script if the libraries change
    println!("cargo:rerun-if-changed=/usr/local/lib/libvosk.dylib");
    println!("cargo:rerun-if-changed=/usr/lib/libvosk.dylib");
}
