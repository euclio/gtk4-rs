// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use glib::GString;
use glib::translate::*;
use gtk_sys;
use std::mem;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SelectionData(Boxed<gtk_sys::GtkSelectionData>);

    match fn {
        copy => |ptr| gtk_sys::gtk_selection_data_copy(mut_override(ptr)),
        free => |ptr| gtk_sys::gtk_selection_data_free(ptr),
        get_type => || gtk_sys::gtk_selection_data_get_type(),
    }
}

impl SelectionData {
    //pub fn get_data_type(&self) -> /*Ignored*/Option<gdk::Atom> {
    //    unsafe { TODO: call gtk_sys:gtk_selection_data_get_data_type() }
    //}

    pub fn get_data_with_length(&self) -> Vec<u8> {
        unsafe {
            let mut length = mem::uninitialized();
            let ret = FromGlibContainer::from_glib_none_num(gtk_sys::gtk_selection_data_get_data_with_length(self.to_glib_none().0, &mut length), length as usize);
            ret
        }
    }

    pub fn get_display(&self) -> Option<gdk::Display> {
        unsafe {
            from_glib_none(gtk_sys::gtk_selection_data_get_display(self.to_glib_none().0))
        }
    }

    pub fn get_format(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_selection_data_get_format(self.to_glib_none().0)
        }
    }

    pub fn get_length(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_selection_data_get_length(self.to_glib_none().0)
        }
    }

    //pub fn get_pixbuf(&self) -> /*Ignored*/Option<gdk_pixbuf::Pixbuf> {
    //    unsafe { TODO: call gtk_sys:gtk_selection_data_get_pixbuf() }
    //}

    //pub fn get_target(&self) -> /*Ignored*/Option<gdk::Atom> {
    //    unsafe { TODO: call gtk_sys:gtk_selection_data_get_target() }
    //}

    //pub fn get_targets(&self, targets: /*Ignored*/Vec<gdk::Atom>) -> Option<i32> {
    //    unsafe { TODO: call gtk_sys:gtk_selection_data_get_targets() }
    //}

    pub fn get_text(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_sys::gtk_selection_data_get_text(self.to_glib_none().0))
        }
    }

    //pub fn get_texture(&self) -> /*Ignored*/Option<gdk::Texture> {
    //    unsafe { TODO: call gtk_sys:gtk_selection_data_get_texture() }
    //}

    pub fn get_uris(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gtk_sys::gtk_selection_data_get_uris(self.to_glib_none().0))
        }
    }

    //pub fn set(&mut self, type_: /*Ignored*/&gdk::Atom, format: i32, data: &[u8]) {
    //    unsafe { TODO: call gtk_sys:gtk_selection_data_set() }
    //}

    //pub fn set_pixbuf(&mut self, pixbuf: /*Ignored*/&gdk_pixbuf::Pixbuf) -> bool {
    //    unsafe { TODO: call gtk_sys:gtk_selection_data_set_pixbuf() }
    //}

    pub fn set_text(&mut self, str: &str) -> bool {
        let len = str.len() as i32;
        unsafe {
            from_glib(gtk_sys::gtk_selection_data_set_text(self.to_glib_none_mut().0, str.to_glib_none().0, len))
        }
    }

    //pub fn set_texture(&mut self, texture: /*Ignored*/&gdk::Texture) -> bool {
    //    unsafe { TODO: call gtk_sys:gtk_selection_data_set_texture() }
    //}

    pub fn set_uris(&mut self, uris: &[&str]) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_selection_data_set_uris(self.to_glib_none_mut().0, uris.to_glib_none().0))
        }
    }

    pub fn targets_include_image(&self, writable: bool) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_selection_data_targets_include_image(self.to_glib_none().0, writable.to_glib()))
        }
    }

    pub fn targets_include_text(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_selection_data_targets_include_text(self.to_glib_none().0))
        }
    }

    pub fn targets_include_uri(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_selection_data_targets_include_uri(self.to_glib_none().0))
        }
    }
}
