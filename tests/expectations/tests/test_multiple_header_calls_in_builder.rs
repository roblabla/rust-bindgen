/* automatically generated by rust-bindgen */

extern "C" {
    #[link_name = "foo"]
    pub static mut foo:
               ::std::option::Option<unsafe extern "C" fn(x:
                                                              ::std::os::raw::c_int,
                                                          y:
                                                              ::std::os::raw::c_int)
                                         -> ::std::os::raw::c_int>;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Foo { Bar = 0, Qux = 1, }
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Neg { MinusOne = -1, One = 1, }
