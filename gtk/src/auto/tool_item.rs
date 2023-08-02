// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Align, Bin, Buildable, Container, IconSize, Orientation, ReliefStyle, ResizeMode, SizeGroup,
    ToolbarStyle, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkToolItem")]
    pub struct ToolItem(Object<ffi::GtkToolItem, ffi::GtkToolItemClass>) @extends Bin, Container, Widget, @implements Buildable;

    match fn {
        type_ => || ffi::gtk_tool_item_get_type(),
    }
}

impl ToolItem {
    pub const NONE: Option<&'static ToolItem> = None;

    #[doc(alias = "gtk_tool_item_new")]
    pub fn new() -> ToolItem {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_tool_item_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ToolItem`] objects.
    ///
    /// This method returns an instance of [`ToolItemBuilder`](crate::builders::ToolItemBuilder) which can be used to create [`ToolItem`] objects.
    pub fn builder() -> ToolItemBuilder {
        ToolItemBuilder::new()
    }
}

impl Default for ToolItem {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ToolItem`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ToolItemBuilder {
    builder: glib::object::ObjectBuilder<'static, ToolItem>,
}

impl ToolItemBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn is_important(self, is_important: bool) -> Self {
        Self {
            builder: self.builder.property("is-important", is_important),
        }
    }

    pub fn visible_horizontal(self, visible_horizontal: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("visible-horizontal", visible_horizontal),
        }
    }

