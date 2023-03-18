// Take a look at the license at the top of the repository in the LICENSE file.

use libc::c_char;

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{prelude::Cast, GString};

use super::cell_renderer::CellRendererImpl;

use crate::CellRendererText;

pub trait CellRendererTextImpl: CellRendererTextImplExt + CellRendererImpl {
    fn edited(&self, path: &str, new_text: &str) {
        self.parent_edited(path, new_text);
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::CellRendererTextImpl> Sealed for T {}
}

pub trait CellRendererTextImplExt: ObjectSubclass + sealed::Sealed {
    fn parent_edited(&self, path: &str, new_text: &str) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCellRendererTextClass;
            if let Some(f) = (*parent_class).edited {
                f(
                    self.obj()
                        .unsafe_cast_ref::<CellRendererText>()
                        .to_glib_none()
                        .0,
                    path.to_glib_none().0,
                    new_text.to_glib_none().0,
                )
            }
        }
    }
}

impl<T: CellRendererTextImpl> CellRendererTextImplExt for T {}

unsafe impl<T: CellRendererTextImpl> IsSubclassable<T> for CellRendererText {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        if !crate::rt::is_initialized() {
            panic!("GTK has to be initialized first");
        }

        let klass = class.as_mut();
        klass.edited = Some(cell_renderer_text_edited::<T>);
    }
}

unsafe extern "C" fn cell_renderer_text_edited<T: CellRendererTextImpl>(
    ptr: *mut ffi::GtkCellRendererText,
    path: *const c_char,
    new_text: *const c_char,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.edited(
        &GString::from_glib_borrow(path),
        &GString::from_glib_borrow(new_text),
    )
}
