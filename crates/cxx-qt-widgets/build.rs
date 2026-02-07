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

    let mut builder = CxxQtBuilder::new()
        .qt_module("Gui")
        .qt_module("Widgets")
        .qt_module("WebEngineCore")
        .qt_module("WebEngineWidgets");

    let rust_bridges = vec![
        "qboxlayout",
        "qdesktopservices",
        "qhboxlayout",
        "qlabel",
        "qlayout",
        "qmainwindow",
        "qmessagebox",
        "qmouseevent",
        "qpushbutton",
        "qsinglepointevent",
        // "qsizepolicy",
        // "qspaceritem",
        "qt",
        "qtimer",
        "qvboxlayout",
        "qwebenginenotification",
        "qwebenginepage",
        "qwebengineprofile",
        "qwebenginepermission",
        "qwebengineview",
        "qwidget",
    ];

    for rust_source in &rust_bridges {
        builder = builder.file(format!("src/{rust_source}.rs"));
    }

    // let cpp_files = vec![
    //     "qwebenginepermission",
    // ];

    // for cpp_file in &cpp_files {
    //     builder = builder.cpp_file(format!("src/{cpp_file}.cpp"));
    // }
    // builder = builder.cpp_file("src/qt_types.cpp");

    println!("cargo::rerun-if-changed=src/assertion_utils.h");

    let interface = builder
        // Use a short name due to the Windows file path limit!
        // We don't re-export these headers anyway.
        .include_prefix("private")
        .crate_include_root(Some("include".to_owned()))
        .build();

    // Disable exporting the standard include directory, as we are exporting custom header
    interface.reexport_dependency("cxx-qt-lib").export();
}
