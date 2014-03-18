//! Describe font subpixel order

///  The subpixel order specifies the order of color elements within each pixel on the display device when rendering with an antialiasing mode of CAIRO_ANTIALIAS_SUBPIXEL.
/// 
/// Since 1.0
#[repr(i32)]
pub enum SubpixelOrder {
  /// Use the default subpixel order for for the target device, since 1.0
  Default = 0,
  /// Subpixel elements are arranged horizontally with red at the left, since 1.0
  RGB = 1,
  /// Subpixel elements are arranged horizontally with blue at the left, since 1.0
  BGR = 2,
  /// Subpixel elements are arranged vertically with red at the top, since 1.0
  VRGB = 3,
  /// Subpixel elements are arranged vertically with blue at the top, since 1.0
  VBGR = 4
}

