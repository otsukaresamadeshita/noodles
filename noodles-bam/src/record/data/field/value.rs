//! BAM record data field value and types.

pub mod subtype;
pub mod ty;

pub use self::{subtype::Subtype, ty::Type};

/// A BAM record data field value.
///
/// BAM record data field values support all the same types as a SAM record data field value:
///
///   * character (`A`),
///   * 32-bit integer (`i`),
///   * single-precision floating-point (`f`),
///   * string (`Z`),
///   * hex string (`H`),
///   * 8-bit integer array (`Bc`),
///   * 8-bit unsigned integer array (`BC`),
///   * 16-bit integer array (`Bs`),
///   * 16-bit unsigned integer array (`BS`),
///   * 32-bit integer array (`Bi`),
///   * 32-bit unsigned integer array (`BI`), and
///   * single-precision floating-point array (`Bf`),
///
/// Additionally, it is a superset of SAM record data field values, supporting additional
/// single-value integer types:
///
///   * 8-bit integer (`c`),
///   * 8-bit unsigned integer (`C`),
///   * 16-bit integer (`s`),
///   * 16-bit unsigned integer (`S`), and
///   * 32-bit unsigned integer (`I`).
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// A BAM data field character (`A`).
    Char(char),
    /// A BAM data field 8-bit integer (`c`).
    Int8(i8),
    /// A BAM data field 8-bit unsigned integer (`C`).
    UInt8(u8),
    /// A BAM data field 16-bit integer (`s`).
    Int16(i16),
    /// A BAM data field 16-bit unsigned integer (`S`).
    UInt16(u16),
    /// A BAM data field 32-bit integer (`i`).
    Int32(i32),
    /// A BAM data field 32-bit unsigned integer (`I`).
    UInt32(u32),
    /// A BAM data field single-precision floating-point (`f`).
    Float(f32),
    /// A BAM data field string (`Z`).
    String(String),
    /// A BAM data field hex string (`H`).
    Hex(String),
    /// A BAM data field 8-bit integer array (`Bc`).
    Int8Array(Vec<i8>),
    /// A BAM data field 8-bit unsigned integer array (`BC`).
    UInt8Array(Vec<u8>),
    /// A BAM data field 16-bit integer array (`Bs`).
    Int16Array(Vec<i16>),
    /// A BAM data field 16-bit unsigned integer array (`BS`).
    UInt16Array(Vec<u16>),
    /// A BAM data field 32-bit integer array (`Bi`).
    Int32Array(Vec<i32>),
    /// A BAM data field 32-bit unsigned integer array (`BI`).
    UInt32Array(Vec<u32>),
    /// A BAM data field single-precision floating-point array (`Bf`).
    FloatArray(Vec<f32>),
}

impl Value {
    /// Returns the type of the value.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::{value::Type, Value};
    /// assert_eq!(Value::Int32(0).ty(), Type::Int32);
    /// ```
    pub fn ty(&self) -> Type {
        match *self {
            Self::Char(_) => Type::Char,
            Self::Int8(_) => Type::Int8,
            Self::UInt8(_) => Type::UInt8,
            Self::Int16(_) => Type::Int16,
            Self::UInt16(_) => Type::UInt16,
            Self::Int32(_) => Type::Int32,
            Self::UInt32(_) => Type::UInt32,
            Self::Float(_) => Type::Float,
            Self::String(_) => Type::String,
            Self::Hex(_) => Type::Hex,
            Self::Int8Array(_) => Type::Array,
            Self::UInt8Array(_) => Type::Array,
            Self::Int16Array(_) => Type::Array,
            Self::UInt16Array(_) => Type::Array,
            Self::Int32Array(_) => Type::Array,
            Self::UInt32Array(_) => Type::Array,
            Self::FloatArray(_) => Type::Array,
        }
    }

