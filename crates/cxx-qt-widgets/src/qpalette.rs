use std::pin::Pin;

use cxx::UniquePtr;

pub use ffi::{ColorRole, QPalette};

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;
    }

    unsafe extern "C++" {
        include!("cxx-qt-widgets/qpalette.h");
        type QWidget = crate::QWidget;

        /// Color palette for widgets.
        type QPalette;

        #[cxx_name = "qpaletteSetColor"]
        fn qpalette_set_color(palette: Pin<&mut QPalette>, role: ColorRole, color: &QColor);

        #[cxx_name = "qpaletteColor"]
        fn qpalette_color(palette: &QPalette, role: ColorRole) -> QColor;

        // #[cxx_name = "qwidgetPalette"]
        // fn qwidget_palette(widget: &QWidget) -> UniquePtr<QPalette>;

        // #[cxx_name = "qwidgetSetPalette"]
        // fn qwidget_set_palette(widget: Pin<&mut QWidget>, palette: &QPalette);

        // #[cxx_name = "qwidgetBackgroundRole"]
        // fn qwidget_background_role(widget: &QWidget) -> ColorRole;
    }

    #[repr(u32)]
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    enum ColorRole {
        WindowText = 0,
        Button = 1,
        Light = 2,
        Midlight = 3,
        Dark = 4,
        Mid = 5,
        Text = 6,
        BrightText = 7,
        ButtonText = 8,
        Base = 9,
        Window = 10,
        Shadow = 11,
        Highlight = 12,
        HighlightedText = 13,
        Link = 14,
        LinkVisited = 15,
        AlternateBase = 16,
        NoRole = 17,
        ToolTipBase = 18,
        ToolTipText = 19,
        PlaceholderText = 20,
        Accent = 21,
    }

    extern "C++" {
        type ColorRole;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_palette() -> UniquePtr<QPalette>;
    }
}

impl ffi::QPalette {
    /// Creates a new palette with default values.
    pub fn new() -> UniquePtr<Self> {
        ffi::new_palette()
    }

    pub fn set_color(self: Pin<&mut Self>, role: ColorRole, color: &cxx_qt_lib::QColor) {
        ffi::qpalette_set_color(self, role, color);
    }

    pub fn color(&self, role: ColorRole) -> cxx_qt_lib::QColor {
        ffi::qpalette_color(self, role)
    }
}
