//! Describe the type of a given surface.

/// surface::surface_type::SurfaceType is used to describe the type of a given surface. The surface types are also known as "backends" or "surface backends" within cairo.
/// 
/// The type of a surface is determined by the function used to create it, which will generally be of the form cairo_type_surface_create(), (though see cairo_surface_create_similar() as well).
/// 
/// The surface type can be queried with cairo_surface_get_type()
/// 
/// The various surface::Surface functions can be used with surfaces of any type, but some backends also provide type-specific functions that must only be called with a surface of the appropriate type. These functions have names that begin with cairo_type_surface such as cairo_image_surface_get_width().
/// 
/// The behavior of calling a type-specific function with a surface of the wrong type is undefined.
/// 
/// New entries may be added in future versions.
/// 
/// Since 1.2 
#[repr(i32)]
pub enum SurfaceType {
  /// The surface is of type image, since 1.2
  Image = 0,
  /// The surface is of type pdf, since 1.2
  PDF = 1,
  /// The surface is of type ps, since 1.2
  PS = 2,
  /// The surface is of type xlib, since 1.2
  XLib = 3,
  /// The surface is of type xcb, since 1.2
  XCB = 4,
  /// The surface is of type glitz, since 1.2
  Glitz = 5,
  /// The surface is of type quartz, since 1.2
  Quartz = 6,
  /// The surface is of type win32, since 1.2
  Win32 = 7,
  /// The surface is of type beos, since 1.2
  BeOS = 8,
  /// The surface is of type directfb, since 1.2
  DirectFB = 9,
  /// The surface is of type svg, since 1.2
  SVG = 10,
  /// The surface is of type os2, since 1.4
  OS2 = 11,
  /// The surface is a win32 printing surface, since 1.6
  Win32Printing = 12,
  /// The surface is of type quartz_image, since 1.6
  QuartzImage = 13,
  /// The surface is of type script, since 1.10
  Script = 14,
  /// The surface is of type Qt, since 1.10
  Qt = 15,
  /// The surface is of type recording, since 1.10
  Recording = 16,
  /// The surface is a OpenVG surface, since 1.10
  VG = 17,
  /// The surface is of type OpenGL, since 1.10
  GL = 18,
  /// The surface is of type Direct Render Manager, since 1.10
  DRM = 19,
  /// The surface is of type 'tee' (a multiplexing surface), since 1.10
  Tee = 20,
  /// The surface is of type XML (for debugging), since 1.10
  XML = 21,
  /// The surface is of type Skia, since 1.10
  Skia = 22,
  /// The surface is a subsurface created with cairo_surface_create_for_rectangle(), since 1.10
  Subsurface = 23,
  /// This surface is of type Cogl, since 1.12
  CoGL = 24
}