    /// Returns the subtype of the value.
    ///
    /// Only arrays have subtypes.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::{value::Subtype, Value};
    /// assert_eq!(Value::UInt8Array(vec![0]).subtype(), Some(Subtype::UInt8));
    /// assert_eq!(Value::Int32(0).subtype(), None);
    /// ```
    pub fn subtype(&self) -> Option<Subtype> {
        match *self {
            Self::Int8Array(_) => Some(Subtype::Int8),
            Self::UInt8Array(_) => Some(Subtype::UInt8),
            Self::Int16Array(_) => Some(Subtype::Int16),
            Self::UInt16Array(_) => Some(Subtype::UInt16),
            Self::Int32Array(_) => Some(Subtype::Int32),
            Self::UInt32Array(_) => Some(Subtype::UInt32),
            Self::FloatArray(_) => Some(Subtype::Float),
            _ => None,
        }
    }

    /// Returns the value as a character if it is a character.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert_eq!(Value::Char('a').as_char(), Some('a'));
    /// assert_eq!(Value::Int32(0).as_char(), None);
    /// ```
    pub fn as_char(&self) -> Option<char> {
        match *self {
            Self::Char(c) => Some(c),
            _ => None,
        }
    }

    /// Returns whether the value is a character.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert!(Value::Char('a').is_char());
    /// assert!(!Value::Int32(0).is_char());
    /// ```
    pub fn is_char(&self) -> bool {
        self.as_char().is_some()
    }

    /// Returns the value as an 8-bit integer if it is an 8-bit integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert_eq!(Value::Int8(0).as_int8(), Some(0));
    /// assert_eq!(Value::Int32(0).as_int8(), None);
    /// ```
    pub fn as_int8(&self) -> Option<i8> {
        match *self {
            Self::Int8(b) => Some(b),
            _ => None,
        }
    }

    /// Returns whether the value is an 8-bit integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert!(Value::Int8(0).is_int8());
    /// assert!(!Value::Int32(0).is_int8());
    /// ```
    pub fn is_int8(&self) -> bool {
        self.as_int8().is_some()
    }

    /// Returns the value as an 8-bit unsigned integer if it is an 8-bit unsigned integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert_eq!(Value::UInt8(0).as_uint8(), Some(0));
    /// assert_eq!(Value::Int32(0).as_uint8(), None);
    /// ```
    pub fn as_uint8(&self) -> Option<u8> {
        match *self {
            Self::UInt8(b) => Some(b),
            _ => None,
        }
    }

    /// Returns whether the value is an 8-bit unsigned integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert!(Value::UInt8(0).is_uint8());
    /// assert!(!Value::Int32(0).is_uint8());
    /// ```
    pub fn is_uint8(&self) -> bool {
        self.as_uint8().is_some()
    }

    /// Returns the value as a 16-bit integer if it is a 16-bit integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert_eq!(Value::Int16(0).as_int16(), Some(0));
    /// assert_eq!(Value::Int32(0).as_int16(), None);
    /// ```
    pub fn as_int16(&self) -> Option<i16> {
        match *self {
            Self::Int16(s) => Some(s),
            _ => None,
        }
    }

    /// Returns whether the value is a 16-bit integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert!(Value::Int16(0).is_int16());
    /// assert!(!Value::Int32(0).is_int16());
    /// ```
    pub fn is_int16(&self) -> bool {
        self.as_int16().is_some()
    }

    /// Returns the value as a 16-bit unsigned integer if it is a 16-bit unsigned integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert_eq!(Value::UInt16(0).as_uint16(), Some(0));
    /// assert_eq!(Value::Int32(0).as_uint16(), None);
    /// ```
    pub fn as_uint16(&self) -> Option<u16> {
        match *self {
            Self::UInt16(s) => Some(s),
            _ => None,
        }
    }

