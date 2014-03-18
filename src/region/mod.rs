//! Representing a pixel-aligned area

use std;
use super::DeepClone;

///  A cairo_region_t represents a set of integer-aligned rectangles.
/// 
/// It allows set-theoretical operations like cairo_region_union() and cairo_region_intersect() to be performed on them.
/// 
/// Memory management of cairo_region_t is done with cairo_region_reference() and cairo_region_destroy().
/// 
/// Since 1.10
pub struct Region {
  /// Wraps Cairo pointer for region.
  opaque: *mut std::libc::c_void
}

///  A data structure for holding a rectangle with integer coordinates.
/// 
/// Since 1.10
pub struct Rectangle {
  /// X coordinate of the left side of the rectangle
  x: i32,
  /// Y coordinate of the the top side of the rectangle
  y: i32,
  /// width of the rectangle
  width: i32,
  /// height of the rectangle
  height: i32
}

impl Region {
  ///  Allocates a new empty region object.
  /// 
  /// Returns : A newly allocated cairo_region_t. Free with cairo_region_destroy(). This function always returns a valid pointer; if memory cannot be allocated, then a special error object is returned where all operations on the object do nothing. You can check for this with cairo_region_status().
  ///
  /// Since 1.10
  pub fn new() -> Region {
    unsafe {
      let foreign_result = cairo_region_create();
      return foreign_result;
    }
  }

  ///  Allocates a new region object containing rectangle.
  /// 
  /// rectangle : a cairo_rectangle_int_t
  /// 
  /// Returns : A newly allocated cairo_region_t. Free with cairo_region_destroy(). This function always returns a valid pointer; if memory cannot be allocated, then a special error object is returned where all operations on the object do nothing. You can check for this with cairo_region_status().
  /// 
  /// Since 1.10
  pub fn rectangle(rectangle: &Rectangle) -> Region {
    unsafe {
      let foreign_result = cairo_region_create_rectangle(rectangle);
      return foreign_result;
    }
  }

  ///  Allocates a new region object containing the union of all given rects.
  /// 
  /// rects : an array of count rectangles
  ///
  /// count : number of rectangles
  ///
  /// Returns : A newly allocated cairo_region_t. Free with cairo_region_destroy(). This function always returns a valid pointer; if memory cannot be allocated, then a special error object is returned where all operations on the object do nothing. You can check for this with cairo_region_status().
  /// 
  /// Since 1.10
  pub fn rectangles(rectangles: &[Rectangle]) -> Region {
    unsafe {
      let foreign_result = cairo_region_create_rectangles(rectangles.as_ptr(), rectangles.len() as i32);
      return foreign_result;
    }
  }

