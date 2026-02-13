use std::mem::MaybeUninit;

use cxx::{ExternType, type_id};

use crate::{ImageConversionFlag, QImage};

#[cxx::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        type ImageConversionFlags = crate::ImageConversionFlags;
        type TransformationMode = crate::TransformationMode;
    }

    unsafe extern "C++" {
        include!("cxx-qt-widgets/qpixmap.h");
        type QImage = crate::QImage;
        type QPixmap = super::QPixmap;

        #[cxx_name = "isNull"]
        fn is_null(self: &QPixmap) -> bool;

        fn width(self: &QPixmap) -> i32;
        fn height(self: &QPixmap) -> i32;

        #[cxx_name = "qpixmapFromImage"]
        fn qpixmap_from_image(image: &QImage, flags: ImageConversionFlags) -> QPixmap;

        #[cxx_name = "scaledToHeight"]
        fn scaled_to_height(self: &QPixmap, height: i32, mode: TransformationMode) -> QPixmap;
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct QPixmap {
    _a1: MaybeUninit<usize>,
    _a2: MaybeUninit<usize>,
    _a3: MaybeUninit<usize>,
}

impl QPixmap {
    pub fn from_image(image: &QImage) -> Self {
        ffi::qpixmap_from_image(image, ImageConversionFlag::AutoColor.into())
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QPixmap {
    type Id = type_id!("QPixmap");
    type Kind = cxx::kind::Trivial;
}
