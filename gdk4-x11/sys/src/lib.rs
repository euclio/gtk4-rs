// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal
)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

use gdk4_sys as gdk;
use glib_sys as glib;

mod manual;

pub use manual::*;

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, time_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type GdkX11DeviceType = c_int;
pub const GDK_X11_DEVICE_TYPE_LOGICAL: GdkX11DeviceType = 0;
pub const GDK_X11_DEVICE_TYPE_PHYSICAL: GdkX11DeviceType = 1;
pub const GDK_X11_DEVICE_TYPE_FLOATING: GdkX11DeviceType = 2;

// Records
#[repr(C)]
pub struct _GdkX11AppLaunchContextClass(c_void);

pub type GdkX11AppLaunchContextClass = *mut _GdkX11AppLaunchContextClass;

#[repr(C)]
pub struct _GdkX11DeviceManagerXI2Class(c_void);

pub type GdkX11DeviceManagerXI2Class = *mut _GdkX11DeviceManagerXI2Class;

#[repr(C)]
pub struct _GdkX11DeviceXI2Class(c_void);

pub type GdkX11DeviceXI2Class = *mut _GdkX11DeviceXI2Class;

#[repr(C)]
pub struct _GdkX11DisplayClass(c_void);

pub type GdkX11DisplayClass = *mut _GdkX11DisplayClass;

#[repr(C)]
pub struct _GdkX11DragClass(c_void);

pub type GdkX11DragClass = *mut _GdkX11DragClass;

#[repr(C)]
pub struct _GdkX11GLContextClass(c_void);

pub type GdkX11GLContextClass = *mut _GdkX11GLContextClass;

#[repr(C)]
pub struct _GdkX11MonitorClass(c_void);

pub type GdkX11MonitorClass = *mut _GdkX11MonitorClass;

#[repr(C)]
pub struct _GdkX11ScreenClass(c_void);

pub type GdkX11ScreenClass = *mut _GdkX11ScreenClass;

#[repr(C)]
pub struct _GdkX11SurfaceClass(c_void);

pub type GdkX11SurfaceClass = *mut _GdkX11SurfaceClass;

// Classes
#[repr(C)]
pub struct GdkX11AppLaunchContext(c_void);

impl ::std::fmt::Debug for GdkX11AppLaunchContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11AppLaunchContext @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct GdkX11DeviceManagerXI2(c_void);

impl ::std::fmt::Debug for GdkX11DeviceManagerXI2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11DeviceManagerXI2 @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct GdkX11DeviceXI2(c_void);

impl ::std::fmt::Debug for GdkX11DeviceXI2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11DeviceXI2 @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct GdkX11Display(c_void);

impl ::std::fmt::Debug for GdkX11Display {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11Display @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct GdkX11Drag(c_void);

impl ::std::fmt::Debug for GdkX11Drag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11Drag @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct GdkX11GLContext(c_void);

impl ::std::fmt::Debug for GdkX11GLContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11GLContext @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct GdkX11Monitor(c_void);

impl ::std::fmt::Debug for GdkX11Monitor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11Monitor @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct GdkX11Screen(c_void);

impl ::std::fmt::Debug for GdkX11Screen {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11Screen @ {:?}", self as *const _))
            .finish()
    }
}

#[repr(C)]
pub struct GdkX11Surface(c_void);

impl ::std::fmt::Debug for GdkX11Surface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GdkX11Surface @ {:?}", self as *const _))
            .finish()
    }
}

