//! Describe type of surface content

/// surface::content::Content is used to describe the content that a surface will contain, whether color information, alpha information (translucence vs. opacity), or both.
/// 
/// Note: The large values here are designed to keep surface::content::Content values distinct from surface::format::Format values so that the implementation can detect the error if users confuse the two types.
/// 
/// Since 1.0
#[repr(i32)]
pub enum Content {
  /// The surface will hold color content only. (Since 1.0)
  Color = 0x1000,
  /// The surface will hold alpha content only. (Since 1.0)
  Alpha = 0x2000,
  /// The surface will hold color and alpha content. (Since 1.0)
  ColorAlpha = 0x3000
}

