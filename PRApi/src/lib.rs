//! Very Simple, Safe API for calling engine/math functions and Supports modding via `call_*` primitives.

mod error;
mod func;

use error::*;
use func::*;

use libloading::{Library, Symbol};

// TODO: Later Seprate Buildin-Functions
//  Also add User-Functions/Args
pub struct PRApi {
    lib: Library,

    pub test: Function, // fn test()
    pub start: Function,
}

impl PRApi {
    pub fn new(library_path: &str) -> Self {
        Self {
            lib: unsafe { Library::new(library_path).unwrap() },

            test: Function::new("test", FnSignature::Void),
            start: Function::new("start", FnSignature::Void),
        }
    }

    pub fn test(&self) -> Result<(), libloading::Error> {
        self.call_void("test")
    }

    pub fn start(&self) -> Result<(), libloading::Error> {
        self.call_void("start")
    }

    pub fn get_scale(&self) -> Result<f32, libloading::Error> {
        self.call_f32("get_scale")
    }

    pub fn get_ratio(&self) -> Result<f64, libloading::Error> {
        self.call_f64("get_ratio")
    }

    pub fn get_frame(&self) -> Result<i32, libloading::Error> {
        self.call_i32("get_frame")
    }

    pub fn get_timestamp(&self) -> Result<i64, libloading::Error> {
        self.call_i64("get_timestamp")
    }

    pub fn get_flags(&self) -> Result<u32, libloading::Error> {
        self.call_u32("get_flags")
    }

    pub fn get_uid(&self) -> Result<u64, libloading::Error> {
        self.call_u64("get_uid")
    }

    pub fn get_version(&self) -> Result<u8, libloading::Error> {
        self.call_u8("get_version")
    }

    // pub fn call_void(name: &str)
    pub fn call_void(&self, name: &str) -> Result<(), libloading::Error> {
        unsafe {
            let f: Symbol<unsafe extern "C" fn()> = self.lib.get(name.as_bytes())?;
            f();
            Ok(())
        }
    }

    // pub fn call_f32(name: &str) -> f32
    pub fn call_f32(&self, name: &str) -> Result<f32, libloading::Error> {
        unsafe {
            let f: Symbol<unsafe extern "C" fn() -> f32> = self.lib.get(name.as_bytes())?;
            Ok(f())
        }
    }

    // pub fn call_f64(name: &str) -> f64
    pub fn call_f64(&self, name: &str) -> Result<f64, libloading::Error> {
        unsafe {
            let f: Symbol<unsafe extern "C" fn() -> f64> = self.lib.get(name.as_bytes())?;
            Ok(f())
        }
    }

    // pub fn call_i32(name: &str) -> i32
    pub fn call_i32(&self, name: &str) -> Result<i32, libloading::Error> {
        unsafe {
            let f: Symbol<unsafe extern "C" fn() -> i32> = self.lib.get(name.as_bytes())?;
            Ok(f())
        }
    }

    // pub fn call_i64(name: &str) -> i64
    pub fn call_i64(&self, name: &str) -> Result<i64, libloading::Error> {
        unsafe {
            let f: Symbol<unsafe extern "C" fn() -> i64> = self.lib.get(name.as_bytes())?;
            Ok(f())
        }
    }

    // pub fn call_u32(name: &str) -> u32
    pub fn call_u32(&self, name: &str) -> Result<u32, libloading::Error> {
        unsafe {
            let f: Symbol<unsafe extern "C" fn() -> u32> = self.lib.get(name.as_bytes())?;
            Ok(f())
        }
    }

    // pub fn call_u64(name: &str) -> u64
    pub fn call_u64(&self, name: &str) -> Result<u64, libloading::Error> {
        unsafe {
            let f: Symbol<unsafe extern "C" fn() -> u64> = self.lib.get(name.as_bytes())?;
            Ok(f())
        }
    }

    // pub fn call_u8(name: &str) -> u8
    pub fn call_u8(&self, name: &str) -> Result<u8, libloading::Error> {
        unsafe {
            let f: Symbol<unsafe extern "C" fn() -> u8> = self.lib.get(name.as_bytes())?;
            Ok(f())
        }
    }

    /// Call any function by name, dispatching on its registered `FnSignature`.
    pub fn call_dyn(&self, func: &Function) -> Result<DynValue, CallError> {
        Ok(match func.signature {
            FnSignature::Void => {
                self.call_void(func.name)?;
                DynValue::Void
            }
            FnSignature::ReturnF32 => DynValue::F32(self.call_f32(func.name)?),
            FnSignature::ReturnF64 => DynValue::F64(self.call_f64(func.name)?),
            FnSignature::ReturnI32 => DynValue::I32(self.call_i32(func.name)?),
            FnSignature::ReturnI64 => DynValue::I64(self.call_i64(func.name)?),
            FnSignature::ReturnU32 => DynValue::U32(self.call_u32(func.name)?),
            FnSignature::ReturnU64 => DynValue::U64(self.call_u64(func.name)?),
            FnSignature::ReturnU8 => DynValue::U8(self.call_u8(func.name)?),
        })
    }
}
