// Random utilities

// Trait that makes working with u128 implicit conversions nicer for integers
pub trait FromU128 {
    fn from_u128(value: u128) -> Self;
}

impl FromU128 for u8 {
    fn from_u128(value: u128) -> Self {
        value as u8
    }
}

impl FromU128 for u16 {
    fn from_u128(value: u128) -> Self {
        value as u16
    }
}

impl FromU128 for u32 {
    fn from_u128(value: u128) -> Self {
        value as u32
    }
}

impl FromU128 for u64 {
    fn from_u128(value: u128) -> Self {
        value as u64
    }
}

impl FromU128 for u128 {
    fn from_u128(value: u128) -> Self {
        value
    }
}

impl FromU128 for i8 {
    fn from_u128(value: u128) -> Self {
        value as i8
    }
}

impl FromU128 for i16 {
    fn from_u128(value: u128) -> Self {
        value as i16
    }
}

impl FromU128 for i32 {
    fn from_u128(value: u128) -> Self {
        value as i32
    }
}

impl FromU128 for i64 {
    fn from_u128(value: u128) -> Self {
        value as i64
    }
}

impl FromU128 for f32 {
    fn from_u128(value: u128) -> Self {
        value as f32
    }
}

impl FromU128 for f64 {
    fn from_u128(value: u128) -> Self {
        value as f64
    }
}

pub trait ToU128 {
    fn to_u128(self) -> u128;
}

impl ToU128 for u8 {
    fn to_u128(self) -> u128 {
        self as u128
    }
}

impl ToU128 for u16 {
    fn to_u128(self) -> u128 {
        self as u128
    }
}

impl ToU128 for u32 {
    fn to_u128(self) -> u128 {
        self as u128
    }
}

impl ToU128 for u64 {
    fn to_u128(self) -> u128 {
        self as u128
    }
}

impl ToU128 for u128 {
    fn to_u128(self) -> u128 {
        self
    }
}

impl ToU128 for i8 {
    fn to_u128(self) -> u128 {
        self as u128
    }
}

impl ToU128 for i16 {
    fn to_u128(self) -> u128 {
        self as u128
    }
}

impl ToU128 for i32 {
    fn to_u128(self) -> u128 {
        self as u128
    }
}

impl ToU128 for i64 {
    fn to_u128(self) -> u128 {
        self as u128
    }
}

impl ToU128 for f32 {
    fn to_u128(self) -> u128 {
        self as u128
    }
}

impl ToU128 for f64 {
    fn to_u128(self) -> u128 {
        self as u128
    }
}
