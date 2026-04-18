/// The C-side return type of a registered function.
#[derive(Debug, Clone)]
pub enum FnSignature {
    Void,      // fn name()
    ReturnF32, // fn name() -> f32
    ReturnF64, // fn name() -> f64
    ReturnI32, // fn name() -> i32
    ReturnI64, // fn name() -> i64
    ReturnU32, // fn name() -> u32
    ReturnU64, // fn name() -> u64
    ReturnU8,  // fn name() -> u8
}

/// Metadata for a single exported engine function.
pub struct Function {
    pub name: &'static str,
    pub signature: FnSignature,
}

impl Function {
    pub const fn new(name: &'static str, signature: FnSignature) -> Self {
        Self { name, signature }
    }
}

/// Dynamically-typed return value from `call_dyn`.
#[derive(Debug)]
pub enum DynValue {
    Void,
    F32(f32),
    F64(f64),
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    U8(u8),
}
