///  Specifies how to render the endpoints of the path when stroking.
/// 
/// The default line cap style is CAIRO_LINE_CAP_BUTT.
/// 
/// CAIRO_LINE_CAP_BUTT
///	start(stop) the line exactly at the start(end) point (Since 1.0)
///
/// CAIRO_LINE_CAP_ROUND
///	use a round ending, the center of the circle is the end point (Since 1.0)
///
/// CAIRO_LINE_CAP_SQUARE
///	use squared ending, the center of the square is the end point (Since 1.0)
/// 
/// Since 1.0
#[repr(i32)]
pub enum LineCap {
  Butt = 0,
  Round = 1,
  Square = 2
}