  ///  Checks whether an error has previous occurred for this region object.
  /// 
  /// region : a cairo_region_t
  ///
  /// Returns : CAIRO_STATUS_SUCCESS or CAIRO_STATUS_NO_MEMORY
  ///
  /// Since 1.10
  pub fn status(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_region_status(self.opaque);
      return foreign_result;
    }
  }

  ///  Gets the bounding rectangle of region as a cairo_rectangle_int_t
  /// 
  /// region : a cairo_region_t
  ///
  /// extents : rectangle into which to store the extents
  ///
  /// Since 1.10
  pub fn get_extents(&mut self) -> Rectangle {
    unsafe {
      let mut extents:Rectangle = std::intrinsics::init();
      cairo_region_get_extents(self.opaque, &mut extents);
      return extents;
    }
  }

  ///  Returns the number of rectangles contained in region.
  /// 
  /// region : a cairo_region_t
  /// 
  /// Returns : The number of rectangles contained in region.
  /// 
  /// Since 1.10
  pub fn num_rectangles(&mut self) -> i32 {
    unsafe {
      let foreign_result = cairo_region_num_rectangles(self.opaque);
      return foreign_result;
    }
  }

  ///  Stores the nth rectangle from the region in rectangle.
  /// 
  /// region : a cairo_region_t
  ///
  /// nth : a number indicating which rectangle should be returned
  ///
  /// rectangle : return location for a cairo_rectangle_int_t
  ///
  /// Since 1.10
  pub fn get_rectangle(&mut self, nth: i32) -> Rectangle {
    unsafe {
      let mut rectangle:Rectangle = std::intrinsics::init();
      cairo_region_get_rectangle(self.opaque, nth, &mut rectangle);
      return rectangle;
    }
  }

  ///  Checks whether region is empty.
  /// 
  /// region : a cairo_region_t
  ///
  /// Returns : TRUE if region is empty, FALSE if it isn't.
  ///
  /// Since 1.1
  pub fn is_empty(&mut self) -> bool {
    unsafe {
      let foreign_result = cairo_region_is_empty(self.opaque);
      return foreign_result != 0;
    }
  }

  ///  Checks whether (x, y) is contained in region.
  /// 
  /// region : a cairo_region_t
  ///
  /// x : the x coordinate of a point
  ///
  /// y : the y coordinate of a point
  ///
  /// Returns : TRUE if (x, y) is contained in region, FALSE if it is not.
  ///
  /// Since 1.10
  pub fn contains_point(&mut self, x: i32, y: i32) -> bool {
    unsafe {
      let foreign_result = cairo_region_contains_point(self.opaque, x, y);
      return foreign_result != 0;
    }
  }

  ///  Checks whether rectangle is inside, outside or partially contained in region
  /// 
  /// region : a cairo_region_t
  /// 
  /// rectangle : a cairo_rectangle_int_t
  /// 
  /// Returns : CAIRO_REGION_OVERLAP_IN if rectangle is entirely inside region, CAIRO_REGION_OVERLAP_OUT if rectangle is entirely outside region, or CAIRO_REGION_OVERLAP_PART if rectangle is partially inside and partially outside region.
  /// 
  /// Since 1.10
  pub fn containts_rectangle(&mut self, rectangle: &Rectangle) -> overlap::Overlap {
    unsafe {
      let foreign_result = cairo_region_contains_rectangle(self.opaque, rectangle);
      return foreign_result;
    }
  }

  ///  Compares whether region_a is equivalent to region_b. NULL as an argument is equal to itself, but not to any non-NULL region.
  ///
  /// a : a cairo_region_t or NULL
  ///
  /// b : a cairo_region_t or NULL
  ///
  /// Returns : TRUE if both regions contained the same coverage, FALSE if it is not or any region is in an error status.
  /// 
  /// Since 1.10
  pub fn equal(&mut self, other: &Region) -> bool {
    unsafe {
      let foreign_result = cairo_region_equal(self.opaque, other.opaque as *std::libc::c_void);
      return foreign_result != 0;
    }
  }

  ///  Translates region by (dx, dy).
  /// 
  /// region : a cairo_region_t
  ///
  /// dx : Amount to translate in the x direction
  ///
  /// dy : Amount to translate in the y direction
  ///
  /// Since 1.10
  pub fn translate(&mut self, dx: i32, dy: i32) {
    unsafe {
      cairo_region_translate(self.opaque, dx, dy);
    }
  }

  ///  Computes the intersection of dst with rectangle and places the result in dst
  /// 
  /// dst : a cairo_region_t
  ///
  /// rectangle : a cairo_rectangle_int_t
  ///
  /// Returns : CAIRO_STATUS_SUCCESS or CAIRO_STATUS_NO_MEMORY
  ///
  /// Since 1.10
  pub fn intersect_rectangle(&mut self, rectangle: &Rectangle) -> super::Status {
    unsafe {
      let foreign_result = cairo_region_intersect_rectangle(self.opaque, rectangle);
      return foreign_result;
    }
  }

  ///  Subtracts other from dst and places the result in dst
  /// 
  /// dst : a cairo_region_t
  ///
  /// other : another cairo_region_t
  /// 
  /// Returns : CAIRO_STATUS_SUCCESS or CAIRO_STATUS_NO_MEMORY
  /// 
  /// Since 1.10
  pub fn subtract(&mut self, region: &Region) -> super::Status {
    unsafe {
      let foreign_result = cairo_region_subtract(self.opaque, region);
      return foreign_result;
    }
  }

  ///  Subtracts rectangle from dst and places the result in dst
  /// 
  /// dst : a cairo_region_t
  ///
  /// rectangle : a cairo_rectangle_int_t
  ///
  /// Returns : CAIRO_STATUS_SUCCESS or CAIRO_STATUS_NO_MEMORY
  ///
  /// Since 1.10
  pub fn subtract_rectangle(&mut self, rectangle: &Rectangle) -> super::Status {
    unsafe {
      let foreign_result = cairo_region_subtract_rectangle(self.opaque, rectangle);
      return foreign_result;
    }
  }

  ///  Computes the union of dst with other and places the result in dst
  /// 
  /// dst : a cairo_region_t
  ///
  /// other : another cairo_region_t
  ///
  /// Returns : CAIRO_STATUS_SUCCESS or CAIRO_STATUS_NO_MEMORY
  ///
  /// Since 1.10
  pub fn union(&mut self, region: &Region) -> super::Status {
    unsafe {
      let foreign_result = cairo_region_union(self.opaque, region);
      return foreign_result;
    }
  }

  ///  Computes the union of dst with rectangle and places the result in dst.
  /// 
  /// dst : a cairo_region_t
  ///
  /// rectangle : a cairo_rectangle_int_t
  ///
  /// Returns : CAIRO_STATUS_SUCCESS or CAIRO_STATUS_NO_MEMORY
  ///
  /// Since 1.10
  pub fn union_rectangle(&mut self, rectangle: &Rectangle) -> super::Status {
    unsafe {
      let foreign_result = cairo_region_union_rectangle(self.opaque, rectangle);
      return foreign_result;
    }
  }

  ///  Computes the exclusive difference of dst with other and places the result in dst. That is, dst will be set to contain all areas that are either in dst or in other, but not in both.
  /// 
  /// dst : a cairo_region_t
  ///
  /// other : another cairo_region_t
  /// 
  /// Returns : CAIRO_STATUS_SUCCESS or CAIRO_STATUS_NO_MEMORY
  ///
  /// Since 1.10
  pub fn xor(&mut self, region: &Region) -> super::Status {
    unsafe {
      let foreign_result = cairo_region_xor(self.opaque, region);
      return foreign_result;
    }
  }

  ///  Computes the exclusive difference of dst with rectangle and places the result in dst. That is, dst will be set to contain all areas that are either in dst or in rectangle, but not in both.
  /// 
  /// dst : a cairo_region_t
  ///
  /// rectangle : a cairo_rectangle_int_t
  ///
  /// Returns : CAIRO_STATUS_SUCCESS or CAIRO_STATUS_NO_MEMORY
  ///
  /// Since 1.10
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
