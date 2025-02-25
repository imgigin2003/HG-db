fn main() {
    // Link the Python 3.13 library
    println!("cargo:rustc-link-lib=python3.13");

    // Specify the search path for the Python libraries
    println!("cargo:rustc-link-search=native=/usr/local/opt/python@3.13/Frameworks/Python.framework/Versions/3.13/lib");

    // Ensure Rust rebuilds when Python version changes
    println!("cargo:rerun-if-env-changed=PYTHON_SYS_EXECUTABLE");
}
