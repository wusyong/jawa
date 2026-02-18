use cxx_qt_build::CxxQtBuilder;

fn main() {
    let _ = CxxQtBuilder::new()
        .qt_module("Widgets")
        .qt_module("PrintSupport")
        .qt_module("WebEngineCore")
        .file("src/printer_handler.rs")
        .qrc("data/data.qrc")
        .qrc_resources(["data/index.html", "data/style.css"])
        .build();
}
