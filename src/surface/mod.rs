use std;

#[repr(i32)]
pub enum SVGVersion {
  SVGVersion_1_1 = 0,
  SVGVersion_1_2 = 1
}

pub struct Surface {
  opaque: *mut std::libc::c_void
}

pub struct Device {
  opaque: *mut std::libc::c_void
}

impl Device {
  pub fn status(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_device_status(self.opaque);
      return foreign_result;
    }
  }

  pub fn finish(&mut self) {
    unsafe {
      cairo_device_finish(self.opaque);
    }
  }

  pub fn flush(&mut self) {
    unsafe {
      cairo_device_flush(self.opaque);
    }
  }

  pub fn get_type(&mut self) -> device_type::DeviceType {
    unsafe {
      let foreign_result = cairo_device_get_type(self.opaque);
      return foreign_result;
    }
  }

  pub fn reference_type(&mut self) -> i32 {
    unsafe {
      let foreign_result = cairo_device_get_reference_count(self.opaque);
      return foreign_result;
    }
  }

  pub fn acquire(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_device_acquire(self.opaque);
      return foreign_result;
    }
  }

  pub fn release(&mut self) {
    unsafe {
      cairo_device_release(self.opaque);
    }
  }
}

extern {
  fn cairo_device_status(self_value: *mut std::libc::c_void) -> super::Status;
  fn cairo_device_finish(self_value: *mut std::libc::c_void);
  fn cairo_device_flush(self_value: *mut std::libc::c_void);
  fn cairo_device_get_type(self_value: *mut std::libc::c_void) -> device_type::DeviceType;
  fn cairo_device_get_reference_count(self_value: *mut std::libc::c_void) -> i32;
  fn cairo_device_acquire(self_value: *mut std::libc::c_void) -> super::Status;
  fn cairo_device_release(self_value: *mut std::libc::c_void);
}

