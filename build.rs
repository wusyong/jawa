use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        // Link Qt's Network library
        // - Qt Core is always linked
        // - Qt Gui is linked by enabling the qt_gui Cargo feature of cxx-qt-lib.
        .qt_module("Network")
        .qt_module("Widgets")
        .qt_module("WebEngineWidgets")
        .file("src/qpushbutton.rs")
        .file("src/qwebengineview.rs")
        .build();
}
