pub use nemo::*;

pub use glib::{g_debug, g_warning};
pub use glib::gobject_ffi::GTypeModule;
pub use glib::gobject_ffi::g_type_module_register_type;
pub use glib::gobject_ffi::GTypeInfo;
pub use glib::gobject_ffi::GTypeInterface;
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
use gobject_sys::GObject;
use gobject_sys::GObjectClass;
use gobject_sys::G_TYPE_OBJECT;
use gobject_sys::g_type_class_peek_parent;

use gobject_sys::g_type_from_name;
use gobject_sys::g_type_parent;
use std::mem;
use std::ptr;

#[no_mangle]
unsafe extern "C" fn get_name_and_desc_list(_provider: *mut NemoNameAndDescProvider) -> *mut glib::ffi::GList {
    let ret = ptr::null_mut();
    let mut s = "Nemo Rust Test:::Hello from Rust!".as_ptr() as gpointer;
    let ret = g_list_append(ret, s);
    return ret;
}

#[no_mangle]
pub unsafe extern "C" fn nemo_nd_provider_iface_init(iface: *mut c_void, _: *mut c_void) {
    g_warning!("", "Made it to nemo_nd_provider_iface_init");
    let mut gtypeiface = GTypeInterface {
        g_type: 0 as GType,
        g_instance_type: 0 as GType
    };
    let mut iface = NemoNameAndDescProviderIface {
        g_iface: gtypeiface,
        get_name_and_desc: Some(get_name_and_desc_list)
    };
    g_debug!("Define I value", "");
}

#[no_mangle]
#[link(name = "nemo_extension")]
pub fn nemo_module_list_types(types: *mut GType, mut num_types: c_int) {
    unsafe {
        let type_list: [GType;1] = [*types];
        *types = *type_list.as_ptr();
        let x = &mut num_types;
        *x = 1 as c_int;
    }
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
        class_size: 3000 as u16,
        base_init: None,
        base_finalize: None,
        class_init: None,
        class_finalize: None,
        class_data: ptr::null(),
        instance_size: 3000 as u16,
        n_preallocs: 0,
        instance_init: None,
        value_table: &fake_valtable,
    };

    let nd_provider_iface_info = GInterfaceInfo {
        interface_init: Some(nemo_nd_provider_iface_init),
        interface_finalize: None,
        interface_data: nemo_nd_provider_iface_init as gpointer,
    };


    let name = CString::new(&"NemoRustTest" as &str).unwrap();
    let mut rt_type = g_type_module_register_type(module, G_TYPE_OBJECT, name.as_ptr(), &info, 0);
    g_debug!("", "registred type");
    g_type_module_add_interface(module, rt_type, nemo_name_and_desc_provider_get_type(), &nd_provider_iface_info);
    g_debug!("", "added interface");
    nemo_module_list_types(rt_type as *mut usize, 1 as c_int);
}
}

#[no_mangle]
#[link(name = "nemo_extension")]
pub fn nemo_module_shutdown(_: c_void) {
    g_debug!("", "Goodbye from Rust");
}
