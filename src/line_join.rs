///  Specifies how to render the junction of two lines when stroking.
/// 
/// The default line join style is CAIRO_LINE_JOIN_MITER.
/// 
/// CAIRO_LINE_JOIN_MITER
/// 	use a sharp (angled) corner, see cairo_set_miter_limit() (Since 1.0)
/// 
/// CAIRO_LINE_JOIN_ROUND
/// 	use a rounded join, the center of the circle is the joint point (Since 1.0)
///
/// CAIRO_LINE_JOIN_BEVEL
///	use a cut-off join, the join is cut off at half the line width from the joint point (Since 1.0)
///
/// Since 1.0
#[repr(i32)]
pub enum LineJoin {
  Miter = 0,
  Round = 1,
  Bevel = 2
}
