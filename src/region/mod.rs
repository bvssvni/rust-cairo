use std;
use super::DeepClone;

pub struct Region {
  opaque: *mut std::libc::c_void
}

pub struct Rectangle {
  x: i32,
  y: i32,
  width: i32,
  height: i32
}

impl Region {
  pub fn new() -> Region {
    unsafe {
      let foreign_result = cairo_region_create();
      return foreign_result;
    }
  }

  pub fn rectangle(rectangle: &Rectangle) -> Region {
    unsafe {
      let foreign_result = cairo_region_create_rectangle(rectangle);
      return foreign_result;
    }
  }

  pub fn rectangles(rectangles: &[Rectangle]) -> Region {
    unsafe {
      let foreign_result = cairo_region_create_rectangles(rectangles.as_ptr(), rectangles.len() as i32);
      return foreign_result;
    }
  }

  pub fn status(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_region_status(self.opaque);
      return foreign_result;
    }
  }

  pub fn get_extents(&mut self) -> Rectangle {
    unsafe {
      let mut extents:Rectangle = std::intrinsics::init();
      cairo_region_get_extents(self.opaque, &mut extents);
      return extents;
    }
  }

  pub fn num_rectangles(&mut self) -> i32 {
    unsafe {
      let foreign_result = cairo_region_num_rectangles(self.opaque);
      return foreign_result;
    }
  }

  pub fn get_rectangle(&mut self, nth: i32) -> Rectangle {
    unsafe {
      let mut rectangle:Rectangle = std::intrinsics::init();
      cairo_region_get_rectangle(self.opaque, nth, &mut rectangle);
      return rectangle;
    }
  }

  pub fn is_empty(&mut self) -> bool {
    unsafe {
      let foreign_result = cairo_region_is_empty(self.opaque);
      return foreign_result != 0;
    }
  }

  pub fn contains_point(&mut self, x: i32, y: i32) -> bool {
    unsafe {
      let foreign_result = cairo_region_contains_point(self.opaque, x, y);
      return foreign_result != 0;
    }
  }

  pub fn containts_rectangle(&mut self, rectangle: &Rectangle) -> overlap::Overlap {
    unsafe {
      let foreign_result = cairo_region_contains_rectangle(self.opaque, rectangle);
      return foreign_result;
    }
  }

  pub fn equal(&mut self, other: &Region) -> bool {
    unsafe {
      let foreign_result = cairo_region_equal(self.opaque, other.opaque as *std::libc::c_void);
      return foreign_result != 0;
    }
  }

  pub fn translate(&mut self, dx: i32, dy: i32) {
    unsafe {
      cairo_region_translate(self.opaque, dx, dy);
    }
  }

  pub fn intersect_rectangle(&mut self, rectangle: &Rectangle) -> super::Status {
    unsafe {
      let foreign_result = cairo_region_intersect_rectangle(self.opaque, rectangle);
      return foreign_result;
    }
  }

  pub fn subtract(&mut self, region: &Region) -> super::Status {
    unsafe {
      let foreign_result = cairo_region_subtract(self.opaque, region);
      return foreign_result;
    }
  }

  pub fn subtract_rectangle(&mut self, rectangle: &Rectangle) -> super::Status {
    unsafe {
      let foreign_result = cairo_region_subtract_rectangle(self.opaque, rectangle);
      return foreign_result;
    }
  }

  pub fn union(&mut self, region: &Region) -> super::Status {
    unsafe {
      let foreign_result = cairo_region_union(self.opaque, region);
      return foreign_result;
    }
  }

  pub fn union_rectangle(&mut self, rectangle: &Rectangle) -> super::Status {
    unsafe {
      let foreign_result = cairo_region_union_rectangle(self.opaque, rectangle);
      return foreign_result;
    }
  }

  pub fn xor(&mut self, region: &Region) -> super::Status {
    unsafe {
      let foreign_result = cairo_region_xor(self.opaque, region);
      return foreign_result;
    }
  }

  pub fn xor_rectangle(&mut self, rectangle: &Rectangle) -> super::Status {
    unsafe {
      let foreign_result = cairo_region_xor_rectangle(self.opaque, rectangle);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_region_create() -> Region;
  fn cairo_region_create_rectangle(rectangle: *Rectangle) -> Region;
  fn cairo_region_create_rectangles(rectangles: *Rectangle, rectangles_length: i32) -> Region;
  fn cairo_region_status(self_value: *mut std::libc::c_void) -> super::Status;
  fn cairo_region_get_extents(self_value: *mut std::libc::c_void, extents: *mut Rectangle);
  fn cairo_region_num_rectangles(self_value: *mut std::libc::c_void) -> i32;
  fn cairo_region_get_rectangle(self_value: *mut std::libc::c_void, nth: i32, rectangle: *mut Rectangle);
  fn cairo_region_is_empty(self_value: *mut std::libc::c_void) -> i32;
  fn cairo_region_contains_point(self_value: *mut std::libc::c_void, x: i32, y: i32) -> i32;
  fn cairo_region_contains_rectangle(self_value: *mut std::libc::c_void, rectangle: *Rectangle) -> overlap::Overlap;
  fn cairo_region_equal(self_value: *mut std::libc::c_void, other: *std::libc::c_void) -> i32;
  fn cairo_region_translate(self_value: *mut std::libc::c_void, dx: i32, dy: i32);
  fn cairo_region_intersect_rectangle(self_value: *mut std::libc::c_void, rectangle: *Rectangle) -> super::Status;
  fn cairo_region_subtract(self_value: *mut std::libc::c_void, region: *Region) -> super::Status;
  fn cairo_region_subtract_rectangle(self_value: *mut std::libc::c_void, rectangle: *Rectangle) -> super::Status;
  fn cairo_region_union(self_value: *mut std::libc::c_void, region: *Region) -> super::Status;
  fn cairo_region_union_rectangle(self_value: *mut std::libc::c_void, rectangle: *Rectangle) -> super::Status;
  fn cairo_region_xor(self_value: *mut std::libc::c_void, region: *Region) -> super::Status;
  fn cairo_region_xor_rectangle(self_value: *mut std::libc::c_void, rectangle: *Rectangle) -> super::Status;
}

impl std::clone::Clone for Region {
  fn clone(&self) -> Region {
    unsafe {
      let foreign_result = cairo_region_reference(self.opaque as *std::libc::c_void);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_region_reference(self_value: *std::libc::c_void) -> Region;
}

impl DeepClone for Region {
  fn deep_clone(&self) -> Region {
    unsafe {
      let foreign_result = cairo_region_copy(self.opaque as *std::libc::c_void);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_region_copy(self_value: *std::libc::c_void) -> Region;
}

impl std::ops::Drop for Region {
  fn drop(&mut self) {
    unsafe {
      cairo_region_destroy(self.opaque);
    }
  }
}

extern {
  fn cairo_region_destroy(self_value: *mut std::libc::c_void);
}

pub mod overlap;
