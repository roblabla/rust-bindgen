/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


/// <div rustbindgen opaque></div>
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct OtherOpaque {
    pub _bindgen_opaque_blob: u32,
}
#[test]
fn bindgen_test_layout_OtherOpaque() {
    assert_eq!(::std::mem::size_of::<OtherOpaque>() , 4usize , concat ! (
               "Size of: " , stringify ! ( OtherOpaque ) ));
    assert_eq! (::std::mem::align_of::<OtherOpaque>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( OtherOpaque ) ));
}
impl Clone for OtherOpaque {
    fn clone(&self) -> Self { *self }
}
/// <div rustbindgen opaque></div>
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Opaque {
}
impl Default for Opaque {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct WithOpaquePtr {
    pub whatever: *mut u8,
    pub other: u32,
    pub t: OtherOpaque,
}
#[test]
fn bindgen_test_layout_WithOpaquePtr() {
    assert_eq!(::std::mem::size_of::<WithOpaquePtr>() , 16usize , concat ! (
               "Size of: " , stringify ! ( WithOpaquePtr ) ));
    assert_eq! (::std::mem::align_of::<WithOpaquePtr>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( WithOpaquePtr ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const WithOpaquePtr ) ) . whatever as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( WithOpaquePtr ) , "::"
                , stringify ! ( whatever ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const WithOpaquePtr ) ) . other as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( WithOpaquePtr ) , "::"
                , stringify ! ( other ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const WithOpaquePtr ) ) . t as * const _ as
                usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( WithOpaquePtr ) , "::"
                , stringify ! ( t ) ));
}
impl Clone for WithOpaquePtr {
    fn clone(&self) -> Self { *self }
}
impl Default for WithOpaquePtr {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[test]
fn __bindgen_test_layout_Opaque_instantiation() {
    assert_eq!(::std::mem::size_of::<u32>() , 4usize , concat ! (
               "Size of template specialization: " , stringify ! ( u32 ) ));
    assert_eq!(::std::mem::align_of::<u32>() , 4usize , concat ! (
               "Alignment of template specialization: " , stringify ! ( u32 )
               ));
}