#[link(name = "gtk-4")]
extern "C" {

    //=========================================================================
    // GdkX11AppLaunchContext
    //=========================================================================
    pub fn gdk_x11_app_launch_context_get_type() -> GType;

    //=========================================================================
    // GdkX11DeviceManagerXI2
    //=========================================================================
    pub fn gdk_x11_device_manager_xi2_get_type() -> GType;

    //=========================================================================
    // GdkX11DeviceXI2
    //=========================================================================
    pub fn gdk_x11_device_xi2_get_type() -> GType;
    pub fn gdk_x11_device_xi2_get_device_type(device: *mut GdkX11DeviceXI2) -> GdkX11DeviceType;
    pub fn gdk_x11_device_xi2_set_device_type(
        device: *mut GdkX11DeviceXI2,
        type_: GdkX11DeviceType,
    );

    //=========================================================================
    // GdkX11Display
    //=========================================================================
    pub fn gdk_x11_display_get_type() -> GType;
    pub fn gdk_x11_display_open(display_name: *const c_char) -> *mut gdk::GdkDisplay;
    pub fn gdk_x11_display_set_program_class(
        display: *mut gdk::GdkDisplay,
        program_class: *const c_char,
    );
    pub fn gdk_x11_display_broadcast_startup_message(
        display: *mut GdkX11Display,
        message_type: *const c_char,
        ...
    );
    pub fn gdk_x11_display_error_trap_pop(display: *mut GdkX11Display) -> c_int;
    pub fn gdk_x11_display_error_trap_pop_ignored(display: *mut GdkX11Display);
    pub fn gdk_x11_display_error_trap_push(display: *mut GdkX11Display);
    pub fn gdk_x11_display_get_default_group(display: *mut GdkX11Display) -> *mut gdk::GdkSurface;
    pub fn gdk_x11_display_get_glx_version(
        display: *mut GdkX11Display,
        major: *mut c_int,
        minor: *mut c_int,
    ) -> gboolean;
    pub fn gdk_x11_display_get_primary_monitor(display: *mut GdkX11Display)
        -> *mut gdk::GdkMonitor;
    pub fn gdk_x11_display_get_screen(display: *mut GdkX11Display) -> *mut GdkX11Screen;
    pub fn gdk_x11_display_get_startup_notification_id(
        display: *mut GdkX11Display,
    ) -> *const c_char;
    pub fn gdk_x11_display_get_user_time(display: *mut GdkX11Display) -> u32;
    pub fn gdk_x11_display_get_xcursor(
        display: *mut GdkX11Display,
        cursor: *mut gdk::GdkCursor,
    ) -> xlib::Cursor;
    pub fn gdk_x11_display_get_xdisplay(display: *mut GdkX11Display) -> *mut xlib::Display;
    pub fn gdk_x11_display_get_xrootwindow(display: *mut GdkX11Display) -> xlib::Window;
    pub fn gdk_x11_display_get_xscreen(display: *mut GdkX11Display) -> *mut xlib::Screen;
    pub fn gdk_x11_display_grab(display: *mut GdkX11Display);
    pub fn gdk_x11_display_set_cursor_theme(
        display: *mut GdkX11Display,
        theme: *const c_char,
        size: c_int,
    );
    pub fn gdk_x11_display_set_startup_notification_id(
        display: *mut GdkX11Display,
        startup_id: *const c_char,
    );
    pub fn gdk_x11_display_set_surface_scale(display: *mut GdkX11Display, scale: c_int);
    pub fn gdk_x11_display_string_to_compound_text(
        display: *mut GdkX11Display,
        str: *const c_char,
        encoding: *mut *const c_char,
        format: *mut c_int,
        ctext: *mut *mut u8,
        length: *mut c_int,
    ) -> c_int;
    pub fn gdk_x11_display_text_property_to_text_list(
        display: *mut GdkX11Display,
        encoding: *const c_char,
        format: c_int,
        text: *const u8,
        length: c_int,
        list: *mut *mut *mut c_char,
    ) -> c_int;
    pub fn gdk_x11_display_ungrab(display: *mut GdkX11Display);
    pub fn gdk_x11_display_utf8_to_compound_text(
        display: *mut GdkX11Display,
        str: *const c_char,
        encoding: *mut *const c_char,
        format: *mut c_int,
        ctext: *mut *mut u8,
        length: *mut c_int,
    ) -> gboolean;

    //=========================================================================
    // GdkX11Drag
    //=========================================================================
    pub fn gdk_x11_drag_get_type() -> GType;

    //=========================================================================
    // GdkX11GLContext
    //=========================================================================
    pub fn gdk_x11_gl_context_get_type() -> GType;

    //=========================================================================
    // GdkX11Monitor
    //=========================================================================
    pub fn gdk_x11_monitor_get_type() -> GType;
    pub fn gdk_x11_monitor_get_output(monitor: *mut GdkX11Monitor) -> xlib::XID;
    pub fn gdk_x11_monitor_get_workarea(
        monitor: *mut GdkX11Monitor,
        workarea: *mut gdk::GdkRectangle,
    );

    //=========================================================================
    // GdkX11Screen
    //=========================================================================
    pub fn gdk_x11_screen_get_type() -> GType;
    pub fn gdk_x11_screen_get_current_desktop(screen: *mut GdkX11Screen) -> u32;
    pub fn gdk_x11_screen_get_monitor_output(
        screen: *mut GdkX11Screen,
        monitor_num: c_int,
    ) -> xlib::XID;
    pub fn gdk_x11_screen_get_number_of_desktops(screen: *mut GdkX11Screen) -> u32;
    pub fn gdk_x11_screen_get_screen_number(screen: *mut GdkX11Screen) -> c_int;
    pub fn gdk_x11_screen_get_window_manager_name(screen: *mut GdkX11Screen) -> *const c_char;
    pub fn gdk_x11_screen_get_xscreen(screen: *mut GdkX11Screen) -> *mut xlib::Screen;
    pub fn gdk_x11_screen_supports_net_wm_hint(
        screen: *mut GdkX11Screen,
        property_name: *const c_char,
    ) -> gboolean;

    //=========================================================================
    // GdkX11Surface
    //=========================================================================
    pub fn gdk_x11_surface_get_type() -> GType;
    pub fn gdk_x11_surface_lookup_for_display(
        display: *mut GdkX11Display,
        window: xlib::Window,
    ) -> *mut GdkX11Surface;
    pub fn gdk_x11_surface_get_desktop(surface: *mut GdkX11Surface) -> u32;
    pub fn gdk_x11_surface_get_group(surface: *mut GdkX11Surface) -> *mut gdk::GdkSurface;
    pub fn gdk_x11_surface_get_xid(surface: *mut GdkX11Surface) -> xlib::Window;
    pub fn gdk_x11_surface_move_to_current_desktop(surface: *mut GdkX11Surface);
    pub fn gdk_x11_surface_move_to_desktop(surface: *mut GdkX11Surface, desktop: u32);
    pub fn gdk_x11_surface_set_frame_sync_enabled(
        surface: *mut GdkX11Surface,
        frame_sync_enabled: gboolean,
    );
    pub fn gdk_x11_surface_set_group(surface: *mut GdkX11Surface, leader: *mut gdk::GdkSurface);
    pub fn gdk_x11_surface_set_skip_pager_hint(surface: *mut GdkX11Surface, skips_pager: gboolean);
    pub fn gdk_x11_surface_set_skip_taskbar_hint(
        surface: *mut GdkX11Surface,
        skips_taskbar: gboolean,
    );
    pub fn gdk_x11_surface_set_theme_variant(surface: *mut GdkX11Surface, variant: *const c_char);
    pub fn gdk_x11_surface_set_urgency_hint(surface: *mut GdkX11Surface, urgent: gboolean);
    pub fn gdk_x11_surface_set_user_time(surface: *mut GdkX11Surface, timestamp: u32);
    pub fn gdk_x11_surface_set_utf8_property(
        surface: *mut GdkX11Surface,
        name: *const c_char,
        value: *const c_char,
    );

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn gdk_x11_device_get_id(device: *mut GdkX11DeviceXI2) -> c_int;
    pub fn gdk_x11_device_manager_lookup(
        device_manager: *mut GdkX11DeviceManagerXI2,
        device_id: c_int,
    ) -> *mut GdkX11DeviceXI2;
    pub fn gdk_x11_free_compound_text(ctext: *mut u8);
    pub fn gdk_x11_free_text_list(list: *mut *mut c_char);
    pub fn gdk_x11_get_server_time(surface: *mut GdkX11Surface) -> u32;
    pub fn gdk_x11_get_xatom_by_name_for_display(
        display: *mut GdkX11Display,
        atom_name: *const c_char,
    ) -> xlib::Atom;
    pub fn gdk_x11_get_xatom_name_for_display(
        display: *mut GdkX11Display,
        xatom: xlib::Atom,
    ) -> *const c_char;
    pub fn gdk_x11_lookup_xdisplay(xdisplay: *mut xlib::Display) -> *mut GdkX11Display;
    pub fn gdk_x11_set_sm_client_id(sm_client_id: *const c_char);

}
