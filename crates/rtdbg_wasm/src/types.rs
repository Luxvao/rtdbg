// Type definitions
pub type Idx = u32;
pub type TypeIdx = Idx;

#[derive(Clone, Debug)]
pub enum Type {
    // Core types in WASM
    Num(NumType),

    // Vectory type
    Vec(VecType),

    // Type use. Extend later apparently
    TypeUse(TypeUse),

    Heap(HeapType),

    Reference(ReferenceType),

    Val(ValueType),

    Result(ResultType),
}

#[derive(Clone, Debug)]
pub enum NumType {
    I32(u32),
    I64(u64),
    F32(f32),
    F64(f64),
}

#[derive(Clone, Debug)]
pub struct VecType(pub u128);

#[derive(Clone, Debug)]
pub struct TypeUse(pub TypeIdx);

// TODO - DO THIS LATER
#[derive(Clone, Debug)]
pub struct HeapType {}

#[derive(Clone, Debug)]
pub struct ReferenceType(pub Option<HeapType>);

#[derive(Clone, Debug)]
pub enum ValueType {
    Num(NumType),
    Vec(VecType),
    Reference(ReferenceType),
}

#[derive(Clone, Debug)]
pub struct ResultType(pub Vec<ValueType>);

#[derive(Clone, Debug)]
pub enum BlockType {
    Val(ValueType),
    TypeIdx(TypeIdx),
}
