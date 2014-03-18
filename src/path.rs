use std;

pub struct Path {
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

