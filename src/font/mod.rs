//! Rendering text and glyphs

use std;
use super::DeepClone;
use libc;

/// An opaque structure holding all options that are used when rendering fonts.
/// 
/// Individual features of a font::Options can be set or accessed using functions named cairo_font_options_set_feature_name() and cairo_font_options_get_feature_name(), like cairo_font_options_set_antialias() and cairo_font_options_get_antialias().
/// 
/// New features may be added to a font::Options in the future. For this reason, cairo_font_options_copy(), cairo_font_options_equal(), cairo_font_options_merge(), and cairo_font_options_hash() should be used to copy, check for equality, merge, or compute a hash value of font::Options objects.
/// 
/// Since 1.0
pub struct Options {
  /// Wraps the Cairo pointer for font options.
  pub opaque: *mut libc::c_void
}

/// A font::FontFace specifies all aspects of a font other than the size or font matrix (a font matrix is used to distort a font by sheering it or scaling it unequally in the two directions) . A font face can be set on a Cairo by using cairo_set_font_face(); the size and font matrix are set with cairo_set_font_size() and cairo_set_font_matrix().
/// 
/// There are various types of font faces, depending on the font backend they use. The type of a font face can be queried using cairo_font_face_get_type().
/// 
/// Memory management of font::FontFace is done with cairo_font_face_reference() and cairo_font_face_destroy().
/// 
/// Since 1.0
pub struct FontFace {
  /// Wraps the Cairo pointer for font face.
  opaque: *mut libc::c_void
}

/// A font::ScaledFont is a font scaled to a particular size and device resolution. A font::ScaledFont is most useful for low-level font usage where a library or application wants to cache a reference to a scaled font to speed up the computation of metrics.
/// 
/// There are various types of scaled fonts, depending on the font backend they use. The type of a scaled font can be queried using cairo_scaled_font_get_type().
/// 
/// Memory management of font::ScaledFont is done with cairo_scaled_font_reference() and cairo_scaled_font_destroy().
/// 
/// Since 1.0
pub struct ScaledFont {
  /// Wraps the Cairo pointer for scaled font.
  opaque: *mut libc::c_void
}

/// The font::Glyph structure holds information about a single glyph when drawing or measuring text. A font is (in simple terms) a collection of shapes used to draw text. A glyph is one of these shapes. There can be multiple glyphs for a single character (alternates to be used in different contexts, for example), or a glyph can be a ligature of multiple characters. Cairo doesn't expose any way of converting input text into glyphs, so in order to use the Cairo interfaces that take arrays of glyphs, you must directly access the appropriate underlying font system.
/// 
/// Note that the offsets given by x and y are not cumulative. When drawing or measuring text, each glyph is individually positioned with respect to the overall origin
/// 
/// Since 1.0
pub struct Glyph {
  /// glyph index in the font. The exact interpretation of the glyph index depends on the font technology being used.
  index: i64,
  /// the offset in the X direction between the origin used for drawing or measuring the string and the origin of this glyph.
  x: f64,
  /// the offset in the Y direction between the origin used for drawing or measuring the string and the origin of this glyph.
  y: f64
}

/// The font::Cluster structure holds information about a single text cluster. A text cluster is a minimal mapping of some glyphs corresponding to some UTF-8 text.
/// 
/// For a cluster to be valid, both num_bytes and num_glyphs should be non-negative, and at least one should be non-zero. Note that clusters with zero glyphs are not as well supported as normal clusters. For example, PDF rendering applications typically ignore those clusters when PDF text is being selected.
/// 
/// See cairo_show_text_glyphs() for how clusters are used in advanced text operations.
/// 
/// Since 1.8
pub struct Cluster {
  /// the number of bytes of UTF-8 text covered by cluster
  num_bytes: i32,
  /// the number of glyphs covered by cluster
  num_glyphs: i32
}

