// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Buildable;
use crate::Filter;
use glib::object::Cast;
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
    pub struct FileFilter(Object<ffi::GtkFileFilter>) @extends Filter, @implements Buildable;

    match fn {
        type_ => || ffi::gtk_file_filter_get_type(),
    }
}

impl FileFilter {
    #[doc(alias = "gtk_file_filter_new")]
    pub fn new() -> FileFilter {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_file_filter_new()) }
    }

    #[doc(alias = "gtk_file_filter_new_from_gvariant")]
    #[doc(alias = "new_from_gvariant")]
    pub fn from_gvariant(variant: &glib::Variant) -> FileFilter {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_file_filter_new_from_gvariant(
                variant.to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`FileFilter`]
    /// This method returns an instance of [`FileFilterBuilder`] which can be used to create a [`FileFilter`].
    pub fn builder() -> FileFilterBuilder {
        FileFilterBuilder::default()
    }

    #[doc(alias = "gtk_file_filter_add_mime_type")]
    pub fn add_mime_type(&self, mime_type: &str) {
        unsafe {
            ffi::gtk_file_filter_add_mime_type(self.to_glib_none().0, mime_type.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_file_filter_add_pattern")]
    pub fn add_pattern(&self, pattern: &str) {
        unsafe {
            ffi::gtk_file_filter_add_pattern(self.to_glib_none().0, pattern.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_file_filter_add_pixbuf_formats")]
    pub fn add_pixbuf_formats(&self) {
        unsafe {
            ffi::gtk_file_filter_add_pixbuf_formats(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_file_filter_get_attributes")]
    #[doc(alias = "get_attributes")]
    pub fn attributes(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_file_filter_get_attributes(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_file_filter_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_file_filter_get_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_file_filter_set_name")]
    pub fn set_name(&self, name: Option<&str>) {
        unsafe {
            ffi::gtk_file_filter_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_file_filter_to_gvariant")]
    pub fn to_gvariant(&self) -> Option<glib::Variant> {
        unsafe { from_glib_none(ffi::gtk_file_filter_to_gvariant(self.to_glib_none().0)) }
    }

    #[doc(alias = "name")]
    pub fn connect_name_notify<F: Fn(&FileFilter) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<F: Fn(&FileFilter) + 'static>(
            this: *mut ffi::GtkFileFilter,
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
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for FileFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`FileFilter`].
pub struct FileFilterBuilder {
    name: Option<String>,
}

impl FileFilterBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`FileFilterBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`FileFilter`].
    pub fn build(self) -> FileFilter {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        glib::Object::new::<FileFilter>(&properties)
            .expect("Failed to create an instance of FileFilter")
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
}

impl fmt::Display for FileFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FileFilter")
    }
}
