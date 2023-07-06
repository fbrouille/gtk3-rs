// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::StateType;
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "AtkStateSet")]
    pub struct StateSet(Object<ffi::AtkStateSet, ffi::AtkStateSetClass>);

    match fn {
        type_ => || ffi::atk_state_set_get_type(),
    }
}

impl StateSet {
    pub const NONE: Option<&'static StateSet> = None;

    #[doc(alias = "atk_state_set_new")]
    pub fn new() -> StateSet {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::atk_state_set_new()) }
    }
}

impl Default for StateSet {
    fn default() -> Self {
        Self::new()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::StateSet>> Sealed for T {}
}

pub trait StateSetExt: IsA<StateSet> + sealed::Sealed + 'static {
    #[doc(alias = "atk_state_set_add_state")]
    fn add_state(&self, type_: StateType) -> bool {
        unsafe {
            from_glib(ffi::atk_state_set_add_state(
                self.as_ref().to_glib_none().0,
                type_.into_glib(),
            ))
        }
    }

    //#[doc(alias = "atk_state_set_add_states")]
    //fn add_states(&self, types: /*Unimplemented*/&CArray TypeId { ns_id: 1, id: 68 }) {
    //    unsafe { TODO: call ffi:atk_state_set_add_states() }
    //}

    #[doc(alias = "atk_state_set_and_sets")]
    #[must_use]
    fn and_sets(&self, compare_set: &impl IsA<StateSet>) -> Option<StateSet> {
        unsafe {
            from_glib_full(ffi::atk_state_set_and_sets(
                self.as_ref().to_glib_none().0,
                compare_set.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "atk_state_set_clear_states")]
    fn clear_states(&self) {
        unsafe {
            ffi::atk_state_set_clear_states(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "atk_state_set_contains_state")]
    fn contains_state(&self, type_: StateType) -> bool {
        unsafe {
            from_glib(ffi::atk_state_set_contains_state(
                self.as_ref().to_glib_none().0,
                type_.into_glib(),
            ))
        }
    }

    //#[doc(alias = "atk_state_set_contains_states")]
    //fn contains_states(&self, types: /*Unimplemented*/&CArray TypeId { ns_id: 1, id: 68 }) -> bool {
    //    unsafe { TODO: call ffi:atk_state_set_contains_states() }
    //}

    #[doc(alias = "atk_state_set_is_empty")]
    fn is_empty(&self) -> bool {
        unsafe { from_glib(ffi::atk_state_set_is_empty(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "atk_state_set_or_sets")]
    #[must_use]
    fn or_sets(&self, compare_set: &impl IsA<StateSet>) -> Option<StateSet> {
        unsafe {
            from_glib_full(ffi::atk_state_set_or_sets(
                self.as_ref().to_glib_none().0,
                compare_set.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "atk_state_set_remove_state")]
    fn remove_state(&self, type_: StateType) -> bool {
        unsafe {
            from_glib(ffi::atk_state_set_remove_state(
                self.as_ref().to_glib_none().0,
                type_.into_glib(),
            ))
        }
    }

    #[doc(alias = "atk_state_set_xor_sets")]
    #[must_use]
    fn xor_sets(&self, compare_set: &impl IsA<StateSet>) -> Option<StateSet> {
        unsafe {
            from_glib_full(ffi::atk_state_set_xor_sets(
                self.as_ref().to_glib_none().0,
                compare_set.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<StateSet>> StateSetExt for O {}

impl fmt::Display for StateSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StateSet")
    }
}
