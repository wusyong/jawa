use std::pin::Pin;

use crate::WidgetPtr;
use cxx_qt_lib::{QFlags, unsafe_impl_qflag};
pub use ffi::{QMessageBox, StandardButton};

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++Qt" {
        include!("cxx-qt-widgets/qmessagebox.h");
        type QWidget = crate::QWidget;
        type QString = cxx_qt_lib::QString;

        /// Message box dialog for displaying information to the user.
        #[qobject]
        #[base = QWidget]
        type QMessageBox;

        /// Sets the primary text shown in the message box.
        #[cxx_name = "setText"]
        fn set_text(self: Pin<&mut QMessageBox>, text: &QString);

        /// Sets the informative text shown below the primary text.
        #[cxx_name = "setInformativeText"]
        fn set_informative_text(self: Pin<&mut QMessageBox>, text: &QString);

        /// Sets the detailed text shown in the "Show Details" area.
        #[cxx_name = "setDetailedText"]
        fn set_detailed_text(self: Pin<&mut QMessageBox>, text: &QString);

        /// Shows the widget and its child widgets.
        fn show(self: Pin<&mut QMessageBox>);

        /// Executes the dialog modally and returns the result.
        #[cxx_name = "exec"]
        fn exec(self: Pin<&mut QMessageBox>) -> i32;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++Qt" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[cxx_name = "make_unique"]
        fn new_message_box() -> UniquePtr<QMessageBox>;

        #[doc(hidden)]
        #[cxx_name = "new_ptr"]
        unsafe fn new_message_box_with_parent(parent: *mut QWidget) -> *mut QMessageBox;
    }

    /// Represents the state of a permission request.
    #[repr(u32)]
    #[derive(Debug)]
    // #[namespace = "rust::cxxqtlib1"]
    enum StandardButton {
        // keep this in sync with QDialogButtonBox::StandardButton and QPlatformDialogHelper::StandardButton
        NoButton = 0x00000000,
        Ok = 0x00000400,
        Save = 0x00000800,
        SaveAll = 0x00001000,
        Open = 0x00002000,
        Yes = 0x00004000,
        YesToAll = 0x00008000,
        No = 0x00010000,
        NoToAll = 0x00020000,
        Abort = 0x00040000,
        Retry = 0x00080000,
        Ignore = 0x00100000,
        Close = 0x00200000,
        Cancel = 0x00400000,
        Discard = 0x00800000,
        Help = 0x01000000,
        Apply = 0x02000000,
        Reset = 0x04000000,
        RestoreDefaults = 0x08000000,
    }

    unsafe extern "C++" {
        type StandardButtons = super::StandardButtons;
        type StandardButton;

        /// Sets the standard buttons to be displayed in the message box.
        #[cxx_name = "setStandardButtons"]
        fn set_standard_buttons(self: Pin<&mut QMessageBox>, buttons: StandardButtons);

        /// Sets the default button for the message box.
        #[cxx_name = "setDefaultButton"]
        fn set_default_button(self: Pin<&mut QMessageBox>, button: StandardButton);

    }
}

impl ffi::QMessageBox {
    /// Creates a new message box without a parent.
    pub fn new() -> WidgetPtr<Self> {
        ffi::new_message_box().into()
    }

    /// Creates a new message box with a parent.
    pub fn new_with_parent(parent: Pin<&mut crate::QWidget>) -> WidgetPtr<Self> {
        unsafe { ffi::new_message_box_with_parent(parent.get_unchecked_mut()).into() }
    }
}

/// [`QFlags`] of [`StandardButton`].
pub type StandardButtons = QFlags<StandardButton>;

unsafe_impl_qflag!(StandardButton, "StandardButtons", u32);
