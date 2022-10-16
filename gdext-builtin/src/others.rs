/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// Stub for various other built-in classes, which are currently incomplete, but whose types
// are required for codegen
use crate::GodotString;
use godot_ffi as sys;
use sys::{ffi_methods, GodotFfi};

// TODO: Swap more inner math types with glam types
impl_builtin_stub!(AABB, OpaqueAABB);
impl_builtin_stub!(Basis, OpaqueBasis);
impl_builtin_stub!(Plane, OpaquePlane);
impl_builtin_stub!(Quaternion, OpaqueQuaternion);
impl_builtin_stub!(Rect2, OpaqueRect2);
impl_builtin_stub!(Rect2i, OpaqueRect2i);

impl_builtin_stub!(RID, OpaqueRID);
impl_builtin_stub!(Callable, OpaqueCallable);
impl_builtin_stub!(Dictionary, OpaqueDictionary);
impl_builtin_stub!(Transform2D, OpaqueTransform2D);
impl_builtin_stub!(Transform3D, OpaqueTransform3D);
impl_builtin_stub!(NodePath, OpaqueNodePath);

#[repr(C)]
pub struct StringName {
    opaque: sys::types::OpaqueStringName,
}
impl StringName {
    fn from_opaque(opaque: sys::types::OpaqueStringName) -> Self {
        Self { opaque }
    }

    ffi_methods! {
        type sys::GDNativeStringNamePtr = *mut Opaque;

        fn from_string_sys = from_sys;
        fn from_string_sys_init = from_sys_init;
        fn string_sys = sys;
        fn write_string_sys = write_sys;
    }

    #[doc(hidden)]
    pub fn leak_string_sys(self) -> sys::GDNativeStringNamePtr {
        let ptr = self.string_sys();
        std::mem::forget(self);
        ptr
    }
}

impl Drop for StringName {
    fn drop(&mut self) {
        unsafe {
            (sys::method_table().string_name_destroy)(self.sys());
        }
    }
}

impl GodotFfi for StringName {
    ffi_methods! {
        type sys::GDNativeTypePtr = *mut Opaque;
        fn from_sys;
        fn sys;
        fn write_sys;
    }

    unsafe fn from_sys_init(init_fn: impl FnOnce(sys::GDNativeTypePtr)) -> Self {
        // Can't use uninitialized pointer -- StringName implementation in C++ expects that on assignment,
        // the target type is a valid string (possibly empty)

        let mut result = Self::default();
        init_fn(result.sys_mut());
        result
    }
}

impl Default for StringName {
    fn default() -> Self {
        // Note: can't use from_sys_init(), as that calls the default constructor

        let mut uninit = std::mem::MaybeUninit::<StringName>::uninit();

        unsafe {
            let self_ptr = (*uninit.as_mut_ptr()).sys_mut();
            let ctor = sys::method_table().string_name_construct_default;
            ctor(self_ptr, std::ptr::null_mut());

            uninit.assume_init()
        }
    }
}
impl From<&GodotString> for StringName {
    fn from(s: &GodotString) -> Self {
        unsafe {
            Self::from_sys_init(|self_ptr| {
                let ctor = sys::method_table().string_name_from_string;
                let args = [s.sys()];
                ctor(self_ptr, args.as_ptr());
            })
        }
    }
}
impl From<&StringName> for GodotString {
    fn from(s: &StringName) -> Self {
        unsafe {
            Self::from_sys_init(|self_ptr| {
                let ctor = sys::method_table().string_from_string_name;
                let args = [s.sys()];
                ctor(self_ptr, args.as_ptr());
            })
        }
    }
}
