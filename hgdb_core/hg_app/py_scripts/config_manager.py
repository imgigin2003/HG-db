import os
import toml
import subprocess
import streamlit as st
import pandas as pd
import json
from data_loader import DataLoader

class ConfigManager:
    def __init__(self):
        self.data_loader = DataLoader()

    def load_db_path(self):
        """Load db_path from Config.toml with correct path"""
        config_file = self.data_loader.get_resources_root() / "Config.toml"
        default_path = self.data_loader.get_statics_root() / "mydbs" / "rocksdb"

        if config_file.exists():
            try:
                with open(config_file, "r") as f:
                    config_data = toml.load(f)
                return config_data.get("db_path", default_path)
            
            except Exception as e:
                st.error(f"Failed to load '{config_file}': {str(e)}. Using default path.")
        return ""

    def update_db_path(self, new_path):
        """Update the db_path in Config.toml with correct path"""
        config_file = self.data_loader.get_resources_root() / "Config.toml"
        try:
            with open(config_file, "r") as f:
                config_data = toml.load(f)
            
            config_data["db_path"] = new_path
            
            with open(config_file, "w") as f:
                toml.dump(config_data, f)
            return True
        
        except Exception as e:
            st.error(f"Error updating Config.toml: {e}")
            return False

    def get_python_version(self):
        """Get Python version (e.g., '3.13') using subprocess"""
        try:
            result = subprocess.run(
                ['python3', '--version'],
                capture_output=True,
                text=True,
                check=True
            )
            version = result.stdout.strip().split()[1]  # Gets "3.13.0" from "Python 3.13.0"
            return '.'.join(version.split('.')[:2])  # Returns "3.13"
        except Exception as e:
            st.error(f"Error detecting Python version: {e}")
            return None

    def get_python_library_path(self):
        """Get the correct Python library path without appending the version"""
        try:
            # First try to get the lib directory path
            result = subprocess.run(
                ['python3', '-c', "import sysconfig; print(sysconfig.get_config_var('LIBDIR'))"],
                capture_output=True,
                text=True,
                check=True
            )
            lib_path = result.stdout.strip()
            
            # Verify this is the correct path by checking for Python library files
            if lib_path and os.path.exists(lib_path):
                # Check for common Python library file patterns
                lib_files = os.listdir(lib_path)
                if any(f.startswith('libpython3') for f in lib_files):
                    return lib_path
            
            # Fallback to stdlib path if LIBDIR doesn't work
            result = subprocess.run(
                ['python3', '-c', "import sysconfig; print(sysconfig.get_path('stdlib'))"],
                capture_output=True,
                text=True,
                check=True
            )
            stdlib_path = result.stdout.strip()
            if stdlib_path:
                # Go up one level from site-packages to lib directory
                return os.path.dirname(stdlib_path)
                
            return None
        
        except subprocess.CalledProcessError as e:
            st.error(f"Error detecting Python library path: {e}")
            return None

    def update_build_rs(self, python_path, python_version):
        """Update build.rs with correct paths without appending python version"""
        build_rs_path = self.data_loader.get_statics_root() / "build.rs"
        
        try:
            with open(build_rs_path, "w") as f:
                f.write(f"""fn main() {{
    // Link the Python {python_version} library
    println!("cargo:rustc-link-lib=python{python_version}");

    // Specify the search path for Python libraries (without pythonX.Y suffix)
    println!("cargo:rustc-link-search=native={python_path}");

    // Ensure Rust rebuilds when Python version changes
    println!("cargo:rerun-if-env-changed=PYTHON_SYS_EXECUTABLE");
}}""")
            return True
        except Exception as e:
            st.error(f"Error updating build.rs: {e}")
            return False
        
    def get_upload_dir(self):
        """Get the upload directory path based on configured db_path"""
        db_path = self.load_db_path()
        if not db_path:
            return None
        return os.path.join(db_path, "hg_Uploads")

    def ensure_upload_dir(self):
        """Ensure upload directory exists"""
        upload_dir = self.get_upload_dir()
        if upload_dir and not os.path.exists(upload_dir):
            os.makedirs(upload_dir, exist_ok=True)
        return upload_dir

    def display_db_config_properties(self):
        """Display DB-Config Properties from JSON file"""
        FILE_PATH = self.data_loader.get_resources_root() / "resources" / "db-config.json"
        
        try:
            with open(FILE_PATH, "r") as f:
                db_data = json.load(f)

            # Column Families table
            properties = []
            for data in db_data["ColumnFamilies"]:
                properties.append({
                    "Name": data["name"],
                    "Type": data["properties"]["type"],
                    "Rows": data["properties"]["numer_of_rows"],
                    "Config": data["properties"]["config"] if data["properties"]["config"] != "None" else "Default"
                })
            # Display dataframe
            df = pd.DataFrame(properties)
            st.dataframe(df)

        except Exception as e:
            st.error(f"Error loading DB-Config properties: {str(e)}")
