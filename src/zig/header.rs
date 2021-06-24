use super::*;
use std::io::Write;

impl ZigGenerator {
    pub fn header<T: Write>(w: &mut PrettyWriter<T>) -> Result<(), Error> {
        w.write_lines(
            "
//
// This file was automatically generated by witx-codegen - Do not edit manually.
//

pub const WasiHandle = i32;
pub const Char8 = u8;
pub const Char32 = u32;
pub fn WasiPtr(comptime T: type) type {
    return [*c]const T;
}
pub fn WasiMutPtr(comptime T: type) type {
    return [*c]T;
}
pub const WasiStringBytesPtr = WasiPtr(Char8);

pub const WasiString = extern struct {
    ptr: WasiStringBytesPtr,
    len: usize,

    fn from_slice(slice: []const u8) WasiString {
        return WasiString{ .ptr = slice.ptr, .len = slice.len };
    }

    fn as_slice(wasi_string: WasiString) []const u8 {
        return wasi_string.ptr[wasi_string.len];
    }
};

pub fn WasiSlice(comptime T: type) type {
    return extern struct {
        ptr: WasiPtr(T),
        len: usize,

        fn from_slice(slice: []const u8) WasiSlice {
            return WasiSlice{ .ptr = slice.ptr, .len = slice.len };
        }

        fn as_slice(wasi_slice: WasiSlice) []const u8 {
            return wasi_slice.ptr[wasi_slice.len];
        }
    };
}

pub fn WasiMutSlice(comptime T: type) type {
    return extern struct {
        ptr: WasiMutPtr(T),
        len: usize,

        fn from_slice(slice: []u8) WasiMutSlice {
            return WasiMutSlice{ .ptr = slice.ptr, .len = slice.len };
        }

        fn as_slice(wasi_slice: WasiMutSlice) []u8 {
            return wasi_slice.ptr[wasi_slice.len];
        }
    };
}
",
        )?;
        w.eob()?;
        Ok(())
    }
}