/// The font::FontExtends structure stores metric information for a font. Values are given in the current user-space coordinate system.
/// 
/// Because font metrics are in user-space coordinates, they are mostly, but not entirely, independent of the current transformation matrix. If you call cairo_scale(cr, 2.0, 2.0), text will be drawn twice as big, but the reported text extents will not be doubled. They will change slightly due to hinting (so you can't assume that metrics are independent of the transformation matrix), but otherwise will remain unchanged.
/// 
/// Since 1.0
pub struct FontExtents {
  /// the distance that the font extends above the baseline. Note that this is not always exactly equal to the maximum of the extents of all the glyphs in the font, but rather is picked to express the font designer's intent as to how the font should align with elements above it.
  ascent: f64,
  /// the distance that the font extends below the baseline. This value is positive for typical fonts that include portions below the baseline. Note that this is not always exactly equal to the maximum of the extents of all the glyphs in the font, but rather is picked to express the font designer's intent as to how the font should align with elements below it.
  descent: f64,
  /// the recommended vertical distance between baselines when setting consecutive lines of text with the font. This is greater than ascent+descent by a quantity known as the line spacing or external leading. When space is at a premium, most fonts can be set with only a distance of ascent+descent between lines.
  height: f64,
  /// the maximum distance in the X direction that the origin is advanced for any glyph in the font.
  max_x_advance: f64,
  /// the maximum distance in the Y direction that the origin is advanced for any glyph in the font. This will be zero for normal fonts used for horizontal writing. (The scripts of East Asia are sometimes written vertically.)
  max_y_advance: f64
}

/// The font::TextExtends structure stores the extents of a single glyph or a string of glyphs in user-space coordinates. Because text extents are in user-space coordinates, they are mostly, but not entirely, independent of the current transformation matrix. If you call cairo_scale(cr, 2.0, 2.0), text will be drawn twice as big, but the reported text extents will not be doubled. They will change slightly due to hinting (so you can't assume that metrics are independent of the transformation matrix), but otherwise will remain unchanged.
/// 
/// Since 1.0
pub struct TextExtents {
  /// the horizontal distance from the origin to the leftmost part of the glyphs as drawn. Positive if the glyphs lie entirely to the right of the origin.
  x_bearing: f64,
  /// the vertical distance from the origin to the topmost part of the glyphs as drawn. Positive only if the glyphs lie completely below the origin; will usually be negative.
  y_bearing: f64,
  /// width of the glyphs as drawn
  width: f64,
  /// height of the glyphs as drawn
  height: f64,
  /// distance to advance in the X direction after drawing these glyphs
  max_x_advance: f64,
  /// distance to advance in the Y direction after drawing these glyphs. Will typically be zero except for vertical text layout as found in East-Asian languages.
  max_y_advance: f64
}

impl Options {
  /// Allocates a new font options object with all options initialized to default values.
  /// 
  /// Returns : a newly allocated font::Options. Free with cairo_font_options_destroy(). This function always returns a valid pointer; if memory cannot be allocated, then a special error object is returned where all operations on the object do nothing. You can check for this with cairo_font_options_status().
  ///
  /// Since 1.0
  pub fn new() -> Options {
    unsafe {
      let foreign_result = cairo_font_options_create();
      return foreign_result;
    }
  }

