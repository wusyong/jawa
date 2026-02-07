// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{QFlags, unsafe_impl_qflag};

#[cxx::bridge(namespace = "Qt")]
mod ffi {
    unsafe extern "C++" {
        include!(<QtWidgets/QWidget>);
    }

    #[derive(Debug)]
    #[repr(u32)]
    /// This type is used to signify an object's orientation.
    enum AlignmentFlag {
        AlignLeft = 0x0001,
        // AlignLeading = AlignLeft,
        AlignRight = 0x0002,
        // AlignTrailing = AlignRight,
        AlignHCenter = 0x0004,
        AlignJustify = 0x0008,
        AlignAbsolute = 0x0010,
        AlignHorizontal_Mask = 0x001f,
        AlignTop = 0x0020,
        AlignBottom = 0x0040,
        AlignVCenter = 0x0080,
        AlignBaseline = 0x0100,
        // Note that 0x100 will clash with Qt::TextSingleLine = 0x100 due to what the comment above
        // this enum declaration states. However, since Qt::AlignBaseline is only used by layouts,
        // it doesn't make sense to pass Qt::AlignBaseline to QPainter::drawText(), so there
        // shouldn't really be any ambiguity between the two overlapping enum values.
        AlignVertical_Mask = 0x01e0,
        AlignCenter = 0x0084
    }

    #[derive(Debug)]
    #[repr(u32)]
    /// This type is used to signify an object's orientation.
    enum WindowType {
        Widget = 0x00000000,
        Window = 0x00000001,
        Dialog = 0x00000003,
        Sheet = 0x00000005,
        Drawer = 0x00000007,
        Popup = 0x00000009,
        Tool = 0x0000000b,
        ToolTip = 0x0000000d,
        SplashScreen = 0x0000000f,
        Desktop = 0x00000011,
        SubWindow = 0x00000012,
        ForeignWindow = 0x00000021,
        CoverWindow = 0x00000041,
        WindowType_Mask = 0x000000ff,
        MSWindowsFixedSizeDialogHint = 0x00000100,
        MSWindowsOwnDC = 0x00000200,
        BypassWindowManagerHint = 0x00000400,
        // X11BypassWindowManagerHint = 0x00000400,
        FramelessWindowHint = 0x00000800,
        WindowTitleHint = 0x00001000,
        WindowSystemMenuHint = 0x00002000,
        WindowMinimizeButtonHint = 0x00004000,
        WindowMaximizeButtonHint = 0x00008000,
        WindowMinMaxButtonsHint = 0x0000c000,
        WindowContextHelpButtonHint = 0x00010000,
        WindowShadeButtonHint = 0x00020000,
        WindowStaysOnTopHint = 0x00040000,
        WindowTransparentForInput = 0x00080000,
        WindowOverridesSystemGestures = 0x00100000,
        WindowDoesNotAcceptFocus = 0x00200000,
        // MaximizeUsingFullscreenGeometryHint = ?,
        CustomizeWindowHint = 0x02000000,
        WindowStaysOnBottomHint = 0x04000000,
        WindowCloseButtonHint = 0x08000000,
        // MacWindowToolBarButtonHint = deprecated,
        BypassGraphicsProxyWidget = 0x20000000,
        NoDropShadowWindowHint = 0x40000000,
        WindowFullscreenButtonHint = 0x80000000,
    }

    extern "C++" {
        type AlignmentFlag;
        type WindowType;
    }
}

use cxx::type_id;
pub use ffi::{WindowType, AlignmentFlag};

/// [`QFlags`] of [`WindowType`].
pub type WindowFlags = QFlags<WindowType>;
unsafe_impl_qflag!(WindowType, "Qt::WindowFlags", u32);

/// [`QFlags`] of [`AlignmentFlag`].
pub type Alignment = QFlags<AlignmentFlag>;
unsafe_impl_qflag!(AlignmentFlag, "Qt::Alignment", u32);