impl std::clone::Clone for Device {
  fn clone(&self) -> Device {
    unsafe {
      let foreign_result = cairo_device_reference(self.opaque as *std::libc::c_void);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_device_reference(self_value: *std::libc::c_void) -> Device;
}

impl std::ops::Drop for Device {
  fn drop(&mut self) {
    unsafe {
      cairo_device_destroy(self.opaque);
    }
  }
}

extern {
  fn cairo_device_destroy(self_value: *mut std::libc::c_void);
}

impl Surface {
  pub fn similar_image(format: format::Format, width: i32, height: i32) -> Surface {
    unsafe {
      let foreign_result = cairo_surface_create_similar_image(format, width, height);
      return foreign_result;
    }
  }

  pub fn for_rectangle(x: f64, y: f64, width: f64, height: f64) -> Surface {
    unsafe {
      let foreign_result = cairo_surface_create_for_rectangle(x, y, width, height);
      return foreign_result;
    }
  }

  pub fn status(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_surface_status(self.opaque);
      return foreign_result;
    }
  }

  pub fn finish(&mut self) {
    unsafe {
      cairo_surface_finish(self.opaque);
    }
  }

  pub fn flush(&mut self) {
    unsafe {
      cairo_surface_flush(self.opaque);
    }
  }

  pub fn get_device(&mut self) -> Device {
    unsafe {
      let foreign_result = cairo_surface_get_device(self.opaque);
      return foreign_result.clone();
    }
  }

  pub fn get_font_options(&mut self, options: &mut super::font::Options) {
    unsafe {
      cairo_surface_get_font_options(self.opaque, options.opaque);
    }
  }

  pub fn get_content(&mut self) -> content::Content {
    unsafe {
      let foreign_result = cairo_surface_get_content(self.opaque);
      return foreign_result;
    }
  }

  pub fn mark_dirty(&mut self) {
    unsafe {
      cairo_surface_mark_dirty(self.opaque);
    }
  }

  pub fn mark_dirty_rectangle(&mut self, x: f64, y: f64, width: f64, height: f64) {
    unsafe {
      cairo_surface_mark_dirty_rectangle(self.opaque, x, y, width, height);
    }
  }

  pub fn set_device_offset(&mut self, x_offset: f64, y_offset: f64) {
    unsafe {
      cairo_surface_set_device_offset(self.opaque, x_offset, y_offset);
    }
  }

  pub fn get_device_offset(&mut self) -> (f64, f64) {
    unsafe {
      let mut x_offset:f64 = std::intrinsics::init();
      let mut y_offset:f64 = std::intrinsics::init();
      cairo_surface_get_device_offset(self.opaque, &mut x_offset, &mut y_offset);
      return (x_offset, y_offset);
    }
  }

  pub fn surface_set_fallback_resolution(&mut self, x_pixels_per_inch: f64, y_pixels_per_inch: f64) {
    unsafe {
      cairo_surface_set_fallback_resolution(self.opaque, x_pixels_per_inch, y_pixels_per_inch);
    }
  }

  pub fn get_fallback_resolution(&mut self) -> (f64, f64) {
    unsafe {
      let mut x_pixels_per_inch:f64 = std::intrinsics::init();
      let mut y_pixels_per_inch:f64 = std::intrinsics::init();
      cairo_surface_get_fallback_resolution(self.opaque, &mut x_pixels_per_inch, &mut y_pixels_per_inch);
      return (x_pixels_per_inch, y_pixels_per_inch);
    }
  }

  pub fn get_type(&mut self) -> surface_type::SurfaceType {
    unsafe {
      let foreign_result = cairo_surface_get_type(self.opaque);
      return foreign_result;
    }
  }

  pub fn get_reference_count(&mut self) -> i32 {
    unsafe {
      let foreign_result = cairo_surface_get_reference_count(self.opaque);
      return foreign_result;
    }
  }

  pub fn copy_page(&mut self) {
    unsafe {
      cairo_surface_copy_page(self.opaque);
    }
  }

  pub fn show_page(&mut self) {
    unsafe {
      cairo_surface_show_page(self.opaque);
    }
  }

  pub fn image(format: format::Format, width: i32, height: i32) -> Surface {
    unsafe {
      let foreign_result = cairo_image_surface_create(format, width, height);
      return foreign_result;
    }
  }

  pub fn get_format(&mut self) -> format::Format {
    unsafe {
      let foreign_result = cairo_image_surface_get_format(self.opaque);
      return foreign_result;
    }
  }

  pub fn get_width(&mut self) -> i32 {
    unsafe {
      let foreign_result = cairo_image_surface_get_width(self.opaque);
      return foreign_result;
    }
  }

  pub fn get_height(&mut self) -> i32 {
    unsafe {
      let foreign_result = cairo_image_surface_get_height(self.opaque);
      return foreign_result;
    }
  }

  pub fn get_stride(&mut self) -> i32 {
    unsafe {
      let foreign_result = cairo_image_surface_get_stride(self.opaque);
      return foreign_result;
    }
  }

  pub fn png(filename: &str) -> Surface {
    unsafe {
      use std::c_str::ToCStr;
      let foreign_result = cairo_image_surface_create_from_png(filename.to_c_str().unwrap());
      return foreign_result;
    }
  }

  pub fn to_png(&mut self, filename: &str) -> super::Status {
    unsafe {
      use std::c_str::ToCStr;
      let foreign_result = cairo_surface_write_to_png(self.opaque, filename.to_c_str().unwrap());
      return foreign_result;
    }
  }

  pub fn svg(filename: &str, width: f64, height: f64) -> Surface {
    unsafe {
      use std::c_str::ToCStr;
      let foreign_result = cairo_svg_surface_create(filename.to_c_str().unwrap(), width, height);
      return foreign_result;
    }
  }

  pub fn restrict_to_svg_version(&mut self, version: SVGVersion) {
    unsafe {
      cairo_svg_surface_restrict_to_version(self, version);
    }
  }

  pub fn svg_version_to_string(version: SVGVersion) -> std::c_str::CString {
    unsafe {
      let foreign_result = cairo_svg_version_to_string(version);
      return std::c_str::CString::new(foreign_result, false);
    }
  }
}

extern {
  fn cairo_surface_create_similar_image(format: format::Format, width: i32, height: i32) -> Surface;
  fn cairo_surface_create_for_rectangle(x: f64, y: f64, width: f64, height: f64) -> Surface;
  fn cairo_surface_status(self_value: *mut std::libc::c_void) -> super::Status;
  fn cairo_surface_finish(self_value: *mut std::libc::c_void);
  fn cairo_surface_flush(self_value: *mut std::libc::c_void);
  fn cairo_surface_get_device(self_value: *mut std::libc::c_void) -> Device;
  fn cairo_surface_get_font_options(self_value: *mut std::libc::c_void, options: *mut std::libc::c_void);
  fn cairo_surface_get_content(self_value: *mut std::libc::c_void) -> content::Content;
  fn cairo_surface_mark_dirty(self_value: *mut std::libc::c_void);
  fn cairo_surface_mark_dirty_rectangle(self_value: *mut std::libc::c_void, x: f64, y: f64, width: f64, height: f64);
  fn cairo_surface_set_device_offset(self_value: *mut std::libc::c_void, x_offset: f64, y_offset: f64);
  fn cairo_surface_get_device_offset(self_value: *mut std::libc::c_void, x_offset: *mut f64, y_offset: *mut f64);
  fn cairo_surface_set_fallback_resolution(self_value: *mut std::libc::c_void, x_pixels_per_inch: f64, y_pixels_per_inch: f64);
  fn cairo_surface_get_fallback_resolution(self_value: *mut std::libc::c_void, x_pixels_per_inch: *mut f64, y_pixels_per_inch: *mut f64);
  fn cairo_surface_get_type(self_value: *mut std::libc::c_void) -> surface_type::SurfaceType;
  fn cairo_surface_get_reference_count(self_value: *mut std::libc::c_void) -> i32;
  fn cairo_surface_copy_page(self_value: *mut std::libc::c_void);
  fn cairo_surface_show_page(self_value: *mut std::libc::c_void);
  fn cairo_image_surface_create(format: format::Format, width: i32, height: i32) -> Surface;
  fn cairo_image_surface_get_format(self_value: *mut std::libc::c_void) -> format::Format;
  fn cairo_image_surface_get_width(self_value: *mut std::libc::c_void) -> i32;
  fn cairo_image_surface_get_height(self_value: *mut std::libc::c_void) -> i32;
  fn cairo_image_surface_get_stride(self_value: *mut std::libc::c_void) -> i32;
  fn cairo_image_surface_create_from_png(filename: *std::libc::c_char) -> Surface;
  fn cairo_surface_write_to_png(self_value: *mut std::libc::c_void, filename: *std::libc::c_char) -> super::Status;
  fn cairo_svg_surface_create(filename: *std::libc::c_char, width: f64, height: f64) -> Surface;
  fn cairo_svg_surface_restrict_to_version(self_value: *mut Surface, version: SVGVersion);
  fn cairo_svg_version_to_string(version: SVGVersion) -> *i8;
}

impl std::clone::Clone for Surface {
  fn clone(&self) -> Surface {
    unsafe {
      let foreign_result = cairo_surface_reference(self.opaque as *std::libc::c_void);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_surface_reference(self_value: *std::libc::c_void) -> Surface;
}

impl std::ops::Drop for Surface {
  fn drop(&mut self) {
    unsafe {
      cairo_surface_destroy(self.opaque);
    }
  }
}

extern {
  fn cairo_surface_destroy(self_value: *mut std::libc::c_void);
}

pub mod content;
pub mod device_type;
pub mod surface_type;
pub mod format;

