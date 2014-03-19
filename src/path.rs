//! Creating paths and manipulating path data

use std;

///  A data structure for holding a path. This data structure serves as the return value for cairo_copy_path() and cairo_copy_path_flat() as well the input value for cairo_append_path().
///
/// See cairo_path_data_t for hints on how to iterate over the actual data within the path.
///
/// The num_data member gives the number of elements in the data array. This number is larger than the number of independent path portions (defined in cairo_path_data_type_t), since the data includes both headers and coordinates for each portion.
///
/// Status status; the current error status
///
/// cairo_path_data_t *data; the elements in the path
///
/// int num_data; the number of elements in the data array
///
/// Since 1.0
pub struct Path {
  /// Wraps Cairo pointer for path.
  opaque: *mut std::libc::c_void
}

impl std::ops::Drop for Path {
  fn drop(&mut self) {
    unsafe {
      cairo_path_destroy(self.opaque);
    }
  }
}

extern {
  fn cairo_path_destroy(self_value: *mut std::libc::c_void);
}

