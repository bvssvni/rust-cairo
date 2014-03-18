//! Describe how to render the endpoints of the path when stroking

///  Specifies how to render the endpoints of the path when stroking.
/// 
/// The default line cap style is CAIRO_LINE_CAP_BUTT.
/// 
/// Since 1.0
#[repr(i32)]
pub enum LineCap {
  /// start(stop) the line exactly at the start(end) point (Since 1.0)
  Butt = 0,
  /// use a round ending, the center of the circle is the end point (Since 1.0)
  Round = 1,
  /// use squared ending, the center of the square is the end point (Since 1.0)
  Square = 2
}

