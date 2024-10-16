// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ColorChooser;
use crate::Orientation;
use gdk::RGBA;
use glib::object::IsA;
use glib::translate::*;
use libc::c_int;

mod sealed {
    pub trait Sealed {}
    impl<T: glib::object::IsA<crate::ColorChooser>> Sealed for T {}
}

pub trait ColorChooserExtManual: IsA<ColorChooser> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_color_chooser_add_palette")]
    fn add_palette(&self, orientation: Orientation, colors_per_line: i32, colors: &[RGBA]) {
        unsafe {
            ffi::gtk_color_chooser_add_palette(
                self.as_ref().to_glib_none().0,
                orientation.into_glib(),
                colors_per_line,
                colors.len() as c_int,
                colors.as_ptr() as *mut gdk::ffi::GdkRGBA,
            )
        }
    }
}

impl<O: IsA<ColorChooser>> ColorChooserExtManual for O {}
