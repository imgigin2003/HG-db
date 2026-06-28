use pyo3::prelude::*;
use std::ffi::CString;
use std::process::Command;

pub fn main() {
    println!("Streamlit app is running...");

    // Initialize the Python interpreter (replaces prepare_freethreaded_python)
    Python::initialize();

    Python::attach(|py| {
        let setup_code = r#"
import sys, os
py_script_path = os.path.join(os.getcwd(), 'py_scripts')
if py_script_path not in sys.path:
    sys.path.insert(0, py_script_path)
"#;
        py.run(&CString::new(setup_code).unwrap(), None, None)
            .expect("Python setup failed");
    });

    // Launch Streamlit as a subprocess — this blocks until the app exits
    let status = Command::new("streamlit")
        .arg("run")
        .arg("py_scripts/main.py")
        .status()
        .expect("Failed to start Streamlit — is it installed?");

    if !status.success() {
        eprintln!("Streamlit exited with: {}", status);
    }
}