    /// Returns whether the value is an 16-bit unsigned integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert!(Value::UInt16(0).is_uint16());
    /// assert!(!Value::Int32(0).is_uint16());
    /// ```
    pub fn is_uint16(&self) -> bool {
        self.as_uint16().is_some()
    }

    /// Returns the value as a 32-bit integer if it is a 32-bit integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert_eq!(Value::Int32(0).as_int32(), Some(0));
    /// assert_eq!(Value::Char('a').as_int32(), None);
    /// ```
    pub fn as_int32(&self) -> Option<i32> {
        match *self {
            Self::Int32(i) => Some(i),
            _ => None,
        }
    }

    /// Returns whether the value is a 32-bit integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert!(Value::Int32(0).is_int32());
    /// assert!(!Value::Char('a').is_int32());
    /// ```
    pub fn is_int32(&self) -> bool {
        self.as_int32().is_some()
    }

    /// Returns the value as a 32-bit unsigned integer if it is a 32-bit unsigned integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert_eq!(Value::UInt32(0).as_uint32(), Some(0));
    /// assert_eq!(Value::Int32(0).as_uint32(), None);
    /// ```
    pub fn as_uint32(&self) -> Option<u32> {
        match *self {
            Self::UInt32(i) => Some(i),
            _ => None,
        }
    }

    /// Returns whether the value is an 32-bit unsigned integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert!(Value::UInt32(0).is_uint32());
    /// assert!(!Value::Int32(0).is_uint32());
    /// ```
    pub fn is_uint32(&self) -> bool {
        self.as_uint32().is_some()
    }

    /// Returns the value as a single-precision floating-point if it is a single-precision
    /// float-point.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert_eq!(Value::Float(0.0).as_float(), Some(0.0));
    /// assert_eq!(Value::Int32(0).as_float(), None);
    /// ```
    pub fn as_float(&self) -> Option<f32> {
        match *self {
            Self::Float(s) => Some(s),
            _ => None,
        }
    }

    /// Returns whether the value is a single-precision floating-point.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert!(Value::Float(0.0).is_float());
    /// assert!(!Value::Int32(0).is_float());
    /// ```
    pub fn is_float(&self) -> bool {
        self.as_float().is_some()
    }

