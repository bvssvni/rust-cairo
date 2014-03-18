//! Describe type of device

///  cairo_device_type_t is used to describe the type of a given device. The devices types are also known as "backends" within cairo.
/// 
/// The device type can be queried with cairo_device_get_type()
/// 
/// The various cairo_device_t functions can be used with devices of any type, but some backends also provide type-specific functions that must only be called with a device of the appropriate type. These functions have names that begin with cairo_type_device such as cairo_xcb_device_debug_cap_xrender_version().
/// 
/// The behavior of calling a type-specific function with a device of the wrong type is undefined.
///
/// New entries may be added in future versions.
/// 
/// Since 1.10
#[repr(i32)]
pub enum DeviceType {
  /// The device is invalid, since 1.10
  Invalid = -1,
  /// The device is of type Direct Render Manager, since 1.10
  DRM = 0,
  /// The device is of type OpenGL, since 1.10
  GL = 1,
  /// The device is of type script, since 1.10
  Script = 2,
  /// The device is of type xcb, since 1.10
  XCB = 3,
  /// The device is of type xlib, since 1.10
  XLib = 4,
  /// The device is of type XML, since 1.10
  XML = 5,
  /// The device is of type cogl, since 1.12
  COGL = 6,
  /// The device is of type win32, since 1.12
  Win32 = 7
}

