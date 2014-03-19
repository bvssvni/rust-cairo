//! Describe filter type

/// pattern::filter::Filter is used to indicate what filtering should be applied when reading pixel values from patterns. See cairo_pattern_set_filter() for indicating the desired filter to be used with a particular pattern.
/// 
/// Since 1.0
#[repr(i32)]
pub enum Filter {
  /// A high-performance filter, with quality similar to CAIRO_FILTER_NEAREST (Since 1.0)
  Fast = 0,
  /// A reasonable-performance filter, with quality similar to CAIRO_FILTER_BILINEAR (Since 1.0)
  Good = 1,
  /// The highest-quality available, performance may not be suitable for interactive use. (Since 1.0)
  Best = 2,
  /// Nearest-neighbor filtering (Since 1.0)
  Nearest = 3,
  /// Linear interpolation in two dimensions (Since 1.0)
  Bilinear = 4,
  /// This filter value is currently unimplemented, and should not be used in current code. (Since 1.0)
  Gaussian = 5
}
