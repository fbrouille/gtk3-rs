// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Adjustment, Align, Bin, Buildable, Container, ResizeMode, Scrollable, ScrollablePolicy,
    ShadowType, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkViewport")]
    pub struct Viewport(Object<ffi::GtkViewport, ffi::GtkViewportClass>) @extends Bin, Container, Widget, @implements Buildable, Scrollable;

    match fn {
        type_ => || ffi::gtk_viewport_get_type(),
    }
}

impl Viewport {
    pub const NONE: Option<&'static Viewport> = None;

    #[doc(alias = "gtk_viewport_new")]
    pub fn new(
        hadjustment: Option<&impl IsA<Adjustment>>,
        vadjustment: Option<&impl IsA<Adjustment>>,
    ) -> Viewport {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_viewport_new(
                hadjustment.map(|p| p.as_ref()).to_glib_none().0,
                vadjustment.map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Viewport`] objects.
    ///
    /// This method returns an instance of [`ViewportBuilder`](crate::builders::ViewportBuilder) which can be used to create [`Viewport`] objects.
    pub fn builder() -> ViewportBuilder {
        ViewportBuilder::new()
    }
}

impl Default for Viewport {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Viewport`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ViewportBuilder {
    builder: glib::object::ObjectBuilder<'static, Viewport>,
}

impl ViewportBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn shadow_type(self, shadow_type: ShadowType) -> Self {
        Self {
            builder: self.builder.property("shadow-type", shadow_type),
        }
    }

    pub fn border_width(self, border_width: u32) -> Self {
        Self {
            builder: self.builder.property("border-width", border_width),
        }
    }

    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn resize_mode(self, resize_mode: ResizeMode) -> Self {
        Self {
            builder: self.builder.property("resize-mode", resize_mode),
        }
    }

    pub fn app_paintable(self, app_paintable: bool) -> Self {
        Self {
            builder: self.builder.property("app-paintable", app_paintable),
        }
    }

    pub fn can_default(self, can_default: bool) -> Self {
        Self {
            builder: self.builder.property("can-default", can_default),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn events(self, events: gdk::EventMask) -> Self {
        Self {
            builder: self.builder.property("events", events),
        }
    }

    pub fn expand(self, expand: bool) -> Self {
        Self {
            builder: self.builder.property("expand", expand),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_default(self, has_default: bool) -> Self {
        Self {
            builder: self.builder.property("has-default", has_default),
        }
    }

    pub fn has_focus(self, has_focus: bool) -> Self {
        Self {
            builder: self.builder.property("has-focus", has_focus),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn is_focus(self, is_focus: bool) -> Self {
        Self {
            builder: self.builder.property("is-focus", is_focus),
        }
    }

    pub fn margin(self, margin: i32) -> Self {
        Self {
            builder: self.builder.property("margin", margin),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn no_show_all(self, no_show_all: bool) -> Self {
        Self {
            builder: self.builder.property("no-show-all", no_show_all),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn parent(self, parent: &impl IsA<Container>) -> Self {
        Self {
            builder: self.builder.property("parent", parent.clone().upcast()),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn hadjustment(self, hadjustment: &impl IsA<Adjustment>) -> Self {
        Self {
            builder: self
                .builder
                .property("hadjustment", hadjustment.clone().upcast()),
        }
    }

    pub fn hscroll_policy(self, hscroll_policy: ScrollablePolicy) -> Self {
        Self {
            builder: self.builder.property("hscroll-policy", hscroll_policy),
        }
    }

    pub fn vadjustment(self, vadjustment: &impl IsA<Adjustment>) -> Self {
        Self {
            builder: self
                .builder
                .property("vadjustment", vadjustment.clone().upcast()),
        }
    }

    pub fn vscroll_policy(self, vscroll_policy: ScrollablePolicy) -> Self {
        Self {
            builder: self.builder.property("vscroll-policy", vscroll_policy),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Viewport`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Viewport {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Viewport>> Sealed for T {}
}

pub trait ViewportExt: IsA<Viewport> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_viewport_get_bin_window")]
    #[doc(alias = "get_bin_window")]
    fn bin_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_viewport_get_bin_window(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_viewport_get_shadow_type")]
    #[doc(alias = "get_shadow_type")]
    fn shadow_type(&self) -> ShadowType {
        unsafe {
            from_glib(ffi::gtk_viewport_get_shadow_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_viewport_get_view_window")]
    #[doc(alias = "get_view_window")]
    fn view_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_viewport_get_view_window(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_viewport_set_shadow_type")]
    fn set_shadow_type(&self, type_: ShadowType) {
        unsafe {
            ffi::gtk_viewport_set_shadow_type(self.as_ref().to_glib_none().0, type_.into_glib());
        }
    }

    #[doc(alias = "shadow-type")]
    fn connect_shadow_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_shadow_type_trampoline<
            P: IsA<Viewport>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkViewport,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Viewport::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::shadow-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_shadow_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Viewport>> ViewportExt for O {}

impl fmt::Display for Viewport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Viewport")
    }
}
