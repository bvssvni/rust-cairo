//! Describe region overlap.

///  Used as the return value for cairo_region_contains_rectangle().
/// 
/// Since 1.10
#[repr(i32)]
pub enum Overlap {
  /// The contents are entirely inside the region. (Since 1.10)
  In = 0,
  /// The contents are entirely outside the region. (Since 1.10)
  Out = 1,
  /// The contents are partially inside and partially outside the region. (Since 1.10)
  Part = 2
}
