// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Buildable;
use crate::CellArea;
use crate::CellLayout;
use crate::CellRenderer;
use crate::Orientable;
use crate::Orientation;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct CellAreaBox(Object<ffi::GtkCellAreaBox>) @extends CellArea, @implements Buildable, CellLayout, Orientable;

    match fn {
        type_ => || ffi::gtk_cell_area_box_get_type(),
    }
}

impl CellAreaBox {
    #[doc(alias = "gtk_cell_area_box_new")]
    pub fn new() -> CellAreaBox {
        assert_initialized_main_thread!();
        unsafe { CellArea::from_glib_none(ffi::gtk_cell_area_box_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`CellAreaBox`]
    /// This method returns an instance of [`CellAreaBoxBuilder`] which can be used to create a [`CellAreaBox`].
    pub fn builder() -> CellAreaBoxBuilder {
        CellAreaBoxBuilder::default()
    }

    #[doc(alias = "gtk_cell_area_box_get_spacing")]
    #[doc(alias = "get_spacing")]
    pub fn spacing(&self) -> i32 {
        unsafe { ffi::gtk_cell_area_box_get_spacing(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_cell_area_box_pack_end")]
    pub fn pack_end<P: IsA<CellRenderer>>(
        &self,
        renderer: &P,
        expand: bool,
        align: bool,
        fixed: bool,
    ) {
        unsafe {
            ffi::gtk_cell_area_box_pack_end(
                self.to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                expand.into_glib(),
                align.into_glib(),
                fixed.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_cell_area_box_pack_start")]
    pub fn pack_start<P: IsA<CellRenderer>>(
        &self,
        renderer: &P,
        expand: bool,
        align: bool,
        fixed: bool,
    ) {
        unsafe {
            ffi::gtk_cell_area_box_pack_start(
                self.to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                expand.into_glib(),
                align.into_glib(),
                fixed.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_cell_area_box_set_spacing")]
    pub fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_cell_area_box_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    #[doc(alias = "spacing")]
    pub fn connect_spacing_notify<F: Fn(&CellAreaBox) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_spacing_trampoline<F: Fn(&CellAreaBox) + 'static>(
            this: *mut ffi::GtkCellAreaBox,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::spacing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_spacing_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for CellAreaBox {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`CellAreaBox`].
pub struct CellAreaBoxBuilder {
    spacing: Option<i32>,
    focus_cell: Option<CellRenderer>,
    orientation: Option<Orientation>,
}

impl CellAreaBoxBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`CellAreaBoxBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`CellAreaBox`].
    pub fn build(self) -> CellAreaBox {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref spacing) = self.spacing {
            properties.push(("spacing", spacing));
        }
        if let Some(ref focus_cell) = self.focus_cell {
            properties.push(("focus-cell", focus_cell));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        glib::Object::new::<CellAreaBox>(&properties)
            .expect("Failed to create an instance of CellAreaBox")
    }

    pub fn spacing(mut self, spacing: i32) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn focus_cell<P: IsA<CellRenderer>>(mut self, focus_cell: &P) -> Self {
        self.focus_cell = Some(focus_cell.clone().upcast());
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

impl fmt::Display for CellAreaBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CellAreaBox")
    }
}
