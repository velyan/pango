// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use pango_sys;
use std::mem;
use TabAlign;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TabArray(Boxed<pango_sys::PangoTabArray>);

    match fn {
        copy => |ptr| pango_sys::pango_tab_array_copy(mut_override(ptr)),
        free => |ptr| pango_sys::pango_tab_array_free(ptr),
        get_type => || pango_sys::pango_tab_array_get_type(),
    }
}

impl TabArray {
    pub fn new(initial_size: i32, positions_in_pixels: bool) -> TabArray {
        unsafe {
            from_glib_full(pango_sys::pango_tab_array_new(
                initial_size,
                positions_in_pixels.to_glib(),
            ))
        }
    }

    //pub fn new_with_positions(size: i32, positions_in_pixels: bool, first_alignment: TabAlign, first_position: i32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> TabArray {
    //    unsafe { TODO: call pango_sys:pango_tab_array_new_with_positions() }
    //}

    pub fn get_positions_in_pixels(&mut self) -> bool {
        unsafe {
            from_glib(pango_sys::pango_tab_array_get_positions_in_pixels(
                self.to_glib_none_mut().0,
            ))
        }
    }

    pub fn get_size(&mut self) -> i32 {
        unsafe { pango_sys::pango_tab_array_get_size(self.to_glib_none_mut().0) }
    }

    pub fn get_tab(&mut self, tab_index: i32) -> (TabAlign, i32) {
        unsafe {
            let mut alignment = mem::MaybeUninit::uninit();
            let mut location = mem::MaybeUninit::uninit();
            pango_sys::pango_tab_array_get_tab(
                self.to_glib_none_mut().0,
                tab_index,
                alignment.as_mut_ptr(),
                location.as_mut_ptr(),
            );
            let alignment = alignment.assume_init();
            let location = location.assume_init();
            (from_glib(alignment), location)
        }
    }

    //pub fn get_tabs(&mut self, locations: Vec<i32>) -> TabAlign {
    //    unsafe { TODO: call pango_sys:pango_tab_array_get_tabs() }
    //}

    pub fn resize(&mut self, new_size: i32) {
        unsafe {
            pango_sys::pango_tab_array_resize(self.to_glib_none_mut().0, new_size);
        }
    }

    pub fn set_tab(&mut self, tab_index: i32, alignment: TabAlign, location: i32) {
        unsafe {
            pango_sys::pango_tab_array_set_tab(
                self.to_glib_none_mut().0,
                tab_index,
                alignment.to_glib(),
                location,
            );
        }
    }
}
