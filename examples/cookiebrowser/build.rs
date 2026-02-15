use cxx_qt_build::CxxQtBuilder;
use std::path::PathBuf;

fn header_dir() -> PathBuf {
    PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("include")
}

fn write_headers() {
    println!("cargo::rerun-if-changed=include/");
    std::fs::create_dir_all(header_dir()).expect("Failed to create include directory");

    for entry in std::fs::read_dir("include").expect("Failed to read include directory") {
        let entry = entry.expect("Failed to read header file!");
        let header_name = entry.file_name();
        println!(
            "cargo::rerun-if-changed=include/{header_name}",
            header_name = header_name.to_string_lossy()
        );

        std::fs::copy(entry.path(), header_dir().join(header_name))
            .expect("Failed to copy header file!");
    }
}

fn main() {
    write_headers();

    let _ = CxxQtBuilder::new()
        .qt_module("Network")
        .qt_module("Widgets")
        .qt_module("WebEngineCore")
        .qrc("data/data.qrc")
        .file("src/main.rs")
        .cpp_file("include/mainwindow.h")
        .include_prefix("private")
        .crate_include_root(Some("include".to_owned()))
        .build();
}
