// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ButtonsType;
use crate::DialogFlags;
use crate::MessageDialog;
use crate::MessageType;
use crate::Widget;
use crate::Window;
use glib::object::{Cast, IsA};
use glib::translate::*;
use libc::c_char;
use std::ptr;

impl MessageDialog {
    #[doc(alias = "gtk_message_dialog_new")]
    pub fn new<T: IsA<Window>>(
        parent: Option<&T>,
        flags: DialogFlags,
        type_: MessageType,
        buttons: ButtonsType,
        message: &str,
    ) -> MessageDialog {
        assert_initialized_main_thread!();
        unsafe {
            let message: Stash<*const c_char, _> = message.to_glib_none();
            Widget::from_glib_none(ffi::gtk_message_dialog_new(
                parent.map(|p| p.as_ref()).to_glib_none().0,
                flags.into_glib(),
                type_.into_glib(),
                buttons.into_glib(),
                b"%s\0".as_ptr() as *const c_char,
                message.0,
                ptr::null::<c_char>(),
            ))
            .unsafe_cast()
        }
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: glib::object::IsA<crate::MessageDialog>> Sealed for T {}
}

pub trait MessageDialogExt: IsA<MessageDialog> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_message_dialog_format_secondary_markup")]
    fn set_secondary_markup(&self, message: Option<&str>) {
        match message {
            Some(m) => unsafe {
                let message: Stash<*const c_char, _> = m.to_glib_none();
                ffi::gtk_message_dialog_format_secondary_markup(
                    self.as_ref().to_glib_none().0,
                    b"%s\0".as_ptr() as *const c_char,
                    message.0,
                    ptr::null::<c_char>(),
                )
            },
            None => unsafe {
                ffi::gtk_message_dialog_format_secondary_markup(
                    self.as_ref().to_glib_none().0,
                    ptr::null::<c_char>(),
                )
            },
        }
    }

    #[doc(alias = "gtk_message_dialog_format_secondary_text")]
    fn set_secondary_text(&self, message: Option<&str>) {
        match message {
            Some(m) => unsafe {
                let message: Stash<*const c_char, _> = m.to_glib_none();
                ffi::gtk_message_dialog_format_secondary_text(
                    self.as_ref().to_glib_none().0,
                    b"%s\0".as_ptr() as *const c_char,
                    message.0,
                    ptr::null::<c_char>(),
                )
            },
            None => unsafe {
                ffi::gtk_message_dialog_format_secondary_text(
                    self.as_ref().to_glib_none().0,
                    ptr::null::<c_char>(),
                )
            },
        }
    }
}

impl<O: IsA<MessageDialog>> MessageDialogExt for O {}