    /// Returns the value as a string slice if it is a string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert_eq!(Value::String(String::from("noodles")).as_str(), Some("noodles"));
    /// assert_eq!(Value::Int32(0).as_str(), None);
    /// ```
    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Self::String(ref s) => Some(s),
            _ => None,
        }
    }

    /// Returns whether the value is a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert!(Value::String(String::from("noodles")).is_str());
    /// assert!(!Value::Int32(0).is_str());
    /// ```
    pub fn is_str(&self) -> bool {
        self.as_str().is_some()
    }

    /// Returns the value as a string slice of hex if it is a string slice of hex.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert_eq!(Value::Hex(String::from("cafe")).as_hex(), Some("cafe"));
    /// assert_eq!(Value::Int32(0).as_hex(), None);
    /// ```
    pub fn as_hex(&self) -> Option<&str> {
        match *self {
            Self::Hex(ref h) => Some(h),
            _ => None,
        }
    }

    /// Returns whether the value is a hex string.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert!(Value::Hex(String::from("cafe")).is_hex());
    /// assert!(!Value::Int32(0).is_hex());
    /// ```
    pub fn is_hex(&self) -> bool {
        self.as_hex().is_some()
    }

    /// Returns the value as an array of 8-bit integers if it is an array of 8-bit integers.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert_eq!(Value::Int8Array(vec![0]).as_int8_array(), Some(&[0][..]));
    /// assert_eq!(Value::Int32(0).as_int8_array(), None);
    /// ```
    pub fn as_int8_array(&self) -> Option<&[i8]> {
        match *self {
            Self::Int8Array(ref a) => Some(a),
            _ => None,
        }
    }

    /// Returns whether the value is an 8-bit integer array.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert!(Value::Int8Array(vec![0]).is_int8_array());
    /// assert!(!Value::Int32(0).is_int8_array());
    /// ```
    pub fn is_int8_array(&self) -> bool {
        self.as_int8_array().is_some()
    }

    /// Returns the value as an array of 8-bit unsigned integers if it is an array of 8-bit
    /// unsigned integers.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert_eq!(Value::UInt8Array(vec![0]).as_uint8_array(), Some(&[0][..]));
    /// assert_eq!(Value::Int32(0).as_uint8_array(), None);
    /// ```
    pub fn as_uint8_array(&self) -> Option<&[u8]> {
        match *self {
            Self::UInt8Array(ref a) => Some(a),
            _ => None,
        }
    }

    /// Returns whether the value is an 8-bit unsigned integer array.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert!(Value::UInt8Array(vec![0]).is_uint8_array());
    /// assert!(!Value::Int32(0).is_uint8_array());
    /// ```
    pub fn is_uint8_array(&self) -> bool {
        self.as_uint8_array().is_some()
    }

    /// Returns the value as an array of 16-bit integers if it is an array of 16-bit integers.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert_eq!(Value::Int16Array(vec![0]).as_int16_array(), Some(&[0][..]));
    /// assert_eq!(Value::Int32(0).as_int16_array(), None);
    /// ```
    pub fn as_int16_array(&self) -> Option<&[i16]> {
        match *self {
            Self::Int16Array(ref a) => Some(a),
            _ => None,
        }
    }

    /// Returns whether the value is a 16-bit integer array.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert!(Value::Int16Array(vec![0]).is_int16_array());
    /// assert!(!Value::Int32(0).is_int16_array());
    /// ```
    pub fn is_int16_array(&self) -> bool {
        self.as_int16_array().is_some()
    }

    /// Returns the value as an array of 16-bit unsigned integers if it is an array of 16-bit
    /// unsigned integers.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert_eq!(Value::UInt16Array(vec![0]).as_uint16_array(), Some(&[0][..]));
    /// assert_eq!(Value::Int32(0).as_uint16_array(), None);
    /// ```
    pub fn as_uint16_array(&self) -> Option<&[u16]> {
        match *self {
            Self::UInt16Array(ref a) => Some(a),
            _ => None,
        }
    }

    /// Returns whether the value is a 16-bit unsigned integer array.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert!(Value::UInt16Array(vec![0]).is_uint16_array());
    /// assert!(!Value::Int32(0).is_int16_array());
    /// ```
    pub fn is_uint16_array(&self) -> bool {
        self.as_uint16_array().is_some()
    }

    /// Returns the value as an array of 32-bit integers if it is an array of 32-bit integers.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert_eq!(Value::Int32Array(vec![0]).as_int32_array(), Some(&[0][..]));
    /// assert_eq!(Value::Int32(0).as_int32_array(), None);
    /// ```
    pub fn as_int32_array(&self) -> Option<&[i32]> {
        match *self {
            Self::Int32Array(ref a) => Some(a),
            _ => None,
        }
    }

    /// Returns whether the value is a 32-bit integer array.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert!(Value::Int32Array(vec![0]).is_int32_array());
    /// assert!(!Value::Int32(0).is_int16_array());
    /// ```
    pub fn is_int32_array(&self) -> bool {
        self.as_int32_array().is_some()
    }

    /// Returns the value as an array of 32-bit unsigned integers if it is an array of 32-bit
    /// unsigned integers.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert_eq!(Value::UInt32Array(vec![0]).as_uint32_array(), Some(&[0][..]));
    /// assert_eq!(Value::Int32(0).as_uint32_array(), None);
    /// ```
    pub fn as_uint32_array(&self) -> Option<&[u32]> {
        match *self {
            Self::UInt32Array(ref a) => Some(a),
            _ => None,
        }
    }

    /// Returns whether the value is a 32-bit unsigned integer array.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert!(Value::UInt32Array(vec![0]).is_uint32_array());
    /// assert!(!Value::Int32(0).is_int32_array());
    /// ```
    pub fn is_uint32_array(&self) -> bool {
        self.as_uint32_array().is_some()
    }

    /// Returns the value as an array of single-precision floating-points if it is an array of
    /// single-precision floating-points.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert_eq!(Value::FloatArray(vec![0.0]).as_float_array(), Some(&[0.0][..]));
    /// assert_eq!(Value::Int32(0).as_float_array(), None);
    /// ```
    pub fn as_float_array(&self) -> Option<&[f32]> {
        match *self {
            Self::FloatArray(ref a) => Some(a),
            _ => None,
        }
    }

    /// Returns whether the value is a 32-bit integer array.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bam::record::data::field::Value;
    /// assert!(Value::Int32Array(vec![0]).is_int32_array());
    /// assert!(!Value::Int32(0).is_int16_array());
    /// ```
    pub fn is_float_array(&self) -> bool {
        self.as_float_array().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ty() {
        assert_eq!(Value::Char('m').ty(), Type::Char);
        assert_eq!(Value::Int8(0).ty(), Type::Int8);
        assert_eq!(Value::UInt8(0).ty(), Type::UInt8);
        assert_eq!(Value::Int16(0).ty(), Type::Int16);
        assert_eq!(Value::UInt16(0).ty(), Type::UInt16);
        assert_eq!(Value::Int32(0).ty(), Type::Int32);
        assert_eq!(Value::UInt32(0).ty(), Type::UInt32);
        assert_eq!(Value::Float(0.0).ty(), Type::Float);
        assert_eq!(Value::String(String::from("noodles")).ty(), Type::String);
        assert_eq!(Value::Hex(String::from("cafe")).ty(), Type::Hex);
        assert_eq!(Value::Int8Array(vec![0]).ty(), Type::Array);
        assert_eq!(Value::UInt8Array(vec![0]).ty(), Type::Array);
        assert_eq!(Value::Int16Array(vec![0]).ty(), Type::Array);
        assert_eq!(Value::UInt16Array(vec![0]).ty(), Type::Array);
        assert_eq!(Value::Int32Array(vec![0]).ty(), Type::Array);
        assert_eq!(Value::UInt32Array(vec![0]).ty(), Type::Array);
        assert_eq!(Value::FloatArray(vec![0.0]).ty(), Type::Array);
    }

    #[test]
    fn test_subtype() {
        assert_eq!(Value::Char('m').subtype(), None);
        assert_eq!(Value::Int8(0).subtype(), None);
        assert_eq!(Value::UInt8(0).subtype(), None);
        assert_eq!(Value::Int16(0).subtype(), None);
        assert_eq!(Value::UInt16(0).subtype(), None);
        assert_eq!(Value::Int32(0).subtype(), None);
        assert_eq!(Value::UInt32(0).subtype(), None);
        assert_eq!(Value::Float(0.0).subtype(), None);
        assert_eq!(Value::String(String::from("noodles")).subtype(), None);
        assert_eq!(Value::Hex(String::from("cafe")).subtype(), None);
        assert_eq!(Value::Int8Array(vec![0]).subtype(), Some(Subtype::Int8));
        assert_eq!(Value::UInt8Array(vec![0]).subtype(), Some(Subtype::UInt8));
        assert_eq!(Value::Int16Array(vec![0]).subtype(), Some(Subtype::Int16));
        assert_eq!(Value::UInt16Array(vec![0]).subtype(), Some(Subtype::UInt16));
        assert_eq!(Value::Int32Array(vec![0]).subtype(), Some(Subtype::Int32));
        assert_eq!(Value::UInt32Array(vec![0]).subtype(), Some(Subtype::UInt32));
        assert_eq!(Value::FloatArray(vec![0.0]).subtype(), Some(Subtype::Float));
    }
}
