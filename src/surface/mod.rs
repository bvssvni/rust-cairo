//! Rendering to surfaces

use std;

/// surface::SVGVersion is used to describe the version number of the SVG specification that a generated SVG file will conform to.
///
/// Since 1.2
#[repr(i32)]
pub enum SVGVersion {
  ///The version 1.1 of the SVG specification. (Since 1.2)
  SVGVersion_1_1 = 0,
  /// The version 1.2 of the SVG specification. (Since 1.2)
  SVGVersion_1_2 = 1
}

/// surface::Surface is the abstract type representing all different drawing targets that cairo can render to. The actual drawings are performed using a cairo context.
/// 
/// A cairo surface is created by using backend-specific constructors, typically of the form cairo_backend_surface_create().
///
/// Most surface types allow accessing the surface without using Cairo functions. If you do this, keep in mind that it is mandatory that you call cairo_surface_flush() before reading from or writing to the surface and that you must use cairo_surface_mark_dirty() after modifying it.
/// 
/// Example 1. Directly modifying an image surface
///
/// ```
/// void
/// modify_image_surface (surface::Surface *surface)
/// {
///   unsigned char *data;
///   int width, height, stride;
/// 
///   // flush to ensure all writing to the image was done
///   cairo_surface_flush (surface);
/// 
///   // modify the image
///   data = cairo_image_surface_get_data (surface);
///   width = cairo_image_surface_get_width (surface);
///   height = cairo_image_surface_get_height (surface);
///   stride = cairo_image_surface_get_stride (surface);
///   modify_image_data (data, width, height, stride);
/// 
///   // mark the image dirty so Cairo clears its caches.
///   cairo_surface_mark_dirty (surface);
/// }
/// ```
/// 
/// Note that for other surface types it might be necessary to acquire the surface's device first. See cairo_device_acquire() for a discussion of devices. 
pub struct Surface {
  /// Wraps the Cairo pointer to surface.
  opaque: *mut std::libc::c_void
}

///  A surface::Device represents the driver interface for drawing operations to a surface::Surface. There are different subtypes of surface::Device for different drawing backends; for example, cairo_egl_device_create() creates a device that wraps an EGL display and context.
/// 
/// The type of a device can be queried with cairo_device_get_type().
/// 
/// Memory management of surface::Device is done with cairo_device_reference() and cairo_device_destroy().
/// 
/// Since 1.10
pub struct Device {
  /// Wraps the Cairo pointer to device.
  opaque: *mut std::libc::c_void
}

