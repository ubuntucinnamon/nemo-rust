pub use nemo::*;

pub use glib::*;
pub use glib::gobject_ffi::GTypeModule;

#[no_mangle]
#[link(name = "nemo_extension")]
pub fn nemo_module_initialize(module: *mut GTypeModule) {
    g_debug!("Hello from Rust!", "");
}



#[no_mangle]
#[link(name = "nemo_extension")]
pub fn nemo_module_shutdown() {
    g_debug!("Rust says goodbye", "");
}

#[no_mangle]
#[link(name = "nemo_extension")]
pub fn nemo_module_list_types() {

}
