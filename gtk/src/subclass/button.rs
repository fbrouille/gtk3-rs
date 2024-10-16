// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::Cast;
use glib::subclass::prelude::*;
use glib::translate::*;

use super::bin::BinImpl;
use crate::Button;

pub trait ButtonImpl: ButtonImplExt + BinImpl {
    fn activate(&self) {
        self.parent_activate()
    }

    fn clicked(&self) {
        self.parent_clicked()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::ButtonImpl> Sealed for T {}
}

pub trait ButtonImplExt: ObjectSubclass + sealed::Sealed {
    fn parent_activate(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkButtonClass;
            if let Some(f) = (*parent_class).activate {
                f(self.obj().unsafe_cast_ref::<Button>().to_glib_none().0)
            }
        }
    }
    fn parent_clicked(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkButtonClass;
            if let Some(f) = (*parent_class).clicked {
                f(self.obj().unsafe_cast_ref::<Button>().to_glib_none().0)
            }
        }
    }
}

impl<T: ButtonImpl> ButtonImplExt for T {}

unsafe impl<T: ButtonImpl> IsSubclassable<T> for Button {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        if !crate::rt::is_initialized() {
            panic!("GTK has to be initialized first");
        }

        let klass = class.as_mut();
        klass.activate = Some(button_activate::<T>);
        klass.clicked = Some(button_clicked::<T>);
    }
}

unsafe extern "C" fn button_activate<T: ButtonImpl>(ptr: *mut ffi::GtkButton) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.activate()
}

unsafe extern "C" fn button_clicked<T: ButtonImpl>(ptr: *mut ffi::GtkButton) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.clicked()
}