impl Device {
  ///  Checks whether an error has previously occurred for this device.
  ///
  /// device : a surface::Device
  ///
  /// Returns : CAIRO_STATUS_SUCCESS on success or an error code if the device is in an error state.
  /// 
  /// Since 1.10
  pub fn status(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_device_status(self.opaque);
      return foreign_result;
    }
  }

  /// This function finishes the device and drops all references to external resources. All surfaces, fonts and other objects created for this device will be finished, too. Further operations on the device will not affect the device but will instead trigger a CAIRO_STATUS_DEVICE_FINISHED error.
  ///
  /// When the last call to cairo_device_destroy() decreases the reference count to zero, cairo will call cairo_device_finish() if it hasn't been called already, before freeing the resources associated with the device.
  ///
  /// This function may acquire devices.
  ///
  /// device : the surface::Device to finish
  ///
  /// Since 1.10
  pub fn finish(&mut self) {
    unsafe {
      cairo_device_finish(self.opaque);
    }
  }

  /// Finish any pending operations for the device and also restore any temporary modifications cairo has made to the device's state. This function must be called before switching from using the device with Cairo to operating on it directly with native APIs. If the device doesn't support direct access, then this function does nothing.
  /// 
  /// This function may acquire devices.
  /// 
  /// device : a surface::Device
  ///
  /// Since 1.10
  pub fn flush(&mut self) {
    unsafe {
      cairo_device_flush(self.opaque);
    }
  }

  /// This function returns the type of the device. See  surface::device_type::DeviceType for available types.
  /// 
  /// device : a surface::Device
  ///
  /// Returns : The type of device.
  ///
  /// Since 1.10
  pub fn get_type(&mut self) -> device_type::DeviceType {
    unsafe {
      let foreign_result = cairo_device_get_type(self.opaque);
      return foreign_result;
    }
  }

  /// Returns the current reference count of device.
  ///
  /// device : a surface::Device
  ///
  /// Returns : the current reference count of device. If the object is a nil object, 0 will be returned.
  ///
  /// Since 1.10
  pub fn reference_count(&mut self) -> i32 {
    unsafe {
      let foreign_result = cairo_device_get_reference_count(self.opaque);
      return foreign_result;
    }
  }

  /// Acquires the device for the current thread. This function will block until no other thread has acquired the device.
  /// 
  /// If the return value is CAIRO_STATUS_SUCCESS, you successfully acquired the device. From now on your thread owns the device and no other thread will be able to acquire it until a matching call to cairo_device_release(). It is allowed to recursively acquire the device multiple times from the same thread.
  /// 
  /// Note
  /// 
  /// You must never acquire two different devices at the same time unless this is explicitly allowed. Otherwise the possibility of deadlocks exist.
  /// 
  /// As various Cairo functions can acquire devices when called, these functions may also cause deadlocks when you call them with an acquired device. So you must not have a device acquired when calling them. These functions are marked in the documentation.
  /// 
  /// device : a surface::Device
  /// 
  /// Returns : CAIRO_STATUS_SUCCESS on success or an error code if the device is in an error state and could not be acquired. After a successful call to cairo_device_acquire(), a matching call to cairo_device_release() is required.
  /// 
  /// Since 1.10
  pub fn acquire(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_device_acquire(self.opaque);
      return foreign_result;
    }
  }

  ///  Releases a device previously acquired using cairo_device_acquire(). See that function for details.
  /// 
  /// device : a surface::Device
  /// 
  /// Since 1.10
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
  /// Create a new image surface that is as compatible as possible for uploading to and the use in conjunction with an existing surface. However, this surface can still be used like any normal image surface.
  /// 
  /// Initially the surface contents are all 0 (transparent if contents have transparency, black otherwise.)
  /// 
  /// Use cairo_surface_create_similar() if you don't need an image surface.
  /// 
  /// other : an existing surface used to select the preference of the new surface
  /// 
  /// format : the format for the new surface
  /// 
  /// width : width of the new surface, (in device-space units)
  /// 
  /// height : height of the new surface (in device-space units)
  /// 
  /// Returns :
  /// a pointer to the newly allocated image surface. The caller owns the surface and should call cairo_surface_destroy() when done with it. This function always returns a valid pointer, but it will return a pointer to a "nil" surface if other is already in an error state or any other error occurs.
  /// 
  /// Since 1.12
  pub fn similar_image(format: format::Format, width: i32, height: i32) -> Surface {
    unsafe {
      let foreign_result = cairo_surface_create_similar_image(format, width, height);
      return foreign_result;
    }
  }

  /// Create a new surface that is a rectangle within the target surface. All operations drawn to this surface are then clipped and translated onto the target surface. Nothing drawn via this sub-surface outside of its bounds is drawn onto the target surface, making this a useful method for passing constrained child surfaces to library routines that draw directly onto the parent surface, i.e. with no further backend allocations, double buffering or copies.
  ///
  /// Note
  ///
  /// The semantics of subsurfaces have not been finalized yet unless the rectangle is in full device units, is contained within the extents of the target surface, and the target or subsurface's device transforms are not changed.
  ///
  /// target : an existing surface for which the sub-surface will point to
  /// 
  /// x : the x-origin of the sub-surface from the top-left of the target surface (in device-space units)
  /// 
  /// y : the y-origin of the sub-surface from the top-left of the target surface (in device-space units)
  /// 
  /// width : width of the sub-surface (in device-space units)
  /// 
  /// height : height of the sub-surface (in device-space units)
  /// 
  /// Returns :
  /// a pointer to the newly allocated surface. The caller owns the surface and should call cairo_surface_destroy() when done with it. This function always returns a valid pointer, but it will return a pointer to a "nil" surface if other is already in an error state or any other error occurs.
  /// 
  /// Since 1.10
  pub fn for_rectangle(x: f64, y: f64, width: f64, height: f64) -> Surface {
    unsafe {
      let foreign_result = cairo_surface_create_for_rectangle(x, y, width, height);
      return foreign_result;
    }
  }

  /// Checks whether an error has previously occurred for this surface.
  ///
  /// surface : a surface::Surface
  ///
  /// Returns : CAIRO_STATUS_SUCCESS, CAIRO_STATUS_NULL_POINTER, CAIRO_STATUS_NO_MEMORY, CAIRO_STATUS_READ_ERROR, CAIRO_STATUS_INVALID_CONTENT, CAIRO_STATUS_INVALID_FORMAT, or CAIRO_STATUS_INVALID_VISUAL.
  ///
  /// Since 1.0
  pub fn status(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_surface_status(self.opaque);
      return foreign_result;
    }
  }

  /// This function finishes the surface and drops all references to external resources. For example, for the Xlib backend it means that cairo will no longer access the drawable, which can be freed. After calling cairo_surface_finish() the only valid operations on a surface are getting and setting user, referencing and destroying, and flushing and finishing it. Further drawing to the surface will not affect the surface but will instead trigger a CAIRO_STATUS_SURFACE_FINISHED error.
  ///
  /// When the last call to cairo_surface_destroy() decreases the reference count to zero, cairo will call cairo_surface_finish() if it hasn't been called already, before freeing the resources associated with the surface.
  ///
  /// surface : the surface::Surface to finish
  ///
  /// Since 1.0
  pub fn finish(&mut self) {
    unsafe {
      cairo_surface_finish(self.opaque);
    }
  }

  /// Do any pending drawing for the surface and also restore any temporary modifications cairo has made to the surface's state. This function must be called before switching from drawing on the surface with cairo to drawing on it directly with native APIs. If the surface doesn't support direct access, then this function does nothing.
  ///
  /// surface : a surface::Surface
  ///
  /// Since 1.0
  pub fn flush(&mut self) {
    unsafe {
      cairo_surface_flush(self.opaque);
    }
  }

  /// This function returns the device for a surface. See surface::Device.
  /// 
  /// surface : a surface::Surface
  ///
  /// Returns : The device for surface or NULL if the surface does not have an associated device.
  ///
  /// Since 1.10
  pub fn get_device(&mut self) -> Device {
    unsafe {
      let foreign_result = cairo_surface_get_device(self.opaque);
      return foreign_result.clone();
    }
  }

  /// Retrieves the default font rendering options for the surface. This allows display surfaces to report the correct subpixel order for rendering on them, print surfaces to disable hinting of metrics and so forth. The result can then be used with cairo_scaled_font_create().
  /// 
  /// surface : a surface::Surface
  ///
  /// options : a font::Options object into which to store the retrieved options. All existing values are overwritten
  ///
  /// Since 1.0
  pub fn get_font_options(&mut self, options: &mut super::font::Options) {
    unsafe {
      cairo_surface_get_font_options(self.opaque, options.opaque);
    }
  }

  /// This function returns the content type of surface which indicates whether the surface contains color and/or alpha information. See surface::content::Content.
  ///
  /// surface : a surface::Surface
  ///
  /// Returns : The content type of surface.
  ///
  /// Since 1.2
  pub fn get_content(&mut self) -> content::Content {
    unsafe {
      let foreign_result = cairo_surface_get_content(self.opaque);
      return foreign_result;
    }
  }

  /// Tells cairo that drawing has been done to surface using means other than cairo, and that cairo should reread any cached areas. Note that you must call cairo_surface_flush() before doing such drawing.
  /// 
  /// surface : a surface::Surface
  ///
  /// Since 1.0
  pub fn mark_dirty(&mut self) {
    unsafe {
      cairo_surface_mark_dirty(self.opaque);
    }
  }

  /// Like cairo_surface_mark_dirty(), but drawing has been done only to the specified rectangle, so that cairo can retain cached contents for other parts of the surface.
  ///
  /// Any cached clip set on the surface will be reset by this function, to make sure that future cairo calls have the clip set that they expect.
  /// 
  /// surface : a surface::Surface
  ///
  /// x : X coordinate of dirty rectangle
  ///
  /// y : Y coordinate of dirty rectangle
  ///
  /// width : width of dirty rectangle
  ///
  /// height : height of dirty rectangle
  ///
  /// Since 1.0
  pub fn mark_dirty_rectangle(&mut self, x: f64, y: f64, width: f64, height: f64) {
    unsafe {
      cairo_surface_mark_dirty_rectangle(self.opaque, x, y, width, height);
    }
  }

  /// Sets an offset that is added to the device coordinates determined by the CTM when drawing to surface. One use case for this function is when we want to create a surface::Surface that redirects drawing for a portion of an onscreen surface to an offscreen surface in a way that is completely invisible to the user of the cairo API. Setting a transformation via cairo_translate() isn't sufficient to do this, since functions like cairo_device_to_user() will expose the hidden offset.
  ///
  /// Note that the offset affects drawing to the surface as well as using the surface in a source pattern.
  /// 
  /// surface : a surface::Surface
  ///
  /// x_offset : the offset in the X direction, in device units
  /// 
  /// y_offset : the offset in the Y direction, in device units
  /// 
  /// Since 1.0
  pub fn set_device_offset(&mut self, x_offset: f64, y_offset: f64) {
    unsafe {
      cairo_surface_set_device_offset(self.opaque, x_offset, y_offset);
    }
  }

  /// This function returns the previous device offset set by cairo_surface_set_device_offset().
  /// 
  /// surface : a surface::Surface
  ///
  /// x_offset : the offset in the X direction, in device units
  ///
  /// y_offset : the offset in the Y direction, in device units
  ///
  /// Since 1.2
  pub fn get_device_offset(&mut self) -> (f64, f64) {
    unsafe {
      let mut x_offset:f64 = std::intrinsics::init();
      let mut y_offset:f64 = std::intrinsics::init();
      cairo_surface_get_device_offset(self.opaque, &mut x_offset, &mut y_offset);
      return (x_offset, y_offset);
    }
  }

  /// Set the horizontal and vertical resolution for image fallbacks.
  /// 
  /// When certain operations aren't supported natively by a backend, cairo will fallback by rendering operations to an image and then overlaying that image onto the output. For backends that are natively vector-oriented, this function can be used to set the resolution used for these image fallbacks, (larger values will result in more detailed images, but also larger file sizes).
  ///
  /// Some examples of natively vector-oriented backends are the ps, pdf, and svg backends.
  ///
  /// For backends that are natively raster-oriented, image fallbacks are still possible, but they are always performed at the native device resolution. So this function has no effect on those backends.
  ///
  /// Note: The fallback resolution only takes effect at the time of completing a page (with cairo_show_page() or cairo_copy_page()) so there is currently no way to have more than one fallback resolution in effect on a single page.
  ///
  /// The default fallback resoultion is 300 pixels per inch in both dimensions.
  ///
  /// surface : a surface::Surface
  ///
  /// x_pixels_per_inch : horizontal setting for pixels per inch
  ///
  /// y_pixels_per_inch : vertical setting for pixels per inch
  ///
  /// Since 1.2
  pub fn surface_set_fallback_resolution(&mut self, x_pixels_per_inch: f64, y_pixels_per_inch: f64) {
    unsafe {
      cairo_surface_set_fallback_resolution(self.opaque, x_pixels_per_inch, y_pixels_per_inch);
    }
  }

  /// This function returns the previous fallback resolution set by cairo_surface_set_fallback_resolution(), or default fallback resolution if never set.
  /// 
  /// surface : a surface::Surface
  ///
  /// x_pixels_per_inch : horizontal pixels per inch
  /// 
  /// y_pixels_per_inch : vertical pixels per inch
  /// 
  /// Since 1.8
  pub fn get_fallback_resolution(&mut self) -> (f64, f64) {
    unsafe {
      let mut x_pixels_per_inch:f64 = std::intrinsics::init();
      let mut y_pixels_per_inch:f64 = std::intrinsics::init();
      cairo_surface_get_fallback_resolution(self.opaque, &mut x_pixels_per_inch, &mut y_pixels_per_inch);
      return (x_pixels_per_inch, y_pixels_per_inch);
    }
  }

  /// This function returns the type of the backend used to create a surface. See surface::surface_type::SurfaceType for available types.
  /// 
  /// surface : a surface::Surface
  ///
  /// Returns : The type of surface.
  ///
  /// Since 1.2
  pub fn get_type(&mut self) -> surface_type::SurfaceType {
    unsafe {
      let foreign_result = cairo_surface_get_type(self.opaque);
      return foreign_result;
    }
  }

  /// Returns the current reference count of surface.
  ///
  /// surface : a surface::Surface
  ///
  /// Returns : the current reference count of surface. If the object is a nil object, 0 will be returned.
  ///
  /// Since 1.4
  pub fn get_reference_count(&mut self) -> i32 {
    unsafe {
      let foreign_result = cairo_surface_get_reference_count(self.opaque);
      return foreign_result;
    }
  }

  /// Emits the current page for backends that support multiple pages, but doesn't clear it, so that the contents of the current page will be retained for the next page. Use cairo_surface_show_page() if you want to get an empty page after the emission.
  ///
  /// There is a convenience function for this that takes a Cairo, namely cairo_copy_page().
  ///
  /// surface : a surface::Surface
  ///
  /// Since 1.6
  pub fn copy_page(&mut self) {
    unsafe {
      cairo_surface_copy_page(self.opaque);
    }
  }

  /// Emits and clears the current page for backends that support multiple pages. Use cairo_surface_copy_page() if you don't want to clear the page.
  ///
  /// There is a convenience function for this that takes a Cairo, namely cairo_show_page().
  ///
  /// surface : a surface::Surface
  ///
  /// Since 1.6
  pub fn show_page(&mut self) {
    unsafe {
      cairo_surface_show_page(self.opaque);
    }
  }

  /// Creates an image surface of the specified format and dimensions. Initially the surface contents are all 0. (Specifically, within each pixel, each color or alpha channel belonging to format will be 0. The contents of bits within a pixel, but not belonging to the given format are undefined).
  ///
  /// format : format of pixels in the surface to create
  ///
  /// width : width of the surface, in pixels
  ///
  /// height : height of the surface, in pixels
  ///
  /// Returns : a pointer to the newly created surface. The caller owns the surface and should call cairo_surface_destroy() when done with it. This function always returns a valid pointer, but it will return a pointer to a "nil" surface if an error such as out of memory occurs. You can use cairo_surface_status() to check for this.
  ///
  /// Since 1.0
  pub fn image(format: format::Format, width: i32, height: i32) -> Surface {
    unsafe {
      let foreign_result = cairo_image_surface_create(format, width, height);
      return foreign_result;
    }
  }

  /// Get the format of the surface.
  /// 
  /// surface : a cairo_image_surface_t
  /// 
  /// Returns : the format of the surface
  /// 
  /// Since 1.2
  pub fn get_format(&mut self) -> format::Format {
    unsafe {
      let foreign_result = cairo_image_surface_get_format(self.opaque);
      return foreign_result;
    }
  }

  /// Get the width of the image surface in pixels.
  ///
  /// surface : a cairo_image_surface_t
  /// 
  /// Returns : the width of the surface in pixels.
  ///
  /// Since 1.0
  pub fn get_width(&mut self) -> i32 {
    unsafe {
      let foreign_result = cairo_image_surface_get_width(self.opaque);
      return foreign_result;
    }
  }

  /// Get the height of the image surface in pixels.
  /// 
  /// surface : a cairo_image_surface_t
  /// 
  /// Returns : the height of the surface in pixels.
  ///
  /// Since 1.0
  pub fn get_height(&mut self) -> i32 {
    unsafe {
      let foreign_result = cairo_image_surface_get_height(self.opaque);
      return foreign_result;
    }
  }

  /// Get the stride of the image surface in bytes
  ///
  /// surface : a cairo_image_surface_t
  ///
  /// Returns : the stride of the image surface in bytes (or 0 if surface is not an image surface). The stride is the distance in bytes from the beginning of one row of the image data to the beginning of the next row.
  ///
  /// Since 1.2
  pub fn get_stride(&mut self) -> i32 {
    unsafe {
      let foreign_result = cairo_image_surface_get_stride(self.opaque);
      return foreign_result;
    }
  }

  /// Creates a new image surface and initializes the contents to the given PNG file.
  ///
  /// filename : name of PNG file to load
  ///
  /// Returns : a new surface::Surface initialized with the contents of the PNG file, or a "nil" surface if any error occurred. A nil surface can be checked for with cairo_surface_status(surface) which may return one of the following values: CAIRO_STATUS_NO_MEMORY CAIRO_STATUS_FILE_NOT_FOUND CAIRO_STATUS_READ_ERROR Alternatively, you can allow errors to propagate through the drawing operations and check the status on the context upon completion using cairo_status().
  ///
  /// Since 1.0
  pub fn png(filename: &str) -> Surface {
    unsafe {
      use std::c_str::ToCStr;
      let foreign_result = cairo_image_surface_create_from_png(filename.to_c_str().unwrap());
      return foreign_result;
    }
  }

  /// Writes the contents of surface to a new file filename as a PNG image.
  ///
  /// surface : a surface::Surface with pixel contents
  ///
  /// filename : the name of a file to write to
  ///
  /// Returns : CAIRO_STATUS_SUCCESS if the PNG file was written successfully. Otherwise, CAIRO_STATUS_NO_MEMORY if memory could not be allocated for the operation or CAIRO_STATUS_SURFACE_TYPE_MISMATCH if the surface does not have pixel contents, or CAIRO_STATUS_WRITE_ERROR if an I/O error occurs while attempting to write the file.
  ///
  /// Since 1.0
  pub fn to_png(&mut self, filename: &str) -> super::Status {
    unsafe {
      use std::c_str::ToCStr;
      let foreign_result = cairo_surface_write_to_png(self.opaque, filename.to_c_str().unwrap());
      return foreign_result;
    }
  }

  /// Creates a SVG surface of the specified size in points to be written to filename.
  /// 
  /// The SVG surface backend recognizes the following MIME types for the data attached to a surface (see cairo_surface_set_mime_data()) when it is used as a source pattern for drawing on this surface: CAIRO_MIME_TYPE_JPEG, CAIRO_MIME_TYPE_PNG, CAIRO_MIME_TYPE_URI. If any of them is specified, the SVG backend emits a href with the content of MIME data instead of a surface snapshot (PNG, Base64-encoded) in the corresponding image tag.
  /// 
  /// The unofficial MIME type CAIRO_MIME_TYPE_URI is examined first. If present, the URI is emitted as is: assuring the correctness of URI is left to the client code.
  /// 
  /// If CAIRO_MIME_TYPE_URI is not present, but CAIRO_MIME_TYPE_JPEG or CAIRO_MIME_TYPE_PNG is specified, the corresponding data is Base64-encoded and emitted.
  /// 
  /// filename : a filename for the SVG output (must be writable), NULL may be used to specify no output. This will generate a SVG surface that may be queried and used as a source, without generating a temporary file.
  /// 
  /// width_in_points : width of the surface, in points (1 point == 1/72.0 inch)
  /// 
  /// height_in_points : height of the surface, in points (1 point == 1/72.0 inch)
  /// 
  /// Returns : a pointer to the newly created surface. The caller owns the surface and should call cairo_surface_destroy() when done with it. This function always returns a valid pointer, but it will return a pointer to a "nil" surface if an error such as out of memory occurs. You can use cairo_surface_status() to check for this.
  /// 
  /// Since 1.2
  pub fn svg(filename: &str, width: f64, height: f64) -> Surface {
    unsafe {
      use std::c_str::ToCStr;
      let foreign_result = cairo_svg_surface_create(filename.to_c_str().unwrap(), width, height);
      return foreign_result;
    }
  }

  /// Restricts the generated SVG file to version. See cairo_svg_get_versions() for a list of available version values that can be used here.
  ///
  /// This function should only be called before any drawing operations have been performed on the given surface. The simplest way to do this is to call this function immediately after creating the surface.
  /// 
  /// surface : a SVG surface::Surface
  ///
  /// version : SVG version
  ///
  /// Since 1.2
  pub fn restrict_to_svg_version(&mut self, version: SVGVersion) {
    unsafe {
      cairo_svg_surface_restrict_to_version(self, version);
    }
  }

  /// Get the string representation of the given version id. This function will return NULL if version isn't valid. See cairo_svg_get_versions() for a way to get the list of valid version ids.
  /// 
  /// version : a version id
  ///
  /// Returns : the string associated to given version.
  ///
  /// Since 1.2
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

