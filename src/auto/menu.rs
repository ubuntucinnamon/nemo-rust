// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::MenuItem;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "NemoMenu")]
    pub struct Menu(Object<ffi::NemoMenu, ffi::NemoMenuClass>);

    match fn {
        type_ => || ffi::nemo_menu_get_type(),
    }
}

impl Menu {
    #[doc(alias = "nemo_menu_new")]
    pub fn new() -> Menu {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::nemo_menu_new())
        }
    }

    #[doc(alias = "nemo_menu_append_item")]
    pub fn append_item(&self, item: &MenuItem) {
        unsafe {
            ffi::nemo_menu_append_item(self.to_glib_none().0, item.to_glib_none().0);
        }
    }

    #[doc(alias = "nemo_menu_get_items")]
    #[doc(alias = "get_items")]
    pub fn items(&self) -> Vec<MenuItem> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::nemo_menu_get_items(self.to_glib_none().0))
        }
    }
}

impl Default for Menu {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Menu")
    }
}