  /// Checks whether an error has previously occurred for this font options object
  /// 
  /// options : a font::Options
  ///
  /// Returns : CAIRO_STATUS_SUCCESS or CAIRO_STATUS_NO_MEMORY
  ///
  /// Since 1.0
  pub fn status(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_font_options_status(self.opaque);
      return foreign_result;
    }
  }

  /// Merges non-default options from other into options, replacing existing values. This operation can be thought of as somewhat similar to compositing other onto options with the operation of CAIRO_OPERATOR_OVER.
  /// 
  /// options : a font::Options
  ///
  /// other : another font::Options
  ///
  /// Since 1.0
  pub fn merge(&mut self, other: &Options) {
    unsafe {
      cairo_font_options_merge(self.opaque, other);
    }
  }

  /// Compute a hash for the font options object; this value will be useful when storing an object containing a font::Options in a hash table.
  /// 
  /// options : a font::Options
  ///
  /// Returns : the hash value for the font options object. The return value can be cast to a 32-bit type if a 32-bit hash value is needed.
  ///
  /// Since 1.0
  pub fn hash(&mut self) -> i64 {
    unsafe {
      let foreign_result = cairo_font_options_hash(self.opaque);
      return foreign_result;
    }
  }

  /// Compares two font options objects for equality.
  /// 
  /// options : a font::Options
  ///
  /// other : another font::Options
  ///
  /// Returns : TRUE if all fields of the two font options objects match. Note that this function will return FALSE if either object is in error.
  ///
  /// Since 1.0
  pub fn equal(&mut self, other: &Options) -> bool {
    unsafe {
      let foreign_result = cairo_font_options_equal(self.opaque, other.opaque as *libc::c_void);
      return foreign_result != 0;
    }
  }

  /// Sets the antialiasing mode for the font options object. This specifies the type of antialiasing to do when rendering text.
  /// 
  /// options : a font::Options
  ///
  /// antialias : the new antialiasing mode
  ///
  /// Since 1.0
  pub fn set_antialias(&mut self, antialias: super::antialias::Antialias) {
    unsafe {
      cairo_font_options_set_antialias(self.opaque, antialias);
    }
  }

  /// Gets the antialiasing mode for the font options object.
  ///
  /// options : a font::Options
  ///
  /// Returns : the antialiasing mode
  ///
  /// Since 1.0
  pub fn get_antialias(&mut self) -> super::antialias::Antialias {
    unsafe {
      let foreign_result = cairo_font_options_get_antialias(self.opaque);
      return foreign_result;
    }
  }

  /// Sets the subpixel order for the font options object. The subpixel order specifies the order of color elements within each pixel on the display device when rendering with an antialiasing mode of CAIRO_ANTIALIAS_SUBPIXEL. See the documentation for font::subpixel_order::SubpixelOrder for full details.
  /// 
  /// options : a font::Options
  /// 
  /// subpixel_order : the new subpixel order
  /// 
  /// Since 1.0
  pub fn set_subpixel_order(&mut self, subpixel_order: subpixel_order::SubpixelOrder) {
    unsafe {
      cairo_font_options_set_subpixel_order(self.opaque, subpixel_order);
    }
  }

  /// Gets the subpixel order for the font options object. See the documentation for font::subpixel_order::SubpixelOrder for full details.
  /// 
  /// options : a font::Options
  ///
  /// Returns : the subpixel order for the font options object
  ///
  /// Since 1.0
  pub fn get_subpixel_order(&mut self) -> subpixel_order::SubpixelOrder {
    unsafe {
      let foreign_result = cairo_font_options_get_subpixel_order(self.opaque);
      return foreign_result;
    }
  }

  /// Sets the hint style for font outlines for the font options object. This controls whether to fit font outlines to the pixel grid, and if so, whether to optimize for fidelity or contrast. See the documentation for font::hint_style::HintStyle for full details.
  ///
  /// options : a font::Options
  ///
  /// hint_style : the new hint style
  ///
  /// Since 1.0
  pub fn set_hint_style(&mut self, hint_style: hint_style::HintStyle) {
    unsafe {
      cairo_font_options_set_hint_style(self.opaque, hint_style);
    }
  }

  /// Gets the hint style for font outlines for the font options object. See the documentation for font::hint_style::HintStyle for full details.
  /// 
  /// options : a font::Options
  ///
  /// Returns : the hint style for the font options object
  /// 
  /// Since 1.0
  pub fn get_hint_style(&mut self) -> hint_style::HintStyle {
    unsafe {
      let foreign_result = cairo_font_options_get_hint_style(self.opaque);
      return foreign_result;
    }
  }

  /// Sets the metrics hinting mode for the font options object. This controls whether metrics are quantized to integer values in device units. See the documentation for font::hint_metrics::HintMetrics for full details.
  /// 
  /// options : a font::Options
  ///
  /// hint_metrics : the new metrics hinting mode
  /// 
  /// Since 1.0
  pub fn set_hint_metrics(&mut self, hint_metrics: hint_metrics::HintMetrics) {
    unsafe {
      cairo_font_options_set_hint_metrics(self.opaque, hint_metrics);
    }
  }

  /// Gets the metrics hinting mode for the font options object. See the documentation for font::hint_metrics::HintMetrics for full details.
  /// 
  /// options : a font::Options
  /// 
  /// Returns : the metrics hinting mode for the font options object
  /// 
  /// Since 1.0
  pub fn get_hint_metrics(&mut self) -> hint_metrics::HintMetrics {
    unsafe {
      let foreign_result = cairo_font_options_get_hint_metrics(self.opaque);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_font_options_create() -> Options;
  fn cairo_font_options_status(self_value: *mut libc::c_void) -> super::Status;
  fn cairo_font_options_merge(self_value: *mut libc::c_void, other: *Options);
  fn cairo_font_options_hash(self_value: *mut libc::c_void) -> i64;
  fn cairo_font_options_equal(self_value: *mut libc::c_void, other: *libc::c_void) -> i32;
  fn cairo_font_options_set_antialias(self_value: *mut libc::c_void, antialias: super::antialias::Antialias);
  fn cairo_font_options_get_antialias(self_value: *mut libc::c_void) -> super::antialias::Antialias;
  fn cairo_font_options_set_subpixel_order(self_value: *mut libc::c_void, subpixel_order: subpixel_order::SubpixelOrder);
  fn cairo_font_options_get_subpixel_order(self_value: *mut libc::c_void) -> subpixel_order::SubpixelOrder;
  fn cairo_font_options_set_hint_style(self_value: *mut libc::c_void, hint_style: hint_style::HintStyle);
  fn cairo_font_options_get_hint_style(self_value: *mut libc::c_void) -> hint_style::HintStyle;
  fn cairo_font_options_set_hint_metrics(self_value: *mut libc::c_void, hint_metrics: hint_metrics::HintMetrics);
 fn cairo_font_options_get_hint_metrics(self_value: *mut libc::c_void) -> hint_metrics::HintMetrics;
}

impl std::clone::Clone for Options {
  fn clone(&self) -> Options {
    unsafe {
      let foreign_result = cairo_font_options_copy(self.opaque as *libc::c_void);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_font_options_copy(self_value: *libc::c_void) -> Options;
}

impl DeepClone for Options {
  fn deep_clone(&self) -> Options {
    unsafe {
      let foreign_result = cairo_font_options_copy(self.opaque as *libc::c_void);
      return foreign_result;
    }
  }
}

impl std::ops::Drop for Options {
  fn drop(&mut self) {
    unsafe {
      cairo_font_options_destroy(self.opaque);
    }
  }
}

extern {
  fn cairo_font_options_destroy(self_value: *mut libc::c_void);
}

impl FontFace {
  /// Creates a font face from a triplet of family, slant, and weight. These font faces are used in implementation of the the Cairo "toy" font API.
  /// 
  /// If family is the zero-length string "", the platform-specific default family is assumed. The default family then can be queried using cairo_toy_font_face_get_family().
  /// 
  /// The cairo_select_font_face() function uses this to create font faces. See that function for limitations and other details of toy font faces.
  /// 
  /// family : a font family name, encoded in UTF-8
  /// 
  /// slant : the slant for the font
  ///
  /// weight : the weight for the font
  /// 
  /// Returns : a newly created font::FontFace. Free with cairo_font_face_destroy() when you are done using it.
  /// 
  /// Since 1.8
  pub fn toy(family: &str, slant: slant::Slant, weight: weight::Weight) -> FontFace {
    unsafe {
      use std::c_str::ToCStr;
      let foreign_result = cairo_toy_font_face_create(family.to_c_str().unwrap(), slant, weight);
      return foreign_result;
    }
  }

  /// Gets the familly name of a toy font.
  /// 
  /// font_face : A toy font face
  /// 
  /// Returns : The family name. This string is owned by the font face and remains valid as long as the font face is alive (referenced).
  /// 
  /// Since 1.8
  pub fn toy_get_family(&mut self) -> std::c_str::CString {
    unsafe {
      let foreign_result = cairo_toy_font_face_get_family(self.opaque);
      return std::c_str::CString::new(foreign_result, false);
    }
  }

  /// Gets the slant a toy font.
  /// 
  /// font_face : A toy font face
  ///
  /// Returns : The slant value
  ///
  /// Since 1.8
  pub fn toy_get_slant(&mut self) -> slant::Slant {
    unsafe {
      let foreign_result = cairo_toy_font_face_get_slant(self.opaque);
      return foreign_result;
    }
  }

  /// Gets the weight a toy font.
  /// 
  /// font_face : A toy font face
  ///
  /// Returns : The weight value
  ///
  /// Since 1.8
  pub fn toy_get_weight(&mut self) -> slant::Slant {
    unsafe {
      let foreign_result = cairo_toy_font_face_get_weight(self.opaque);
      return foreign_result;
    }
  }

  /// Checks whether an error has previously occurred for this font face
  /// 
  /// font_face : a font::FontFace
  /// 
  /// Returns : CAIRO_STATUS_SUCCESS or another error such as CAIRO_STATUS_NO_MEMORY.
  /// 
  /// Since 1.0
  pub fn status(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_font_face_status(self.opaque);
      return foreign_result;
    }
  }

  /// This function returns the type of the backend used to create a font face. See font::font_type::FontType for available types.
  /// 
  /// font_face : a font face
  /// 
  /// Returns : The type of font_face.
  /// 
  /// Since 1.2
  pub fn get_type(&mut self) -> font_type::FontType {
    unsafe {
      let foreign_result = cairo_font_face_get_type(self.opaque);
      return foreign_result;
    }
  }

  /// Returns the current reference count of font_face.
  /// 
  /// font_face : a font::FontFace
  ///
  /// Returns : the current reference count of font_face. If the object is a nil object, 0 will be returned.
  ///
  /// Since 1.4
  pub fn reference_count(&mut self) -> i32 {
    unsafe {
      let foreign_result = cairo_font_face_get_reference_count(self.opaque);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_toy_font_face_create(family: *libc::c_char, slant: slant::Slant, weight: weight::Weight) -> FontFace;
  fn cairo_toy_font_face_get_family(self_value: *mut libc::c_void) -> *i8;
  fn cairo_toy_font_face_get_slant(self_value: *mut libc::c_void) -> slant::Slant;
  fn cairo_toy_font_face_get_weight(self_value: *mut libc::c_void) -> slant::Slant;
  fn cairo_font_face_status(self_value: *mut libc::c_void) -> super::Status;
  fn cairo_font_face_get_type(self_value: *mut libc::c_void) -> font_type::FontType;
  fn cairo_font_face_get_reference_count(self_value: *mut libc::c_void) -> i32;
}

impl std::clone::Clone for FontFace {
  fn clone(&self) -> FontFace {
    unsafe {
      let foreign_result = cairo_font_face_reference(self.opaque as *libc::c_void);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_font_face_reference(self_value: *libc::c_void) -> FontFace;
}

impl std::ops::Drop for FontFace {
  fn drop(&mut self) {
    unsafe {
      cairo_font_face_destroy(self.opaque);
    }
  }
}

extern {
  fn cairo_font_face_destroy(self_value: *mut libc::c_void);
}

impl ScaledFont {
  /// Creates a font::ScaledFont object from a font face and matrices that describe the size of the font and the environment in which it will be used.
  /// 
  /// font_face : a font::FontFace
  ///
  /// font_matrix : font space to user space transformation matrix for the font. In the simplest case of a N point font, this matrix is just a scale by N, but it can also be used to shear the font or stretch it unequally along the two axes. See cairo_set_font_matrix().
  /// 
  /// ctm : user to device transformation matrix with which the font will be used.
  ///
  /// options : options to use when getting metrics for the font and rendering with it.
  ///
  /// Returns : a newly created font::ScaledFont. Destroy with cairo_scaled_font_destroy()
  ///
  /// Since 1.0
  pub fn new(font_face: &mut FontFace, font_matrix: &super::matrix::Matrix, ctm: &super::matrix::Matrix, options: &mut Options) -> ScaledFont {
    unsafe {
      let foreign_result = cairo_scaled_font_create(font_face, font_matrix, ctm, options);
      return foreign_result;
    }
  }

  /// Checks whether an error has previously occurred for this scaled_font.
  /// 
  /// scaled_font : a font::ScaledFont
  ///
  /// Returns : CAIRO_STATUS_SUCCESS or another error such as CAIRO_STATUS_NO_MEMORY.
  /// 
  /// Since 1.0
  pub fn status(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_scaled_font_status(self.opaque);
      return foreign_result;
    }
  }

  /// Gets the metrics for a font::ScaledFont.
  /// 
  /// scaled_font : a font::ScaledFont
  /// 
  /// extents : a font::FontExtends which to store the retrieved extents.
  /// 
  /// Since 1.0
  pub fn font_extents(&mut self) -> FontExtents {
    unsafe {
      let mut extents:FontExtents = std::intrinsics::init();
      cairo_scaled_font_extents(self.opaque, &mut extents);
      return extents;
    }
  }

  /// Gets the extents for a string of text. The extents describe a user-space rectangle that encloses the "inked" portion of the text drawn at the origin (0,0) (as it would be drawn by cairo_show_text() if the cairo graphics state were set to the same font_face, font_matrix, ctm, and font_options as scaled_font). Additionally, the x_advance and y_advance values indicate the amount by which the current point would be advanced by cairo_show_text().
  /// 
  /// Note that whitespace characters do not directly contribute to the size of the rectangle (extents.width and extents.height). They do contribute indirectly by changing the position of non-whitespace characters. In particular, trailing whitespace characters are likely to not affect the size of the rectangle, though they will affect the x_advance and y_advance values.
  /// 
  /// scaled_font : a font::ScaledFont
  /// 
  /// utf8 : a NUL-terminated string of text, encoded in UTF-8
  /// 
  /// extents : a font::TextExtends which to store the retrieved extents.
  /// 
  /// Since 1.2
  pub fn text_extents(&mut self, utf8: &str) -> TextExtents {
    unsafe {
      let mut extents:TextExtents = std::intrinsics::init();
      cairo_scaled_font_text_extents(self.opaque, utf8.to_c_str().unwrap(), &mut extents);
      return extents;
    }
  }

  /// Gets the extents for an array of glyphs. The extents describe a user-space rectangle that encloses the "inked" portion of the glyphs, (as they would be drawn by cairo_show_glyphs() if the cairo graphics state were set to the same font_face, font_matrix, ctm, and font_options as scaled_font). Additionally, the x_advance and y_advance values indicate the amount by which the current point would be advanced by cairo_show_glyphs().
  /// 
  /// Note that whitespace glyphs do not contribute to the size of the rectangle (extents.width and extents.height).
  /// 
  /// scaled_font : a font::ScaledFont
  /// 
  /// glyphs : an array of glyph IDs with X and Y offsets.
  /// 
  /// num_glyphs : the number of glyphs in the glyphs array
  /// 
  /// extents : a font::TextExtends which to store the retrieved extents.
  /// 
  /// Since 1.0
  pub fn glyph_extents(&mut self, glyphs: &[Glyph]) -> TextExtents {
    unsafe {
      let mut extents:TextExtents = std::intrinsics::init();
      cairo_scaled_font_glyph_extents(self.opaque, glyphs.as_ptr(), glyphs.len() as i32, &mut extents);
      return extents;
    }
  }

  /// Gets the font face that this scaled font uses. This might be the font face passed to cairo_scaled_font_create(), but this does not hold true for all possible cases.
  /// 
  /// scaled_font : a font::ScaledFont
  /// 
  /// Returns : The font::FontFace with which scaled_font was created. This object is owned by cairo. To keep a reference to it, you must call cairo_scaled_font_reference().
  /// 
  /// Since 1.2
  pub fn get_font_face(&mut self) -> FontFace {
    unsafe {
      let foreign_result = cairo_scaled_font_get_font_face(self.opaque);
      return foreign_result;
    }
  }

  /// Stores the font options with which scaled_font was created into options.
  /// 
  /// scaled_font : a font::ScaledFont
  /// 
  /// options : return value for the font options
  /// 
  /// Since 1.2
  pub fn get_font_options(&mut self, options: FontExtents) {
    unsafe {
      cairo_scaled_font_get_font_options(self.opaque, options);
    }
  }

  /// Stores the font matrix with which scaled_font was created into matrix.
  /// 
  /// scaled_font :
  /// 	a font::ScaledFont
  /// 
  /// font_matrix :
  /// 	return value for the matrix
  /// 
  /// Since 1.2
  pub fn get_font_matrix(&mut self) -> super::matrix::Matrix {
    unsafe {
      let mut font_matrix:super::matrix::Matrix = std::intrinsics::init();
      cairo_scaled_font_get_font_matrix(self.opaque, &mut font_matrix);
      return font_matrix;
    }
  }

  /// Stores the CTM with which scaled_font was created into ctm. Note that the translation offsets (x0, y0) of the CTM are ignored by cairo_scaled_font_create(). So, the matrix this function returns always has 0,0 as x0,y0.
  ///
  /// scaled_font :
  /// 	a font::ScaledFont
  /// 
  /// ctm :
  /// 	return value for the CTM
  /// 
  /// Since 1.2
  pub fn get_ctm(&mut self) -> super::matrix::Matrix {
    unsafe {
      let mut ctm:super::matrix::Matrix = std::intrinsics::init();
      cairo_scaled_font_get_ctm(self.opaque, &mut ctm);
      return ctm;
    }
  }

  /// Stores the scale matrix of scaled_font into matrix. The scale matrix is product of the font matrix and the ctm associated with the scaled font, and hence is the matrix mapping from font space to device space.
  /// 
  /// scaled_font :
  /// 	a font::ScaledFont
  /// 
  /// scale_matrix :
  /// 	return value for the matrix
  /// 
  /// Since 1.8
  pub fn get_scale_matrix(&mut self) -> super::matrix::Matrix {
    unsafe {
      let mut scale_matrix:super::matrix::Matrix = std::intrinsics::init();
      cairo_scaled_font_get_scale_matrix(self.opaque, &mut scale_matrix);
      return scale_matrix;
    }
  }

  /// This function returns the type of the backend used to create a scaled font. See font::font_type::FontType for available types. However, this function never returns CAIRO_FONT_TYPE_TOY.
  /// 
  /// scaled_font :
  /// 	a font::ScaledFont
  /// 
  /// Returns :
  /// 	The type of scaled_font.
  /// 
  /// Since 1.2
  pub fn get_type(&mut self) -> font_type::FontType {
    unsafe {
      let foreign_result = cairo_scaled_font_get_type(self.opaque);
      return foreign_result;
    }
  }

  /// Returns the current reference count of scaled_font.
  /// 
  /// scaled_font :
  /// 	a font::ScaledFont
  /// 
  /// Returns :
  /// 	the current reference count of scaled_font. If the object is a nil object, 0 will be returned.
  /// 
  /// Since 1.4
  pub fn reference_count(&mut self) -> i32 {
    unsafe {
      let foreign_result = cairo_scaled_font_get_reference_count(self.opaque);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_scaled_font_create(font_face: *mut FontFace, font_matrix: *super::matrix::Matrix, ctm: *super::matrix::Matrix, options: *mut Options) -> ScaledFont;
  fn cairo_scaled_font_status(self_value: *mut libc::c_void) -> super::Status;
  fn cairo_scaled_font_extents(self_value: *mut libc::c_void, extents: *mut FontExtents);
  fn cairo_scaled_font_text_extents(self_value: *mut libc::c_void, utf8: *libc::c_char, extents: *mut TextExtents);
  fn cairo_scaled_font_glyph_extents(self_value: *mut libc::c_void, glyphs: *Glyph, glyphs_length: i32, extents: *mut TextExtents);
  fn cairo_scaled_font_get_font_face(self_value: *mut libc::c_void) -> FontFace;
  fn cairo_scaled_font_get_font_options(self_value: *mut libc::c_void, options: FontExtents);
  fn cairo_scaled_font_get_font_matrix(self_value: *mut libc::c_void, font_matrix: *mut super::matrix::Matrix);
  fn cairo_scaled_font_get_ctm(self_value: *mut libc::c_void, ctm: *mut super::matrix::Matrix);
  fn cairo_scaled_font_get_scale_matrix(self_value: *mut libc::c_void, scale_matrix: *mut super::matrix::Matrix);
  fn cairo_scaled_font_get_type(self_value: *mut libc::c_void) -> font_type::FontType;
  fn cairo_scaled_font_get_reference_count(self_value: *mut libc::c_void) -> i32;
}

impl std::clone::Clone for ScaledFont {
  fn clone(&self) -> ScaledFont {
    unsafe {
      let foreign_result = cairo_scaled_font_reference(self.opaque as *libc::c_void);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_scaled_font_reference(self_value: *libc::c_void) -> ScaledFont;
}

impl std::ops::Drop for ScaledFont {
  fn drop(&mut self) {
    unsafe {
      cairo_scaled_font_destroy(self.opaque);
    }
  }
}

extern {
  fn cairo_scaled_font_destroy(self_value: *mut libc::c_void);
}

pub mod cluster_flags;
pub mod font_type;
pub mod slant;
pub mod weight;
pub mod subpixel_order;
pub mod hint_style;
pub mod hint_metrics;

