fn main() {
    // Link Python library
    println!("cargo:rustc-link-lib=python311");

    // Specify search path for Python libraries
    println!("cargo:rustc-link-search=native=C:\\Users\\Hp\\AppData\\Local\\Programs\\Python\\Python311\\libs");

    // Ensure rebuild if Python changes
    println!("cargo:rerun-if-env-changed=PYTHON_SYS_EXECUTABLE");
}
