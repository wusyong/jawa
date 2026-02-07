use std::mem::MaybeUninit;

use cxx::{ExternType, type_id};
pub use ffi::{PermissionType, State};

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-widgets/qwebenginepermission.h");
        /// Represents a permission request for web content.
        type QWebEnginePermission = super::QWebEnginePermission;
        type QUrl = cxx_qt_lib::QUrl;
        type PermissionType;
        type State;

        /// Returns whether this permission object is valid.
        #[cxx_name = "isValid"]
        fn is_valid(self: &QWebEnginePermission) -> bool;

        /// Returns the origin associated with the permission request.
        fn origin(self: &QWebEnginePermission) -> QUrl;

        /// Grants the permission request.
        fn grant(self: &QWebEnginePermission);

        /// Denies the permission request.
        fn deny(self: &QWebEnginePermission);

        /// Resets the permission decision.
        fn reset(self: &QWebEnginePermission);

        /// Returns the type of permission being requested.
        #[cxx_name = "permissionType"]
        fn permission_type(self: &QWebEnginePermission) -> PermissionType;

        /// Returns the state of the permission request.
        fn state(self: &QWebEnginePermission) -> State;
    }

    /// Represents the type of permission being requested.
    #[repr(u8)]
    #[derive(Debug)]
    enum PermissionType {
        Unsupported,
        MediaAudioCapture,
        MediaVideoCapture,
        MediaAudioVideoCapture,
        DesktopVideoCapture,
        DesktopAudioVideoCapture,
        MouseLock,
        Notifications,
        Geolocation,
        ClipboardReadWrite,
        LocalFontsAccess,
    }

    /// Represents the state of a permission request.
    #[repr(u8)]
    #[derive(Debug)]
    enum State {
        Invalid,
        Ask,
        Granted,
        Denied,
    }
}

/// Represents a permission request for web content.
#[derive(Clone, Debug)]
#[repr(C)]
pub struct QWebEnginePermission {
    _a: MaybeUninit<usize>,
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QWebEnginePermission {
    type Id = type_id!("QWebEnginePermission");
    type Kind = cxx::kind::Trivial;
}
