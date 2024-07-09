use crate::table::TableError;

#[derive(Clone, Copy, PartialEq)]
pub enum ValueType {
    Str,
    Int,
    UInt,
    Float,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Value<'a> {
    Str(&'a str),
    Int(i32),
    UInt(u32),
    Float(f32),
}

pub trait ToType {
    fn to_type(&self) -> ValueType;
}

impl<'a> ToType for Value<'a> {
    fn to_type(&self) -> ValueType {
        return match self {
            Self::Str(_) => ValueType::Str,
            Self::Int(_) => ValueType::Int,
            Self::UInt(_) => ValueType::UInt,
            Self::Float(_) => ValueType::Float,
        };
    }
}

impl<'a> TryFrom<&'a Value<'a>> for &'a str {
    type Error = TableError;
    fn try_from(value: &'a Value<'a>) -> Result<Self, Self::Error> {
        return match value {
            Value::Str(value) => Ok(*value),
            _ => Err(TableError::MismatchedTypes),
        };
    }
}
impl<'a> TryFrom<&'a Value<'a>> for i32 {
    type Error = TableError;
    fn try_from(value: &'a Value<'a>) -> Result<Self, Self::Error> {
        return match value {
            Value::Int(value) => Ok(*value),
            _ => Err(TableError::MismatchedTypes),
        };
    }
}
impl<'a> TryFrom<&'a Value<'a>> for u32 {
    type Error = TableError;
    fn try_from(value: &'a Value<'a>) -> Result<Self, Self::Error> {
        return match value {
            Value::UInt(value) => Ok(*value),
            _ => Err(TableError::MismatchedTypes),
        };
    }
}
impl<'a> TryFrom<&'a Value<'a>> for f32 {
    type Error = TableError;
    fn try_from(value: &'a Value<'a>) -> Result<Self, Self::Error> {
        return match value {
            Value::Float(value) => Ok(*value),
            _ => Err(TableError::MismatchedTypes),
        };
    }
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(value: &'a str) -> Self {
        Self::Str(value)
    }
}
impl From<i32> for Value<'_> {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}
impl From<u32> for Value<'_> {
    fn from(value: u32) -> Self {
        Self::UInt(value)
    }
}
impl From<f32> for Value<'_> {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}
