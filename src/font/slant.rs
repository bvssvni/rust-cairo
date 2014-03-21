//! Describe font slant

/// Specifies variants of a font face based on their slant.
/// 
/// Since 1.0
#[repr(i32)]
pub enum Slant {
  /// Upright font style, since 1.0
  Normal = 0,
  /// Italic font style, since 1.0
  Italic = 1,
  /// Oblique font style, since 1.0
  Oblique = 2
}

