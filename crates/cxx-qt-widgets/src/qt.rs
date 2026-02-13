use crate::{QFlags, unsafe_impl_qflag};

#[cxx::bridge(namespace = "Qt")]
mod ffi {
    unsafe extern "C++" {
        include!(<QtWidgets/QWidget>);
    }

    #[derive(Debug)]
    #[repr(u32)]
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

    #[derive(Debug)]
    #[repr(u32)]
    enum ImageConversionFlag {
        // ColorMode_Mask          = 0x00000003,
        AutoColor               = 0x00000000,
        ColorOnly               = 0x00000003,
        MonoOnly                = 0x00000002,
        // Reserved             = 0x00000001,
        // AlphaDither_Mask        = 0x0000000c,
        // ThresholdAlphaDither    = 0x00000000,
        OrderedAlphaDither      = 0x00000004,
        DiffuseAlphaDither      = 0x00000008,
        NoAlpha                 = 0x0000000c, // Not supported
        Dither_Mask             = 0x00000030,
        // DiffuseDither           = 0x00000000,
        OrderedDither           = 0x00000010,
        ThresholdDither         = 0x00000020,
        // ReservedDither       = 0x00000030,
        DitherMode_Mask         = 0x000000c0,
        // AutoDither              = 0x00000000,
        PreferDither            = 0x00000040,
        AvoidDither             = 0x00000080,
        NoOpaqueDetection       = 0x00000100,
        NoFormatConversion      = 0x00000200
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum TransformationMode {
        FastTransformation,
        SmoothTransformation,
    }

    extern "C++" {
        type AlignmentFlag;
        type WindowType;
        type ImageConversionFlag;
        type TransformationMode;
    }
}

pub use ffi::{WindowType, AlignmentFlag, ImageConversionFlag, TransformationMode};

/// [`QFlags`] of [`WindowType`].
pub type WindowFlags = QFlags<WindowType>;
unsafe_impl_qflag!(WindowType, "Qt::WindowFlags", u32);

/// [`QFlags`] of [`AlignmentFlag`].
pub type Alignment = QFlags<AlignmentFlag>;
unsafe_impl_qflag!(AlignmentFlag, "Qt::Alignment", u32);

/// [`QFlags`] of [`ImageConversionFlag`].
pub type ImageConversionFlags = QFlags<ffi::ImageConversionFlag>;
unsafe_impl_qflag!(ffi::ImageConversionFlag, "Qt::ImageConversionFlags", u32);