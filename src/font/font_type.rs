//! Describe font type

///  cairo_font_type_t is used to describe the type of a given font face or scaled font. The font types are also known as "font backends" within cairo.
/// 
/// The type of a font face is determined by the function used to create it, which will generally be of the form cairo_type_font_face_create(). The font face type can be queried with cairo_font_face_get_type()
/// 
/// The various cairo_font_face_t functions can be used with a font face of any type.
/// 
/// The type of a scaled font is determined by the type of the font face passed to cairo_scaled_font_create(). The scaled font type can be queried with cairo_scaled_font_get_type()
/// 
/// The various cairo_scaled_font_t functions can be used with scaled fonts of any type, but some font backends also provide type-specific functions that must only be called with a scaled font of the appropriate type. These functions have names that begin with cairo_type_scaled_font() such as cairo_ft_scaled_font_lock_face().
/// 
/// The behavior of calling a type-specific function with a scaled font of the wrong type is undefined.
/// 
/// New entries may be added in future versions.
/// 
/// Since 1.2
#[repr(i32)]
pub enum FontType {
  /// The font was created using cairo's toy font api (Since: 1.2)
  Toy = 0,
  /// The font is of type FreeType (Since: 1.2)
  FT = 1,
  /// The font is of type Win32 (Since: 1.2)
  Win32 = 2,
  /// The font is of type Quartz (Since: 1.6, in 1.2 and 1.4 it was named CAIRO_FONT_TYPE_ATSUI)
  Quartz = 3,
  /// The font was create using cairo's user font api (Since: 1.8)
  User = 4
}

