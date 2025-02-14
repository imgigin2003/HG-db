fn main() {
    // Tell cargo to tell rustc to link the system rocksdb library
    println!("cargo:rustc-link-lib=dylib=rocksdb");

    // Specify the location of RocksDB headers
    println!("cargo:include=/usr/local/Cellar/rocksdb/9.10.0/include");

    // Specify the location of RocksDB libraries
    println!("cargo:lib=/usr/local/Cellar/rocksdb/9.10.0/lib");
}
