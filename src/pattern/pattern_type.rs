//! Describe pattern type

/// pattern::pattern_type::PatternType is used to describe the type of a given pattern.
/// 
/// The type of a pattern is determined by the function used to create it. The cairo_pattern_create_rgb() and cairo_pattern_create_rgba() functions create SOLID patterns. The remaining cairo_pattern_create functions map to pattern types in obvious ways.
/// 
/// The pattern type can be queried with cairo_pattern_get_type()
/// 
/// Most pattern::Pattern functions can be called with a pattern of any type, (though trying to change the extend or filter for a solid pattern will have no effect). A notable exception is cairo_pattern_add_color_stop_rgb() and cairo_pattern_add_color_stop_rgba() which must only be called with gradient patterns (either LINEAR or RADIAL). Otherwise the pattern will be shutdown and put into an error state.
/// 
/// New entries may be added in future versions.
/// 
/// Since 1.2
#[repr(i32)]
pub enum PatternType {
  /// The pattern is a solid (uniform) color. It may be opaque or translucent, since 1.2.
  Solid = 0,
  /// The pattern is a based on a surface (an image), since 1.2.
  Surface = 1,
  /// The pattern is a linear gradient, since 1.2.
  Linear = 2,
  /// The pattern is a radial gradient, since 1.2.
  Radial = 3,
  /// The pattern is a mesh, since 1.12.
  Mesh = 4,
  /// The pattern is a user pattern providing raster data, since 1.12.
  RasterSource = 5
}
