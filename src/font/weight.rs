//! Describe font weight

/// Specifies variants of a font face based on their weight.
/// 
/// Since 1.0i
#[repr(i32)]
pub enum Weight {
  /// Normal font weight, since 1.0
  Normal = 0,
  /// Bold font weight, since 1.0
  Bold = 1
}

