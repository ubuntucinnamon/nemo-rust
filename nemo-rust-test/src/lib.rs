pub use nemo::*;

pub use glib::{g_debug};
pub use glib::gobject_ffi::GTypeModule;
pub use glib::gobject_ffi::g_type_module_register_type;
pub use glib::gobject_ffi::GTypeInfo;
use glib::gobject_ffi::GTypeValueTable;
use glib::gobject_ffi::GInterfaceInfo;
use glib::gobject_ffi::g_type_module_add_interface;
use glib::ffi::g_list_append;
pub use glib_sys::g_list_alloc;
pub use glib::ffi::GList;
use glib_sys::gpointer;
pub use nemo_sys::*;
use nemo_sys::nemo_name_and_desc_provider_get_type;

use std::ffi::c_char;
use std::ffi::CString;
use std::ffi::c_void;
use std::ffi::c_int;
use glib_sys::GType;

use std::mem;
use std::ptr;

#[no_mangle]
unsafe extern "C" fn get_name_and_desc_list(provider: *mut NemoNameAndDescProvider) -> *mut glib::ffi::GList {
    let ret = ptr::null_mut();
    let s = "NemoRustTest".as_ptr() as gpointer;
    let ret = g_list_append(ret, s);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn nemo_nd_provider_iface_init(iface: *mut c_void, _: *mut c_void) {
    g_debug!("", "Made it to nemo_nd_provider_iface_init");
    let mut i: NemoNameAndDescProviderIface = mem::uninitialized();
    i.get_name_and_desc = Some(get_name_and_desc_list);
    g_debug!("Define I value", "");
}

#[no_mangle]
#[link(name = "nemo_extension")]
pub fn nemo_module_initialize(module: *mut GTypeModule) {
    unsafe {
    g_debug!("Hello from Rust!", "");

    let fake_valtable = GTypeValueTable {
        value_init: None,
        value_free: None,
        value_copy: None,
        value_peek_pointer: None,
        collect_format: 0 as *const c_char,
        collect_value: None,
        lcopy_format: 0 as *const c_char,
        lcopy_value: None,
    };

    let info = GTypeInfo {
        class_size: 3000,
        base_init: None,
        base_finalize: None,
        class_init: None,
        class_finalize: None,
        class_data: ptr::null(),
        instance_size: 0,
        n_preallocs: 0,
        instance_init: None,
        value_table: &fake_valtable,
    };

    let nd_provider_iface_info = GInterfaceInfo {
        interface_init: Some(nemo_nd_provider_iface_init),
        interface_finalize: None,
        interface_data: ptr::null_mut(),
    };

    let rt_type: GType = 1;

    let name = CString::new(&"NemoRustTest" as &str).unwrap();
    let mut rt_type = g_type_module_register_type(module, rt_type, name.as_ptr(), &info, 10);
    g_debug!("", "registred type");
    let nd = g_type_module_add_interface(module, rt_type, nemo_name_and_desc_provider_get_type(), &nd_provider_iface_info);
    g_debug!("", "added interface");
}
}

#[no_mangle]
#[link(name = "nemo_extension")]
pub fn nemo_module_shutdown(_: c_void) {
    g_debug!("", "Goodbye from Rust");
}


#[no_mangle]
#[link(name = "nemo_extension")]
pub fn nemo_module_list_types(types: *mut *const GType, num_types: *mut c_int) {
    unsafe {
    let mut type_list: [GType; 1] = [1 as GType];
    type_list[0] = 0 as GType;
    *types = &type_list[0];
    *num_types = 1; // note: unsafe bc this can be a NPD
    }
}
