use pyo3::prelude::*;
use std::ffi::CString;

use std::fs;
use std::process::Command;

pub fn main() {
    println!("Streamlit app is running..");
    Python::with_gil(|py| {
        // Dynamically get Python executable path and site-packages
        let setup_code = r#"
import sys
import os

# Automatically detect the current working directory and append the py_script path
py_script_path = os.path.join(os.getcwd(), 'py_script')
sys.path.insert(0, py_script_path)

# Alternatively, use an environment variable for py_script path
# py_script_path = os.getenv("PY_SCRIPT_PATH", os.path.join(os.getcwd(), 'py_script'))
# sys.path.insert(0, py_script_path)
"#;

        // Convert the setup code to a C-style string and execute it
        let setup_string =
            CString::new(setup_code).expect("Failed to create CString for setup code.");
        py.run(setup_string.as_c_str(), None, None).unwrap();

        // Path to the external Python file
        let python_file_path = "py_scripts/main.py";

        // Read the Python script from the file
        let python_code = fs::read_to_string(python_file_path)
            .expect("Failed to read the Python file. Ensure the file exists and is readable.");

        // Convert the Python script into a CString
        let python_cstring =
            CString::new(python_code).expect("Failed to create CString for Python code.");

        // Launch Streamlit using the system's Python environment
        let streamlit_command = Command::new("streamlit")
            .arg("run")
            .arg(python_file_path)
            .status()
            .expect("Failed to start Streamlit server");

        if streamlit_command.success() {
            ()
        } else {
            eprintln!("Failed to run the Streamlit app. Check if Streamlit is installed and the path is correct.");
        }

        // Execute the Python script
        py.run(python_cstring.as_c_str(), None, None).unwrap();
    });

    println!("Finished executing the Python code from Rust.");
}