    pub fn visible_vertical(self, visible_vertical: bool) -> Self {
        Self {
            builder: self.builder.property("visible-vertical", visible_vertical),
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

    // rustdoc-stripper-ignore-next
    /// Build the [`ToolItem`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ToolItem {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ToolItem>> Sealed for T {}
}

pub trait ToolItemExt: IsA<ToolItem> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_tool_item_get_ellipsize_mode")]
    #[doc(alias = "get_ellipsize_mode")]
    fn ellipsize_mode(&self) -> pango::EllipsizeMode {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_ellipsize_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tool_item_get_expand")]
    #[doc(alias = "get_expand")]
    fn expands(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_expand(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tool_item_get_homogeneous")]
    #[doc(alias = "get_homogeneous")]
    fn is_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_homogeneous(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tool_item_get_icon_size")]
    #[doc(alias = "get_icon_size")]
    fn icon_size(&self) -> IconSize {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_icon_size(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tool_item_get_is_important")]
    #[doc(alias = "get_is_important")]
    fn is_important(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_is_important(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tool_item_get_orientation")]
    #[doc(alias = "get_orientation")]
    fn orientation(&self) -> Orientation {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_orientation(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tool_item_get_proxy_menu_item")]
    #[doc(alias = "get_proxy_menu_item")]
    fn proxy_menu_item(&self, menu_item_id: &str) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_get_proxy_menu_item(
                self.as_ref().to_glib_none().0,
                menu_item_id.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tool_item_get_relief_style")]
    #[doc(alias = "get_relief_style")]
    fn relief_style(&self) -> ReliefStyle {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_relief_style(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tool_item_get_text_alignment")]
    #[doc(alias = "get_text_alignment")]
    fn text_alignment(&self) -> f32 {
        unsafe { ffi::gtk_tool_item_get_text_alignment(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gtk_tool_item_get_text_orientation")]
    #[doc(alias = "get_text_orientation")]
    fn text_orientation(&self) -> Orientation {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_text_orientation(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tool_item_get_text_size_group")]
    #[doc(alias = "get_text_size_group")]
    fn text_size_group(&self) -> Option<SizeGroup> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_get_text_size_group(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tool_item_get_toolbar_style")]
    #[doc(alias = "get_toolbar_style")]
    fn toolbar_style(&self) -> ToolbarStyle {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_toolbar_style(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tool_item_get_use_drag_window")]
    #[doc(alias = "get_use_drag_window")]
    fn uses_drag_window(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_use_drag_window(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tool_item_get_visible_horizontal")]
    #[doc(alias = "get_visible_horizontal")]
    fn is_visible_horizontal(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_visible_horizontal(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tool_item_get_visible_vertical")]
    #[doc(alias = "get_visible_vertical")]
    fn is_visible_vertical(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_get_visible_vertical(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tool_item_rebuild_menu")]
    fn rebuild_menu(&self) {
        unsafe {
            ffi::gtk_tool_item_rebuild_menu(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_tool_item_retrieve_proxy_menu_item")]
    fn retrieve_proxy_menu_item(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_retrieve_proxy_menu_item(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tool_item_set_expand")]
    fn set_expand(&self, expand: bool) {
        unsafe {
            ffi::gtk_tool_item_set_expand(self.as_ref().to_glib_none().0, expand.into_glib());
        }
    }

    #[doc(alias = "gtk_tool_item_set_homogeneous")]
    fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_tool_item_set_homogeneous(
                self.as_ref().to_glib_none().0,
                homogeneous.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_tool_item_set_is_important")]
    fn set_is_important(&self, is_important: bool) {
        unsafe {
            ffi::gtk_tool_item_set_is_important(
                self.as_ref().to_glib_none().0,
                is_important.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_tool_item_set_proxy_menu_item")]
    fn set_proxy_menu_item(&self, menu_item_id: &str, menu_item: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_tool_item_set_proxy_menu_item(
                self.as_ref().to_glib_none().0,
                menu_item_id.to_glib_none().0,
                menu_item.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_tool_item_set_tooltip_markup")]
    fn set_tooltip_markup(&self, markup: &str) {
        unsafe {
            ffi::gtk_tool_item_set_tooltip_markup(
                self.as_ref().to_glib_none().0,
                markup.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_tool_item_set_tooltip_text")]
    fn set_tooltip_text(&self, text: &str) {
        unsafe {
            ffi::gtk_tool_item_set_tooltip_text(
                self.as_ref().to_glib_none().0,
                text.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_tool_item_set_use_drag_window")]
    fn set_use_drag_window(&self, use_drag_window: bool) {
        unsafe {
            ffi::gtk_tool_item_set_use_drag_window(
                self.as_ref().to_glib_none().0,
                use_drag_window.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_tool_item_set_visible_horizontal")]
    fn set_visible_horizontal(&self, visible_horizontal: bool) {
        unsafe {
            ffi::gtk_tool_item_set_visible_horizontal(
                self.as_ref().to_glib_none().0,
                visible_horizontal.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_tool_item_set_visible_vertical")]
    fn set_visible_vertical(&self, visible_vertical: bool) {
        unsafe {
            ffi::gtk_tool_item_set_visible_vertical(
                self.as_ref().to_glib_none().0,
                visible_vertical.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_tool_item_toolbar_reconfigured")]
    fn toolbar_reconfigured(&self) {
        unsafe {
            ffi::gtk_tool_item_toolbar_reconfigured(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "create-menu-proxy")]
    fn connect_create_menu_proxy<F: Fn(&Self) -> glib::Propagation + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn create_menu_proxy_trampoline<
            P: IsA<ToolItem>,
            F: Fn(&P) -> glib::Propagation + 'static,
        >(
            this: *mut ffi::GtkToolItem,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(ToolItem::from_glib_borrow(this).unsafe_cast_ref()).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"create-menu-proxy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    create_menu_proxy_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "toolbar-reconfigured")]
    fn connect_toolbar_reconfigured<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn toolbar_reconfigured_trampoline<
            P: IsA<ToolItem>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkToolItem,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ToolItem::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"toolbar-reconfigured\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    toolbar_reconfigured_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "is-important")]
    fn connect_is_important_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_important_trampoline<
            P: IsA<ToolItem>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkToolItem,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ToolItem::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-important\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_important_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "visible-horizontal")]
    fn connect_visible_horizontal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_horizontal_trampoline<
            P: IsA<ToolItem>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkToolItem,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ToolItem::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::visible-horizontal\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_horizontal_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "visible-vertical")]
    fn connect_visible_vertical_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_vertical_trampoline<
            P: IsA<ToolItem>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkToolItem,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ToolItem::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::visible-vertical\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_vertical_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<ToolItem>> ToolItemExt for O {}

impl fmt::Display for ToolItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ToolItem")
    }
}
