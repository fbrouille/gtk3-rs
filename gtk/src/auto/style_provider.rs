// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{StateFlags, WidgetPath};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkStyleProvider")]
    pub struct StyleProvider(Interface<ffi::GtkStyleProvider, ffi::GtkStyleProviderIface>);

    match fn {
        type_ => || ffi::gtk_style_provider_get_type(),
    }
}

impl StyleProvider {
    pub const NONE: Option<&'static StyleProvider> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::StyleProvider>> Sealed for T {}
}

pub trait StyleProviderExt: IsA<StyleProvider> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_style_provider_get_style_property")]
    #[doc(alias = "get_style_property")]
    fn style_property(
        &self,
        path: &WidgetPath,
        state: StateFlags,
        pspec: impl AsRef<glib::ParamSpec>,
    ) -> Option<glib::Value> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let ret = from_glib(ffi::gtk_style_provider_get_style_property(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                state.into_glib(),
                pspec.as_ref().to_glib_none().0,
                value.to_glib_none_mut().0,
            ));
            if ret {
                Some(value)
            } else {
                None
            }
        }
    }
}

impl<O: IsA<StyleProvider>> StyleProviderExt for O {}

impl fmt::Display for StyleProvider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StyleProvider")
    }
}
