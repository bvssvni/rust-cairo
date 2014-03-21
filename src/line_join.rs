//! Descirbe how to render the junction of two lines

/// Specifies how to render the junction of two lines when stroking.
/// 
/// The default line join style is CAIRO_LINE_JOIN_MITER.
/// 
/// Since 1.0
#[repr(i32)]
pub enum LineJoin {
  /// use a sharp (angled) corner, see cairo_set_miter_limit() (Since 1.0)
  Miter = 0,
  /// use a rounded join, the center of the circle is the joint point (Since 1.0)
  Round = 1,
  /// use a cut-off join, the join is cut off at half the line width from the joint point (Since 1.0)
  Bevel = 2
}
