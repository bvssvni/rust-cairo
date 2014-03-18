//! Describe color/alpha for areas "outside" pattern's natural area

///  cairo_extend_t is used to describe how pattern color/alpha will be determined for areas "outside" the pattern's natural area, (for example, outside the surface bounds or outside the gradient geometry).
/// 
/// Mesh patterns are not affected by the extend mode.
/// 
/// The default extend mode is CAIRO_EXTEND_NONE for surface patterns and CAIRO_EXTEND_PAD for gradient patterns.
/// 
/// New entries may be added in future versions.
/// 
/// Since 1.0
#[repr(i32)]
pub enum Extend {
  /// pixels outside of the source pattern are fully transparent (Since 1.0)
  None = 0,
  /// the pattern is tiled by repeating (Since 1.0)
  Repeat = 1,
  /// the pattern is tiled by reflecting at the edges (Since 1.0; but only implemented for surface patterns since 1.6)
  Reflect = 2,
  /// pixels outside of the pattern copy the closest pixel from the source (Since 1.2; but only implemented for surface patterns since 1.6)
  Pad = 3,
}

