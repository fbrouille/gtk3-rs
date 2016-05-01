// This file was generated by gir (324239f) from gir-files (11e0e6d)
// DO NOT EDIT

use CellEditable;
use Editable;
use Entry;
use Widget;
use ffi;
#[cfg(feature = "v3_10")]
use ffi::GtkSearchEntry;
use glib::object::Downcast;
#[cfg(feature = "v3_10")]
use glib::signal::connect;
use glib::translate::*;
#[cfg(feature = "v3_10")]
use glib_ffi::gpointer;
#[cfg(feature = "v3_10")]
use std::boxed::Box as Box_;
#[cfg(feature = "v3_10")]
use std::mem::transmute;

glib_wrapper! {
    pub struct SearchEntry(Object<ffi::GtkSearchEntry>): Entry, Widget, CellEditable, Editable;

    match fn {
        get_type => || ffi::gtk_search_entry_get_type(),
    }
}

impl SearchEntry {
    #[cfg(feature = "v3_6")]
    pub fn new() -> SearchEntry {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_search_entry_new()).downcast_unchecked()
        }
    }

    //#[cfg(feature = "v3_16")]
    //pub fn handle_event(&self, event: /*Unknown conversion*//*Unimplemented*/Event) -> bool {
    //    unsafe { TODO: call ffi::gtk_search_entry_handle_event() }
    //}

    #[cfg(feature = "v3_16")]
    pub fn connect_next_match<F: Fn(&SearchEntry) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&SearchEntry) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "next-match",
                transmute(next_match_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn connect_previous_match<F: Fn(&SearchEntry) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&SearchEntry) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "previous-match",
                transmute(previous_match_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn connect_search_changed<F: Fn(&SearchEntry) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&SearchEntry) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "search-changed",
                transmute(search_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn connect_stop_search<F: Fn(&SearchEntry) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&SearchEntry) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "stop-search",
                transmute(stop_search_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_16")]
unsafe extern "C" fn next_match_trampoline(this: *mut GtkSearchEntry, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&SearchEntry) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

#[cfg(feature = "v3_16")]
unsafe extern "C" fn previous_match_trampoline(this: *mut GtkSearchEntry, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&SearchEntry) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

#[cfg(feature = "v3_10")]
unsafe extern "C" fn search_changed_trampoline(this: *mut GtkSearchEntry, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&SearchEntry) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

#[cfg(feature = "v3_16")]
unsafe extern "C" fn stop_search_trampoline(this: *mut GtkSearchEntry, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&SearchEntry) + 'static> = transmute(f);
    f(&from_glib_none(this))
}
