/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Base {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Derived {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Derived() {
    assert_eq!(::std::mem::size_of::<Derived>() , 1usize , concat ! (
               "Size of: " , stringify ! ( Derived ) ));
    assert_eq! (::std::mem::align_of::<Derived>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( Derived ) ));
}
impl Clone for Derived {
    fn clone(&self) -> Self { *self }
}
impl Default for Derived {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct BaseWithDestructor {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug)]
pub struct DerivedFromBaseWithDestructor {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_DerivedFromBaseWithDestructor() {
    assert_eq!(::std::mem::size_of::<DerivedFromBaseWithDestructor>() , 1usize
               , concat ! (
               "Size of: " , stringify ! ( DerivedFromBaseWithDestructor ) ));
    assert_eq! (::std::mem::align_of::<DerivedFromBaseWithDestructor>() ,
                1usize , concat ! (
                "Alignment of " , stringify ! ( DerivedFromBaseWithDestructor
                ) ));
}
impl Default for DerivedFromBaseWithDestructor {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[test]
fn __bindgen_test_layout_Base_instantiation() {
    assert_eq!(::std::mem::size_of::<Base>() , 1usize , concat ! (
               "Size of template specialization: " , stringify ! ( Base ) ));
    assert_eq!(::std::mem::align_of::<Base>() , 1usize , concat ! (
               "Alignment of template specialization: " , stringify ! ( Base )
               ));
}
#[test]
fn __bindgen_test_layout_BaseWithDestructor_instantiation() {
    assert_eq!(::std::mem::size_of::<BaseWithDestructor>() , 1usize , concat !
               (
               "Size of template specialization: " , stringify ! (
               BaseWithDestructor ) ));
    assert_eq!(::std::mem::align_of::<BaseWithDestructor>() , 1usize , concat
               ! (
               "Alignment of template specialization: " , stringify ! (
               BaseWithDestructor ) ));
}
