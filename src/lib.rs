#[crate_id = "cairo"];
#[deny(missing_doc)];

//! A Cairo bindings library.

#[link(name = "cairo")]
extern {}

/// Was removed from std, therefore declared here.
pub trait DeepClone {
  /// Does a deep clone of the object.
  fn deep_clone(&self) -> Self;
}

/// cairo_status_t is used to indicate errors that can occur when using Cairo. In some cases it is returned directly by functions. but when using cairo_t, the last error, if any, is stored in the context and can be retrieved with cairo_status().
/// 
/// New entries may be added in future versions. Use cairo_status_to_string() to get a human-readable representation of an error message.
/// 
/// Since 1.0
#[repr(i32)]
pub enum Status {
  /// no error has occurred (Since 1.0)
  Success = 0,
  /// out of memory (Since 1.0)
  NoMemory = 1,
  /// cairo_restore() called without matching cairo_save() (Since 1.0)
  InvalidRestore = 2,
  /// no saved group to pop, i.e. cairo_pop_group() without matching cairo_push_group() (Since 1.0)
  InvalidPopGroup = 3,
  /// no current point defined (Since 1.0)
  NoCurrentPoint = 4,
  /// invalid matrix (not invertible) (Since 1.0)
  InvalidMatrix = 5,
  /// invalid value for an input cairo_status_t (Since 1.0)
  InvalidStatus = 6,
  /// NULL pointer (Since 1.0)
  NullPointer = 7,
  /// input string not valid UTF-8 (Since 1.0)
  InvalidString = 8,
  /// input path data not valid (Since 1.0)
  InvalidPathData = 9,
  /// error while reading from input stream (Since 1.0)
  ReadError = 10,
  /// error while writing to output stream (Since 1.0)
  WriteError = 11,
  /// target surface has been finished (Since 1.0)
  SurfaceFinished = 12,
  /// the surface type is not appropriate for the operation (Since 1.0)
  SurfaceTypeMismatch = 13,
  /// the pattern type is not appropriate for the operation (Since 1.0)
  PatternTypeMismatch = 14,
  /// invalid value for an input cairo_content_t (Since 1.0)
  InvalidContent = 15,
  /// invalid value for an input cairo_format_t (Since 1.0)
  InvalidFormat = 16,
  /// invalid value for an input Visual* (Since 1.0)
  InvalidVisual = 17,
  /// file not found (Since 1.0)
  FileNotFound = 18,
  /// invalid value for a dash setting (Since 1.0)
  InvalidDash = 19,
  /// invalid value for a DSC comment (Since 1.2)
  InvalidDSCComment = 21,
  /// invalid index passed to getter (Since 1.4)
  InvalidIndex = 22,
  /// clip region not representable in desired format (Since 1.4)
  ClipNotRepresentable = 23,
  /// error creating or writing to a temporary file (Since 1.6)
  TempFileError = 24,
  /// invalid value for stride (Since 1.6)
  InvalidStride = 25,
  /// the font type is not appropriate for the operation (Since 1.8)
  FontTypeMismatch = 26,
  /// the user-font is immutable (Since 1.8)
  UserFontImmutable = 27,
  /// error occurred in a user-font callback function (Since 1.8)
  UserFontError = 28,
  /// negative number used where it is not allowed (Since 1.8)
  NegativeCount = 29,
  /// input clusters do not represent the accompanying text and glyph array (Since 1.8)
  InvalidClusters = 30,
  /// invalid value for an input cairo_font_slant_t (Since 1.8)
  InvalidSlant = 31,
  /// invalid value for an input cairo_font_weight_t (Since 1.8)
  InvalidWeight = 32,
  /// invalid value (typically too big) for the size of the input (surface, pattern, etc.) (Since 1.10)
  InvalidSize = 33,
  /// user-font method not implemented (Since 1.10)
  UserFontNotImplemented = 34,
  /// the device type is not appropriate for the operation (Since 1.10)
  DeviceTypeMismatch = 35,
  /// an operation to the device caused an unspecified error (Since 1.10)
  DeviceError = 36,
  /// a mesh pattern construction operation was used outside of a cairo_mesh_pattern_begin_patch()/cairo_mesh_pattern_end_patch() pair (Since 1.12)
  InvalidMeshConstruction = 37,
  /// target device has been finished (Since 1.12)
  DeviceFinished = 38,
  /// this is a special value indicating the number of status values defined in this enumeration. When using this value, note that the version of cairo at run-time may have additional status values defined than the value of this symbol at compile-time. (Since 1.10)
  LastStatus = 39,
}

/// The cairo drawing context
pub struct Cairo {
  /// Wraps the Cairo pointer for context.
  opaque: *mut std::libc::c_void
}

impl Cairo {
  ///  Creates a new cairo_t with all graphics state parameters set to default values and with target as a target surface. The target surface should be constructed with a backend-specific function such as cairo_image_surface_create() (or any other cairo_backend_surface_create() variant).
  ///
  /// This function references target, so you can immediately call cairo_surface_destroy() on it if you don't need to maintain a separate reference to it.
  ///
  /// target : target surface for the context
  ///
  /// Returns : a newly allocated cairo_t with a reference count of 1. The initial reference count should be released with cairo_destroy() when you are done using the cairo_t. This function never returns NULL. If memory cannot be allocated, a special cairo_t object will be returned on which cairo_status() returns CAIRO_STATUS_NO_MEMORY. If you attempt to target a surface which does not support writing (such as cairo_mime_surface_t) then a CAIRO_STATUS_WRITE_ERROR will be raised. You can use this object normally, but no drawing will be done.
  ///
  /// Since 1.0
  pub fn new(surface: &mut surface::Surface) -> Cairo {
    unsafe {
      let foreign_result = cairo_create(surface.opaque);
      return foreign_result;
    }
  }

  ///  Checks whether an error has previously occurred for this context.
  ///
  /// cr : a cairo context
  ///
  /// Returns : the current status of this context, see cairo_status_t
  ///
  /// Since 1.0
  pub fn status(&mut self) -> Status {
    unsafe {
      let foreign_result = cairo_status(self.opaque);
      return foreign_result;
    }
  }

  ///  Makes a copy of the current state of cr and saves it on an internal stack of saved states for cr. When cairo_restore() is called, cr will be restored to the saved state. Multiple calls to cairo_save() and cairo_restore() can be nested; each call to cairo_restore() restores the state from the matching paired cairo_save().
  /// 
  /// It isn't necessary to clear all saved states before a cairo_t is freed. If the reference count of a cairo_t drops to zero in response to a call to cairo_destroy(), any saved states will be freed along with the cairo_t.
  ///
  /// cr : a cairo_t
  ///
  /// Since 1.0
  pub fn save(&mut self) {
    unsafe {
      cairo_save(self.opaque);
    }
  }

  ///  Restores cr to the state saved by a preceding call to cairo_save() and removes that state from the stack of saved states.
  ///
  /// cr : a cairo_t
  ///
  /// Since 1.0
  pub fn restore(&mut self) {
    unsafe {
      cairo_restore(self.opaque);
    }
  }

  ///  Gets the target surface for the cairo context as passed to cairo_create().
  ///
  /// This function will always return a valid pointer, but the result can be a "nil" surface if cr is already in an error state, (ie. cairo_status() != CAIRO_STATUS_SUCCESS). A nil surface is indicated by cairo_surface_status() != CAIRO_STATUS_SUCCESS.
  ///
  /// cr : a cairo context
  ///
  /// Returns : the target surface. This object is owned by cairo. To keep a reference to it, you must call cairo_surface_reference().
  ///
  /// Since 1.0
  pub fn get_target(&mut self) -> surface::Surface {
    unsafe {
      let foreign_result = cairo_get_target(self.opaque);
      return foreign_result.clone();
    }
  }

  ///  Temporarily redirects drawing to an intermediate surface known as a group. The redirection lasts until the group is completed by a call to cairo_pop_group() or cairo_pop_group_to_source(). These calls provide the result of any drawing to the group as a pattern, (either as an explicit object, or set as the source pattern).
  ///
  /// This group functionality can be convenient for performing intermediate compositing. One common use of a group is to render objects as opaque within the group, (so that they occlude each other), and then blend the result with translucence onto the destination.
  ///
  /// Groups can be nested arbitrarily deep by making balanced calls to cairo_push_group()/cairo_pop_group(). Each call pushes/pops the new target group onto/from a stack.
  ///
  /// The cairo_push_group() function calls cairo_save() so that any changes to the graphics state will not be visible outside the group, (the pop_group functions call cairo_restore()).
  ///
  /// By default the intermediate group will have a content type of CAIRO_CONTENT_COLOR_ALPHA. Other content types can be chosen for the group by using cairo_push_group_with_content() instead.
  ///
  /// As an example, here is how one might fill and stroke a path with translucence, but without any portion of the fill being visible under the stroke:
  ///
  /// ```
  /// cairo_push_group (cr);
  /// cairo_set_source (cr, fill_pattern);
  /// cairo_fill_preserve (cr);
  /// cairo_set_source (cr, stroke_pattern);
  /// cairo_stroke (cr);
  /// cairo_pop_group_to_source (cr);
  /// cairo_paint_with_alpha (cr, alpha);
  /// ```
  ///
  /// cr : a cairo context
  ///
  /// Since 1.2
  pub fn push_group(&mut self) {
    unsafe {
      cairo_push_group(self.opaque);
    }
  }

  ///  Temporarily redirects drawing to an intermediate surface known as a group. The redirection lasts until the group is completed by a call to cairo_pop_group() or cairo_pop_group_to_source(). These calls provide the result of any drawing to the group as a pattern, (either as an explicit object, or set as the source pattern).
  ///
  /// The group will have a content type of content. The ability to control this content type is the only distinction between this function and cairo_push_group() which you should see for a more detailed description of group rendering.
  ///
  /// cr : a cairo context
  ///
  /// content : a cairo_content_t indicating the type of group that will be created
  ///
  /// Since 1.2
  pub fn push_group_with_content(&mut self, content: surface::content::Content) {
    unsafe {
      cairo_push_group_with_content(self.opaque, content);
    }
  }

  ///  Terminates the redirection begun by a call to cairo_push_group() or cairo_push_group_with_content() and returns a new pattern containing the results of all drawing operations performed to the group.
  ///
  /// The cairo_pop_group() function calls cairo_restore(), (balancing a call to cairo_save() by the push_group function), so that any changes to the graphics state will not be visible outside the group.
  ///
  /// cr : a cairo context
  ///
  /// Returns : a newly created (surface) pattern containing the results of all drawing operations performed to the group. The caller owns the returned object and should call cairo_pattern_destroy() when finished with it.
  ///
  /// Since 1.2
  pub fn pop_group(&mut self) -> pattern::Pattern {
    unsafe {
      let foreign_result = cairo_pop_group(self.opaque);
      return foreign_result;
    }
  }

  ///  Terminates the redirection begun by a call to cairo_push_group() or cairo_push_group_with_content() and installs the resulting pattern as the source pattern in the given cairo context.
  ///
  /// The behavior of this function is equivalent to the sequence of operations:
  /// 
  /// ```
  /// cairo_pattern_t *group = cairo_pop_group (cr);
  /// cairo_set_source (cr, group);
  /// cairo_pattern_destroy (group);
  /// ```
  ///
  /// but is more convenient as their is no need for a variable to store the short-lived pointer to the pattern.
  ///
  /// The cairo_pop_group() function calls cairo_restore(), (balancing a call to cairo_save() by the push_group function), so that any changes to the graphics state will not be visible outside the group.
  ///
  /// cr : a cairo context
  ///
  /// Since 1.2
  pub fn pop_group_to_source(&mut self) {
    unsafe {
      cairo_pop_group_to_source(self.opaque);
    }
  }

  ///  Gets the current destination surface for the context. This is either the original target surface as passed to cairo_create() or the target surface for the current group as started by the most recent call to cairo_push_group() or cairo_push_group_with_content().
  ///
  /// This function will always return a valid pointer, but the result can be a "nil" surface if cr is already in an error state, (ie. cairo_status() != CAIRO_STATUS_SUCCESS). A nil surface is indicated by cairo_surface_status() != CAIRO_STATUS_SUCCESS.
  ///
  /// cr : a cairo context
  ///
  /// Returns : the target surface. This object is owned by cairo. To keep a reference to it, you must call cairo_surface_reference().
  ///
  /// Since 1.2
  pub fn get_group_target(&mut self) -> surface::Surface {
    unsafe {
      let foreign_result = cairo_get_group_target(self.opaque);
      return foreign_result.clone();
    }
  }

  ///  Sets the source pattern within cr to an opaque color. This opaque color will then be used for any subsequent drawing operation until a new source pattern is set.
  ///
  /// The color components are floating point numbers in the range 0 to 1. If the values passed in are outside that range, they will be clamped.
  ///
  /// The default source pattern is opaque black, (that is, it is equivalent to cairo_set_source_rgb(cr, 0.0, 0.0, 0.0)).
  ///
  /// cr : a cairo context
  ///
  /// red : red component of color
  ///
  /// green : green component of color
  ///
  /// blue : blue component of color
  ///
  /// Since 1.0
  pub fn set_source_rgb(&mut self, red: f64, green: f64, blue: f64) {
    unsafe {
      cairo_set_source_rgb(self.opaque, red, green, blue);
    }
  }

  ///  Sets the source pattern within cr to a translucent color. This color will then be used for any subsequent drawing operation until a new source pattern is set.
  ///
  /// The color and alpha components are floating point numbers in the range 0 to 1. If the values passed in are outside that range, they will be clamped.
  ///
  /// The default source pattern is opaque black, (that is, it is equivalent to cairo_set_source_rgba(cr, 0.0, 0.0, 0.0, 1.0)).
  ///
  /// cr : a cairo context
  ///
  /// red : red component of color
  ///
  /// green : green component of color
  ///
  /// blue : blue component of color
  ///
  /// alpha : alpha component of color
  ///
  /// Since 1.0
  pub fn set_source_rgba(&mut self, red: f64, green: f64, blue: f64, alpha: f64) {
    unsafe {
      cairo_set_source_rgba(self.opaque, red, green, blue, alpha);
    }
  }

  ///  Sets the source pattern within cr to source. This pattern will then be used for any subsequent drawing operation until a new source pattern is set.
  ///
  /// Note: The pattern's transformation matrix will be locked to the user space in effect at the time of cairo_set_source(). This means that further modifications of the current transformation matrix will not affect the source pattern. See cairo_pattern_set_matrix().
  ///
  /// The default source pattern is a solid pattern that is opaque black, (that is, it is equivalent to cairo_set_source_rgb(cr, 0.0, 0.0, 0.0)).
  ///
  /// cr : a cairo context
  ///
  /// source : a cairo_pattern_t to be used as the source for subsequent drawing operations.
  ///
  /// Since 1.0
  pub fn set_source(&mut self, source: &mut pattern::Pattern) {
    unsafe {
      cairo_set_source(self.opaque, source.opaque);
    }
  }

  ///  This is a convenience function for creating a pattern from surface and setting it as the source in cr with cairo_set_source().
  ///
  /// The x and y parameters give the user-space coordinate at which the surface origin should appear. (The surface origin is its upper-left corner before any transformation has been applied.) The x and y parameters are negated and then set as translation values in the pattern matrix.
  ///
  /// Other than the initial translation pattern matrix, as described above, all other pattern attributes, (such as its extend mode), are set to the default values as in cairo_pattern_create_for_surface(). The resulting pattern can be queried with cairo_get_source() so that these attributes can be modified if desired, (eg. to create a repeating pattern with cairo_pattern_set_extend()).
  ///
  /// cr : a cairo context
  ///
  /// surface : a surface to be used to set the source pattern
  ///
  /// x : User-space X coordinate for surface origin
  ///
  /// y : User-space Y coordinate for surface origin
  ///
  /// Since 1.0
  pub fn set_source_surface(&mut self, surface: &mut surface::Surface, x: f64, y: f64) {
    unsafe {
      cairo_set_source_surface(self.opaque, surface.opaque, x, y);
    }
  }

  ///  Gets the current source pattern for cr.
  ///
  /// cr : a cairo context
  ///
  /// Returns : the current source pattern. This object is owned by cairo. To keep a reference to it, you must call cairo_pattern_reference().
  ///
  /// Since 1.0
  pub fn get_source(&mut self) -> pattern::Pattern {
    unsafe {
      let foreign_result = cairo_get_source(self.opaque);
      return foreign_result;
    }
  }

  ///  Set the antialiasing mode of the rasterizer used for drawing shapes. This value is a hint, and a particular backend may or may not support a particular value. At the current time, no backend supports CAIRO_ANTIALIAS_SUBPIXEL when drawing shapes.
  ///
  /// Note that this option does not affect text rendering, instead see cairo_font_options_set_antialias().
  ///
  /// cr : a cairo_t
  ///
  /// antialias : the new antialiasing mode
  ///
  /// Since 1.0
  pub fn set_antialias(&mut self, antialias: antialias::Antialias) {
    unsafe {
      cairo_set_antialias(self.opaque, antialias);
    }
  }

  ///  Gets the current shape antialiasing mode, as set by cairo_set_antialias().
  ///
  /// cr : a cairo context
  ///
  /// Returns : the current shape antialiasing mode.
  ///
  /// Since 1.0
  pub fn get_antialias(&mut self) -> antialias::Antialias {
    unsafe {
      let foreign_result = cairo_get_antialias(self.opaque);
      return foreign_result;
    }
  }

  ///  Sets the dash pattern to be used by cairo_stroke(). A dash pattern is specified by dashes, an array of positive values. Each value provides the length of alternate "on" and "off" portions of the stroke. The offset specifies an offset into the pattern at which the stroke begins.
  ///
  /// Each "on" segment will have caps applied as if the segment were a separate sub-path. In particular, it is valid to use an "on" length of 0.0 with CAIRO_LINE_CAP_ROUND or CAIRO_LINE_CAP_SQUARE in order to distributed dots or squares along a path.
  ///
  /// Note: The length values are in user-space units as evaluated at the time of stroking. This is not necessarily the same as the user space at the time of cairo_set_dash().
  ///
  /// If num_dashes is 0 dashing is disabled.
  ///
  /// If num_dashes is 1 a symmetric pattern is assumed with alternating on and off portions of the size specified by the single value in dashes.
  ///
  /// If any value in dashes is negative, or if all values are 0, then cr will be put into an error state with a status of CAIRO_STATUS_INVALID_DASH.
  ///
  /// cr : a cairo context
  ///
  /// dashes : an array specifying alternate lengths of on and off stroke portions
  ///
  /// num_dashes : the length of the dashes array
  ///
  /// offset : an offset into the dash pattern at which the stroke should start
  ///
  /// Since 1.0
  pub fn set_dash(&mut self, dashes: &[f64], offset: f64) {
    unsafe {
      cairo_set_dash(self.opaque, dashes.as_ptr(), dashes.len() as i32, offset);
    }
  }

  ///  This function returns the length of the dash array in cr (0 if dashing is not currently in effect).
  ///
  /// See also cairo_set_dash() and cairo_get_dash().
  ///
  /// cr : a cairo_t
  ///
  /// Returns : the length of the dash array, or 0 if no dash array set.
  ///
  /// Since 1.4
  pub fn get_dash_count(&mut self) -> i32 {
    unsafe {
      let foreign_result = cairo_get_dash_count(self.opaque);
      return foreign_result;
    }
  }

  ///  Gets the current dash array. If not NULL, dashes should be big enough to hold at least the number of values returned by cairo_get_dash_count().
  ///
  /// cr : a cairo_t
  ///
  /// dashes : return value for the dash array, or NULL
  ///
  /// offset : return value for the current dash offset, or NULL
  ///
  /// Since 1.4
  pub fn get_dash(&mut self) -> (std::vec_ng::Vec<f64>, f64) {
    use std::vec_ng::Vec;
    unsafe {
      use std::num::Zero;
      use std::vec::MutableVector;
      let dashes_len = self.get_dash_count() as uint;
      let mut dashes:Vec<f64> = Vec::from_elem(dashes_len, Zero::zero());
      let mut offset:f64 = std::intrinsics::init();
      cairo_get_dash(self.opaque, dashes.as_mut_ptr(), &mut offset);
      return (dashes, offset);
    }
  }

  ///  Set the current fill rule within the cairo context. The fill rule is used to determine which regions are inside or outside a complex (potentially self-intersecting) path. The current fill rule affects both cairo_fill() and cairo_clip(). See cairo_fill_rule_t for details on the semantics of each available fill rule.
  ///
  /// The default fill rule is CAIRO_FILL_RULE_WINDING.
  ///
  /// cr : a cairo_t
  ///
  /// fill_rule : a fill rule, specified as a cairo_fill_rule_t
  ///
  /// Since 1.0
  pub fn set_fill_rule(&mut self, fill_rule: fill_rule::FillRule) {
    unsafe {
      cairo_set_fill_rule(self.opaque, fill_rule);
    }
  }

  ///  Gets the current fill rule, as set by cairo_set_fill_rule().
  ///
  /// cr : a cairo context
  ///
  /// Returns : the current fill rule.
  ///
  /// Since 1.0
  pub fn get_fill_rule(&mut self) -> fill_rule::FillRule {
    unsafe {
      let foreign_result = cairo_get_fill_rule(self.opaque);
      return foreign_result;
    }
  }

  ///  Sets the current line cap style within the cairo context. See cairo_line_cap_t for details about how the available line cap styles are drawn.
  ///
  /// As with the other stroke parameters, the current line cap style is examined by cairo_stroke(), cairo_stroke_extents(), and cairo_stroke_to_path(), but does not have any effect during path construction.
  ///
  /// The default line cap style is CAIRO_LINE_CAP_BUTT.
  ///
  /// cr : a cairo context
  ///
  /// line_cap :
  /// a line cap style
  ///
  /// Since 1.0
  pub fn set_line_cap(&mut self, line_cap: line_cap::LineCap) {
    unsafe {
      cairo_set_line_cap(self.opaque, line_cap);
    }
  }

  ///  Gets the current line cap style, as set by cairo_set_line_cap().
  ///
  /// cr : a cairo context
  ///
  /// Returns : the current line cap style.
  ///
  /// Since 1.0
  pub fn get_line_cap(&mut self) -> line_cap::LineCap {
    unsafe {
      let foreign_result = cairo_get_line_cap(self.opaque);
      return foreign_result;
    }
  }

  ///  Sets the current line join style within the cairo context. See cairo_line_join_t for details about how the available line join styles are drawn.
  ///
  /// As with the other stroke parameters, the current line join style is examined by cairo_stroke(), cairo_stroke_extents(), and cairo_stroke_to_path(), but does not have any effect during path construction.
  ///
  /// The default line join style is CAIRO_LINE_JOIN_MITER.
  /// 
  /// cr : a cairo context
  ///
  /// line_join : a line join style
  ///
  /// Since 1.0
  pub fn set_line_join(&mut self, line_join: line_join::LineJoin) {
    unsafe {
      cairo_set_line_join(self.opaque, line_join);
    }
  }

  ///  Gets the current line join style, as set by cairo_set_line_join().
  ///
  /// cr : a cairo context
  ///
  /// Returns : the current line join style.
  ///
  /// Since 1.0
  pub fn get_line_join(&mut self) -> line_join::LineJoin {
    unsafe {
      let foreign_result = cairo_get_line_join(self.opaque);
      return foreign_result;
    }
  }

  ///  Sets the current line width within the cairo context. The line width value specifies the diameter of a pen that is circular in user space, (though device-space pen may be an ellipse in general due to scaling/shear/rotation of the CTM).
  ///
  /// Note: When the description above refers to user space and CTM it refers to the user space and CTM in effect at the time of the stroking operation, not the user space and CTM in effect at the time of the call to cairo_set_line_width(). The simplest usage makes both of these spaces identical. That is, if there is no change to the CTM between a call to cairo_set_line_width() and the stroking operation, then one can just pass user-space values to cairo_set_line_width() and ignore this note.
  ///
  /// As with the other stroke parameters, the current line width is examined by cairo_stroke(), cairo_stroke_extents(), and cairo_stroke_to_path(), but does not have any effect during path construction.
  ///
  /// The default line width value is 2.0.
  ///
  /// cr : a cairo_t
  /// 
  /// width : a line width
  ///
  /// Since 1.0
  pub fn set_line_width(&mut self, width: f64) {
    unsafe {
      cairo_set_line_width(self.opaque, width);
    }
  }

  ///  This function returns the current line width value exactly as set by cairo_set_line_width(). Note that the value is unchanged even if the CTM has changed between the calls to cairo_set_line_width() and cairo_get_line_width().
  ///
  /// cr : a cairo context
  ///
  /// Returns : the current line width.
  ///
  /// Since 1.0
  pub fn get_line_width(&mut self) -> f64 {
    unsafe {
      let foreign_result = cairo_get_line_width(self.opaque);
      return foreign_result;
    }
  }

  ///  Sets the current miter limit within the cairo context.
  ///
  /// If the current line join style is set to CAIRO_LINE_JOIN_MITER (see cairo_set_line_join()), the miter limit is used to determine whether the lines should be joined with a bevel instead of a miter. Cairo divides the length of the miter by the line width. If the result is greater than the miter limit, the style is converted to a bevel.
  ///
  /// As with the other stroke parameters, the current line miter limit is examined by cairo_stroke(), cairo_stroke_extents(), and cairo_stroke_to_path(), but does not have any effect during path construction.
  ///
  /// The default miter limit value is 10.0, which will convert joins with interior angles less than 11 degrees to bevels instead of miters. For reference, a miter limit of 2.0 makes the miter cutoff at 60 degrees, and a miter limit of 1.414 makes the cutoff at 90 degrees.
  ///
  /// A miter limit for a desired angle can be computed as: miter limit = 1/sin(angle/2)
  ///
  /// cr : a cairo context
  ///
  /// limit : miter limit to set
  ///
  /// Since 1.0
  pub fn set_miter_limit(&mut self, limit: f64) {
    unsafe {
      cairo_set_miter_limit(self.opaque, limit);
    }
  }

  ///  Gets the current miter limit, as set by cairo_set_miter_limit().
  ///
  /// cr : a cairo context
  ///
  /// Returns : the current miter limit.
  ///
  /// Since 1.0
  pub fn get_miter_limit(&mut self) -> f64 {
    unsafe {
      let foreign_result = cairo_get_miter_limit(self.opaque);
      return foreign_result;
    }
  }

  ///  Sets the compositing operator to be used for all drawing operations. See cairo_operator_t for details on the semantics of each available compositing operator.
  ///
  /// The default operator is CAIRO_OPERATOR_OVER.
  ///
  /// cr : a cairo_t
  ///
  /// op : a compositing operator, specified as a cairo_operator_t
  ///
  /// Since 1.0
  pub fn set_operator(&mut self, operator: operator::Operator) {
    unsafe {
      cairo_set_operator(self.opaque, operator);
    }
  }

  ///  Gets the current compositing operator for a cairo context.
  ///
  /// cr : a cairo context
  ///
  /// Returns : the current compositing operator.
  ///
  /// Since 1.0
  pub fn get_operator(&mut self) -> operator::Operator {
    unsafe {
      let foreign_result = cairo_get_operator(self.opaque);
      return foreign_result;
    }
  }

  ///  Sets the tolerance used when converting paths into trapezoids. Curved segments of the path will be subdivided until the maximum deviation between the original path and the polygonal approximation is less than tolerance. The default value is 0.1. A larger value will give better performance, a smaller value, better appearance. (Reducing the value from the default value of 0.1 is unlikely to improve appearance significantly.) The accuracy of paths within Cairo is limited by the precision of its internal arithmetic, and the prescribed tolerance is restricted to the smallest representable internal value.
  ///
  /// cr : a cairo_t
  ///
  /// tolerance : the tolerance, in device units (typically pixels)
  ///
  /// Since 1.0
  pub fn set_tolerance(&mut self, tolerance: f64) {
    unsafe {
      cairo_set_tolerance(self.opaque, tolerance);
    }
  }

  ///  Gets the current tolerance value, as set by cairo_set_tolerance().
  ///
  /// cr : a cairo context
  ///
  /// Returns : the current tolerance value.
  ///
  /// Since 1.0
  pub fn get_tolerance(&mut self) -> f64 {
    unsafe {
      let foreign_result = cairo_get_tolerance(self.opaque);
      return foreign_result;
    }
  }

  ///  Establishes a new clip region by intersecting the current clip region with the current path as it would be filled by cairo_fill() and according to the current fill rule (see cairo_set_fill_rule()).
  ///
  /// After cairo_clip(), the current path will be cleared from the cairo context.
  ///
  /// The current clip region affects all drawing operations by effectively masking out any changes to the surface that are outside the current clip region.
  ///
  /// Calling cairo_clip() can only make the clip region smaller, never larger. But the current clip is part of the graphics state, so a temporary restriction of the clip region can be achieved by calling cairo_clip() within a cairo_save()/cairo_restore() pair. The only other means of increasing the size of the clip region is cairo_reset_clip().
  ///
  /// cr : a cairo context
  ///
  /// Since 1.0
  pub fn clip(&mut self) {
    unsafe {
      cairo_clip(self.opaque);
    }
  }

  ///  Establishes a new clip region by intersecting the current clip region with the current path as it would be filled by cairo_fill() and according to the current fill rule (see cairo_set_fill_rule()).
  ///
  /// Unlike cairo_clip(), cairo_clip_preserve() preserves the path within the cairo context.
  ///
  /// The current clip region affects all drawing operations by effectively masking out any changes to the surface that are outside the current clip region.
  ///
  /// Calling cairo_clip_preserve() can only make the clip region smaller, never larger. But the current clip is part of the graphics state, so a temporary restriction of the clip region can be achieved by calling cairo_clip_preserve() within a cairo_save()/cairo_restore() pair. The only other means of increasing the size of the clip region is cairo_reset_clip().
  ///
  /// cr : a cairo context
  ///
  /// Since 1.0
  pub fn clip_preserve(&mut self) {
    unsafe {
      cairo_clip_preserve(self.opaque);
    }
  }

  ///  Computes a bounding box in user coordinates covering the area inside the current clip.
  ///
  /// cr : a cairo context
  ///
  /// x1 : left of the resulting extents
  ///
  /// y1 : top of the resulting extents
  ///
  /// x2 : right of the resulting extents
  ///
  /// y2 : bottom of the resulting extents
  ///
  /// Since 1.4
  pub fn clip_extents(&mut self) -> (f64, f64, f64, f64) {
    unsafe {
      let mut x1:f64 = std::intrinsics::init();
      let mut y1:f64 = std::intrinsics::init();
      let mut x2:f64 = std::intrinsics::init();
      let mut y2:f64 = std::intrinsics::init();
      cairo_clip_extents(self.opaque, &mut x1, &mut y1, &mut x2, &mut y2);
      return (x1, y1, x2, y2);
    }
  }

  ///  Tests whether the given point is inside the area that would be visible through the current clip, i.e. the area that would be filled by a cairo_paint() operation.
  ///
  /// See cairo_clip(), and cairo_clip_preserve().
  ///
  /// cr : a cairo context
  ///
  /// x : X coordinate of the point to test
  ///
  /// y : Y coordinate of the point to test
  ///
  /// Returns : A non-zero value if the point is inside, or zero if outside.
  ///
  /// Since 1.10
  pub fn in_clip(&mut self, x: f64, y: f64) -> bool {
    unsafe {
      let foreign_result = cairo_in_clip(self.opaque, x, y);
      return foreign_result != 0;
    }
  }

  ///  Reset the current clip region to its original, unrestricted state. That is, set the clip region to an infinitely large shape containing the target surface. Equivalently, if infinity is too hard to grasp, one can imagine the clip region being reset to the exact bounds of the target surface.
  ///
  /// Note that code meant to be reusable should not call cairo_reset_clip() as it will cause results unexpected by higher-level code which calls cairo_clip(). Consider using cairo_save() and cairo_restore() around cairo_clip() as a more robust means of temporarily restricting the clip region.
  ///
  /// cr : a cairo context
  ///
  /// Since 1.0
  pub fn reset_clip(&mut self) {
    unsafe {
      cairo_reset_clip(self.opaque);
    }
  }

  ///  A drawing operator that fills the current path according to the current fill rule, (each sub-path is implicitly closed before being filled). After cairo_fill(), the current path will be cleared from the cairo context. See cairo_set_fill_rule() and cairo_fill_preserve().
  ///
  /// cr : a cairo context
  ///
  /// Since 1.0
  pub fn fill(&mut self) {
    unsafe {
      cairo_fill(self.opaque);
    }
  }

  ///  A drawing operator that fills the current path according to the current fill rule, (each sub-path is implicitly closed before being filled). Unlike cairo_fill(), cairo_fill_preserve() preserves the path within the cairo context.
  ///
  /// See cairo_set_fill_rule() and cairo_fill().
  /// 
  /// cr : a cairo context
  ///
  /// Since 1.0i
  pub fn fill_preserve(&mut self) {
    unsafe {
      cairo_fill_preserve(self.opaque);
    }
  }

  ///  Computes a bounding box in user coordinates covering the area that would be affected, (the "inked" area), by a cairo_fill() operation given the current path and fill parameters. If the current path is empty, returns an empty rectangle ((0,0), (0,0)). Surface dimensions and clipping are not taken into account.
  ///
  /// Contrast with cairo_path_extents(), which is similar, but returns non-zero extents for some paths with no inked area, (such as a simple line segment).
  ///
  /// Note that cairo_fill_extents() must necessarily do more work to compute the precise inked areas in light of the fill rule, so cairo_path_extents() may be more desirable for sake of performance if the non-inked path extents are desired.
  ///
  /// See cairo_fill(), cairo_set_fill_rule() and cairo_fill_preserve().
  ///
  /// cr : a cairo context
  ///
  /// x1 : left of the resulting extents
  ///
  /// y1 : top of the resulting extents
  ///
  /// x2 : right of the resulting extents
  ///
  /// y2 : bottom of the resulting extents
  ///
  /// Since 1.0
  pub fn fill_extents(&mut self) -> (f64, f64, f64, f64) {
    unsafe {
      let mut x1:f64 = std::intrinsics::init();
      let mut y1:f64 = std::intrinsics::init();
      let mut x2:f64 = std::intrinsics::init();
      let mut y2:f64 = std::intrinsics::init();
      cairo_fill_extents(self.opaque, &mut x1, &mut y1, &mut x2, &mut y2);
      return (x1, y1, x2, y2);
    }
  }

  ///  Tests whether the given point is inside the area that would be affected by a cairo_fill() operation given the current path and filling parameters. Surface dimensions and clipping are not taken into account.
  ///
  /// See cairo_fill(), cairo_set_fill_rule() and cairo_fill_preserve().
  ///
  /// cr : a cairo context
  ///
  /// x : X coordinate of the point to test
  ///
  /// y : Y coordinate of the point to test
  ///
  /// Returns : A non-zero value if the point is inside, or zero if outside.
  /// 
  /// Since 1.0
  pub fn in_fill(&mut self, x: f64, y: f64) -> bool {
    unsafe {
      let foreign_result = cairo_in_fill(self.opaque, x, y);
      return foreign_result != 0;
    }
  }

  ///  A drawing operator that paints the current source using the alpha channel of pattern as a mask. (Opaque areas of pattern are painted with the source, transparent areas are not painted.)
  ///
  /// cr : a cairo context
  ///
  /// pattern : a cairo_pattern_t
  ///
  /// Since 1.0
  pub fn mask(&mut self, pattern: &mut pattern::Pattern) {
    unsafe {
      cairo_mask(self.opaque, pattern.opaque);
    }
  }

  ///  A drawing operator that paints the current source using the alpha channel of surface as a mask. (Opaque areas of surface are painted with the source, transparent areas are not painted.)
  ///
  /// cr : a cairo context
  ///
  /// surface : a cairo_surface_t
  ///
  /// surface_x : X coordinate at which to place the origin of surface
  ///
  /// surface_y : Y coordinate at which to place the origin of surface
  ///
  /// Since 1.0
  pub fn mask_surface(&mut self, surface: &mut surface::Surface, surface_x: f64, surface_y: f64) {
    unsafe {
      cairo_mask_surface(self.opaque, surface.opaque, surface_x, surface_y);
    }
  }

  ///  A drawing operator that paints the current source everywhere within the current clip region.
  ///
  /// cr : a cairo context
  ///
  /// Since 1.0
  pub fn paint(&mut self) {
    unsafe {
      cairo_paint(self.opaque);
    }
  }

  ///  A drawing operator that paints the current source everywhere within the current clip region using a mask of constant alpha value alpha. The effect is similar to cairo_paint(), but the drawing is faded out using the alpha value.
  ///
  /// cr : a cairo context
  ///
  /// alpha : alpha value, between 0 (transparent) and 1 (opaque)
  ///
  /// Since 1.0
  pub fn paint_with_alpha(&mut self, alpha: f64) {
    unsafe {
      cairo_paint_with_alpha(self.opaque, alpha);
    }
  }

  ///  A drawing operator that strokes the current path according to the current line width, line join, line cap, and dash settings. After cairo_stroke(), the current path will be cleared from the cairo context. See cairo_set_line_width(), cairo_set_line_join(), cairo_set_line_cap(), cairo_set_dash(), and cairo_stroke_preserve().
  ///
  /// Note: Degenerate segments and sub-paths are treated specially and provide a useful result. These can result in two different situations:
  ///
  /// 1. Zero-length "on" segments set in cairo_set_dash(). If the cap style is CAIRO_LINE_CAP_ROUND or CAIRO_LINE_CAP_SQUARE then these segments will be drawn as circular dots or squares respectively. In the case of CAIRO_LINE_CAP_SQUARE, the orientation of the squares is determined by the direction of the underlying path.
  ///
  /// 2. A sub-path created by cairo_move_to() followed by either a cairo_close_path() or one or more calls to cairo_line_to() to the same coordinate as the cairo_move_to(). If the cap style is CAIRO_LINE_CAP_ROUND then these sub-paths will be drawn as circular dots. Note that in the case of CAIRO_LINE_CAP_SQUARE a degenerate sub-path will not be drawn at all, (since the correct orientation is indeterminate).
  ///
  /// In no case will a cap style of CAIRO_LINE_CAP_BUTT cause anything to be drawn in the case of either degenerate segments or sub-paths.
  ///
  /// cr : a cairo context
  ///
  /// Since 1.0
  pub fn stroke(&mut self) {
    unsafe {
      cairo_stroke(self.opaque);
    }
  }

  ///  A drawing operator that strokes the current path according to the current line width, line join, line cap, and dash settings. Unlike cairo_stroke(), cairo_stroke_preserve() preserves the path within the cairo context.
  ///
  /// See cairo_set_line_width(), cairo_set_line_join(), cairo_set_line_cap(), cairo_set_dash(), and cairo_stroke_preserve().
  ///
  /// cr : a cairo context
  ///
  /// Since 1.0
  pub fn stroke_preserve(&mut self) {
    unsafe {
      cairo_stroke_preserve(self.opaque);
    }
  }

  ///  Computes a bounding box in user coordinates covering the area that would be affected, (the "inked" area), by a cairo_stroke() operation given the current path and stroke parameters. If the current path is empty, returns an empty rectangle ((0,0), (0,0)). Surface dimensions and clipping are not taken into account.
  ///
  /// Note that if the line width is set to exactly zero, then cairo_stroke_extents() will return an empty rectangle. Contrast with cairo_path_extents() which can be used to compute the non-empty bounds as the line width approaches zero.
  ///
  /// Note that cairo_stroke_extents() must necessarily do more work to compute the precise inked areas in light of the stroke parameters, so cairo_path_extents() may be more desirable for sake of performance if non-inked path extents are desired.
  ///
  /// See cairo_stroke(), cairo_set_line_width(), cairo_set_line_join(), cairo_set_line_cap(), cairo_set_dash(), and cairo_stroke_preserve().
  ///
  /// cr : a cairo context
  ///
  /// x1 : left of the resulting extents
  ///
  /// y1 : top of the resulting extents
  ///
  /// x2 : right of the resulting extents
  ///
  /// y2 : bottom of the resulting extents
  ///
  /// Since 1.0
  pub fn stroke_extents(&mut self) -> (f64, f64, f64, f64) {
    unsafe {
      let mut x1:f64 = std::intrinsics::init();
      let mut y1:f64 = std::intrinsics::init();
      let mut x2:f64 = std::intrinsics::init();
      let mut y2:f64 = std::intrinsics::init();
      cairo_stroke_extents(self.opaque, &mut x1, &mut y1, &mut x2, &mut y2);
      return (x1, y1, x2, y2);
    }
  }

  ///  Tests whether the given point is inside the area that would be affected by a cairo_stroke() operation given the current path and stroking parameters. Surface dimensions and clipping are not taken into account.
  ///
  /// See cairo_stroke(), cairo_set_line_width(), cairo_set_line_join(), cairo_set_line_cap(), cairo_set_dash(), and cairo_stroke_preserve().
  ///
  /// cr : a cairo context
  ///
  /// x : X coordinate of the point to test
  ///
  /// y : Y coordinate of the point to test
  ///
  /// Returns : A non-zero value if the point is inside, or zero if outside.
  ///
  /// Since 1.0
  pub fn in_stroke(&mut self, x: f64, y: f64) -> bool {
    unsafe {
      let foreign_result = cairo_in_stroke(self.opaque, x, y);
      return foreign_result != 0;
    }
  }

  ///  Emits the current page for backends that support multiple pages, but doesn't clear it, so, the contents of the current page will be retained for the next page too. Use cairo_show_page() if you want to get an empty page after the emission.
  ///
  /// This is a convenience function that simply calls cairo_surface_copy_page() on cr's target.
  ///
  /// cr : a cairo context
  ///
  /// Since 1.0
  pub fn copy_page(&mut self) {
    unsafe {
      cairo_copy_page(self.opaque);
    }
  }

  ///  Emits and clears the current page for backends that support multiple pages. Use cairo_copy_page() if you don't want to clear the page.
  ///
  /// This is a convenience function that simply calls cairo_surface_show_page() on cr's target.
  ///
  /// cr : a cairo context
  ///
  /// Since 1.0
  pub fn show_page(&mut self) {
    unsafe {
      cairo_show_page(self.opaque);
    }
  }

  ///  Returns the current reference count of cr.
  ///
  /// cr : a cairo_t
  ///
  /// Returns : the current reference count of cr. If the object is a nil object, 0 will be returned.
  ///
  /// Since 1.4
  pub fn get_reference_count(&mut self) -> i32 {
    unsafe {
      let foreign_result = cairo_get_reference_count(self.opaque);
      return foreign_result;
    }
  }

  ///  Creates a copy of the current path and returns it to the user as a cairo_path_t. See cairo_path_data_t for hints on how to iterate over the returned data structure.
  ///
  /// This function will always return a valid pointer, but the result will have no data (data==NULL and num_data==0), if either of the following conditions hold:
  ///
  /// If there is insufficient memory to copy the path. In this case path->status will be set to CAIRO_STATUS_NO_MEMORY.
  /// If cr is already in an error state. In this case path->status will contain the same status that would be returned by cairo_status().
  ///
  /// cr : a cairo context
  ///
  /// Returns : the copy of the current path. The caller owns the returned object and should call cairo_path_destroy() when finished with it.
  ///
  /// Since 1.0
  pub fn copy_path(&mut self) -> path::Path {
    unsafe {
      let foreign_result = cairo_copy_path(self.opaque);
      return foreign_result;
    }
  }


  ///  Gets a flattened copy of the current path and returns it to the user as a cairo_path_t. See cairo_path_data_t for hints on how to iterate over the returned data structure.
  /// 
  /// This function is like cairo_copy_path() except that any curves in the path will be approximated with piecewise-linear approximations, (accurate to within the current tolerance value). That is, the result is guaranteed to not have any elements of type CAIRO_PATH_CURVE_TO which will instead be replaced by a series of CAIRO_PATH_LINE_TO elements.
  ///
  /// This function will always return a valid pointer, but the result will have no data (data==NULL and num_data==0), if either of the following conditions hold:
  ///
  /// If there is insufficient memory to copy the path. In this case path->status will be set to CAIRO_STATUS_NO_MEMORY.
  /// If cr is already in an error state. In this case path->status will contain the same status that would be returned by cairo_status().
  ///
  /// cr : a cairo context
  ///
  /// Returns : the copy of the current path. The caller owns the returned object and should call cairo_path_destroy() when finished with it.
  ///
  /// Since 1.0
  pub fn copy_path_flat(&mut self) -> path::Path {
    unsafe {
      let foreign_result = cairo_copy_path_flat(self.opaque);
      return foreign_result;
    }
  }

  ///  Append the path onto the current path. The path may be either the return value from one of cairo_copy_path() or cairo_copy_path_flat() or it may be constructed manually. See cairo_path_t for details on how the path data structure should be initialized, and note that path->status must be initialized to CAIRO_STATUS_SUCCESS.
  ///
  /// cr : a cairo context
  ///
  /// path : path to be appended
  ///
  /// Since 1.0
  pub fn append_path(&mut self, path: &path::Path) {
    unsafe {
      cairo_append_path(self.opaque, path.opaque as *std::libc::c_void);
    }
  }

  ///  Returns whether a current point is defined on the current path. See cairo_get_current_point() for details on the current point.
  ///
  /// cr : a cairo context
  ///
  /// Returns : whether a current point is defined.
  ///
  /// Since 1.6
  pub fn has_current_point(&mut self) -> bool {
    unsafe {
      let foreign_result = cairo_has_current_point(self.opaque);
      return foreign_result != 0;
    }
  }

  ///  Gets the current point of the current path, which is conceptually the final point reached by the path so far.
  ///
  /// The current point is returned in the user-space coordinate system. If there is no defined current point or if cr is in an error status, x and y will both be set to 0.0. It is possible to check this in advance with cairo_has_current_point().
  ///
  /// Most path construction functions alter the current point. See the following for details on how they affect the current point: cairo_new_path(), cairo_new_sub_path(), cairo_append_path(), cairo_close_path(), cairo_move_to(), cairo_line_to(), cairo_curve_to(), cairo_rel_move_to(), cairo_rel_line_to(), cairo_rel_curve_to(), cairo_arc(), cairo_arc_negative(), cairo_rectangle(), cairo_text_path(), cairo_glyph_path(), cairo_stroke_to_path().
  ///
  /// Some functions use and alter the current point but do not otherwise change current path: cairo_show_text().
  ///
  /// Some functions unset the current path and as a result, current point: cairo_fill(), cairo_stroke().
  ///
  /// cr : a cairo context
  ///
  /// x : return value for X coordinate of the current point
  ///
  /// y : return value for Y coordinate of the current point
  ///
  /// Since 1.0
  pub fn get_current_point(&mut self) -> (f64, f64) {
    unsafe {
      let mut x:f64 = std::intrinsics::init();
      let mut y:f64 = std::intrinsics::init();
      cairo_get_current_point(self.opaque, &mut x, &mut y);
      return (x, y);
    }
  }

  ///  Clears the current path. After this call there will be no path and no current point.
  ///
  /// cr : a cairo context
  ///
  /// Since 1.0
  pub fn new_path(&mut self) {
    unsafe {
      cairo_new_path(self.opaque);
    }
  }

  ///  Begin a new sub-path. Note that the existing path is not affected. After this call there will be no current point.
  ///
  /// In many cases, this call is not needed since new sub-paths are frequently started with cairo_move_to().
  ///
  /// A call to cairo_new_sub_path() is particularly useful when beginning a new sub-path with one of the cairo_arc() calls. This makes things easier as it is no longer necessary to manually compute the arc's initial coordinates for a call to cairo_move_to().
  ///
  /// cr : a cairo context
  ///
  /// Since 1.2
  pub fn new_sub_path(&mut self) {
    unsafe {
      cairo_new_sub_path(self.opaque);
    }
  }

  ///  Adds a line segment to the path from the current point to the beginning of the current sub-path, (the most recent point passed to cairo_move_to()), and closes this sub-path. After this call the current point will be at the joined endpoint of the sub-path.
  ///
  /// The behavior of cairo_close_path() is distinct from simply calling cairo_line_to() with the equivalent coordinate in the case of stroking. When a closed sub-path is stroked, there are no caps on the ends of the sub-path. Instead, there is a line join connecting the final and initial segments of the sub-path.
  ///
  /// If there is no current point before the call to cairo_close_path(), this function will have no effect.
  /// 
  /// Note: As of cairo version 1.2.4 any call to cairo_close_path() will place an explicit MOVE_TO element into the path immediately after the CLOSE_PATH element, (which can be seen in cairo_copy_path() for example). This can simplify path processing in some cases as it may not be necessary to save the "last move_to point" during processing as the MOVE_TO immediately after the CLOSE_PATH will provide that point.
  ///
  /// cr : a cairo context
  ///
  /// Since 1.0
  pub fn close_path(&mut self) {
    unsafe {
      cairo_close_path(self.opaque);
    }
  }

  ///  Adds a circular arc of the given radius to the current path. The arc is centered at (xc, yc), begins at angle1 and proceeds in the direction of increasing angles to end at angle2. If angle2 is less than angle1 it will be progressively increased by 2*M_PI until it is greater than angle1.
  ///
  /// If there is a current point, an initial line segment will be added to the path to connect the current point to the beginning of the arc. If this initial line is undesired, it can be avoided by calling cairo_new_sub_path() before calling cairo_arc().
  ///
  /// Angles are measured in radians. An angle of 0.0 is in the direction of the positive X axis (in user space). An angle of M_PI/2.0 radians (90 degrees) is in the direction of the positive Y axis (in user space). Angles increase in the direction from the positive X axis toward the positive Y axis. So with the default transformation matrix, angles increase in a clockwise direction.
  ///
  /// (To convert from degrees to radians, use degrees * (M_PI / 180.).)
  ///
  /// This function gives the arc in the direction of increasing angles; see cairo_arc_negative() to get the arc in the direction of decreasing angles.
  ///
  /// The arc is circular in user space. To achieve an elliptical arc, you can scale the current transformation matrix by different amounts in the X and Y directions. For example, to draw an ellipse in the box given by x, y, width, height:
  ///
  /// ```
  /// cairo_save (cr);
  /// cairo_translate (cr, x + width / 2., y + height / 2.);
  /// cairo_scale (cr, width / 2., height / 2.);
  /// cairo_arc (cr, 0., 0., 1., 0., 2 * M_PI);
  /// cairo_restore (cr);
  /// ```
  ///
  /// cr : a cairo context
  ///
  /// xc : X position of the center of the arc
  ///
  /// yc : Y position of the center of the arc
  ///
  /// radius : the radius of the arc
  ///
  /// angle1 : the start angle, in radians
  ///
  /// angle2 : the end angle, in radians
  ///
  /// Since 1.0
  pub fn arc(&mut self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64) {
    unsafe {
      cairo_arc(self.opaque, xc, yc, radius, angle1, angle2);
    }
  }

  ///  Adds a circular arc of the given radius to the current path. The arc is centered at (xc, yc), begins at angle1 and proceeds in the direction of decreasing angles to end at angle2. If angle2 is greater than angle1 it will be progressively decreased by 2*M_PI until it is less than angle1.
  ///
  /// See cairo_arc() for more details. This function differs only in the direction of the arc between the two angles.
  ///
  /// cr : a cairo context
  ///
  /// xc : X position of the center of the arc
  ///
  /// yc : Y position of the center of the arc
  ///
  /// radius : the radius of the arc
  ///
  /// angle1 : the start angle, in radians
  ///
  /// angle2 : the end angle, in radians
  ///
  /// Since 1.0
  pub fn arc_negative(&mut self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64) {
    unsafe {
      cairo_arc_negative(self.opaque, xc, yc, radius, angle1, angle2);
    }
  }


  ///  Adds a cubic Bézier spline to the path from the current point to position (x3, y3) in user-space coordinates, using (x1, y1) and (x2, y2) as the control points. After this call the current point will be (x3, y3).
  ///
  /// If there is no current point before the call to cairo_curve_to() this function will behave as if preceded by a call to cairo_move_to(cr, x1, y1).
  ///
  /// cr : a cairo context
  ///
  /// x1 : the X coordinate of the first control point
  ///
  /// y1 : the Y coordinate of the first control point
  ///
  /// x2 : the X coordinate of the second control point
  ///
  /// y2 : the Y coordinate of the second control point
  ///
  /// x3 : the X coordinate of the end of the curve
  ///
  /// y3 : the Y coordinate of the end of the curve
  ///
  /// Since 1.0
  pub fn curve_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
    unsafe {
      cairo_curve_to(self.opaque, x1, y1, x2, y2, x3, y3);
    }
  }

  ///  Adds a line to the path from the current point to position (x, y) in user-space coordinates. After this call the current point will be (x, y).
  ///
  /// If there is no current point before the call to cairo_line_to() this function will behave as cairo_move_to(cr, x, y).
  ///
  /// cr : a cairo context
  /// 
  /// x : the X coordinate of the end of the new line
  ///
  /// y : the Y coordinate of the end of the new line
  ///
  /// Since 1.0
  pub fn line_to(&mut self, x: f64, y: f64) {
    unsafe {
      cairo_line_to(self.opaque, x, y);
    }
  }

  ///  Begin a new sub-path. After this call the current point will be (x, y).
  ///
  /// cr : a cairo context
  ///
  /// x : the X coordinate of the new position
  ///
  /// y : the Y coordinate of the new position
  ///
  /// Since 1.0
  pub fn move_to(&mut self, x: f64, y: f64) {
    unsafe {
      cairo_move_to(self.opaque, x, y);
    }
  }


  ///  Adds a closed sub-path rectangle of the given size to the current path at position (x, y) in user-space coordinates.
  ///
  /// This function is logically equivalent to:
  ///	
  /// ```
  /// cairo_move_to (cr, x, y);
  /// cairo_rel_line_to (cr, width, 0);
  /// cairo_rel_line_to (cr, 0, height);
  /// cairo_rel_line_to (cr, -width, 0);
  /// cairo_close_path (cr);
  ///
  /// cr : a cairo context
  ///
  /// x : the X coordinate of the top left corner of the rectangle
  ///
  /// y : the Y coordinate to the top left corner of the rectangle
  ///
  /// width : the width of the rectangle
  ///
  /// height : the height of the rectangle
  ///
  /// Since 1.0
  pub fn rectangle(&mut self, x: f64, y: f64, width: f64, height: f64) {
    unsafe {
      cairo_rectangle(self.opaque, x, y, width, height);
    }
  }

  ///  Adds closed paths for the glyphs to the current path. The generated path if filled, achieves an effect similar to that of cairo_show_glyphs().
  ///
  /// cr : a cairo context
  ///
  /// glyphs : array of glyphs to show
  ///
  /// num_glyphs : number of glyphs to show
  ///
  /// Since 1.0
  pub fn glyph_path(&mut self, glyphs: &[font::Glyph]) {
    unsafe {
      cairo_glyph_path(self.opaque, glyphs.as_ptr(), glyphs.len() as i32);
    }
  }

  ///  Adds closed paths for text to the current path. The generated path if filled, achieves an effect similar to that of cairo_show_text().
  ///
  /// Text conversion and positioning is done similar to cairo_show_text().
  ///
  /// Like cairo_show_text(), After this call the current point is moved to the origin of where the next glyph would be placed in this same progression. That is, the current point will be at the origin of the final glyph offset by its advance values. This allows for chaining multiple calls to to cairo_text_path() without having to set current point in between.
  ///
  /// Note: The cairo_text_path() function call is part of what the cairo designers call the "toy" text API. It is convenient for short demos and simple programs, but it is not expected to be adequate for serious text-using applications. See cairo_glyph_path() for the "real" text path API in cairo.
  ///
  /// cr : a cairo context
  ///
  /// utf8 : a NUL-terminated string of text encoded in UTF-8, or NULL
  ///
  /// Since 1.0
  pub fn text_path(&mut self, text_path: &str) {
    unsafe {
      use std::c_str::ToCStr;
      cairo_text_path(self.opaque, text_path.to_c_str().unwrap());
    }
  }

  ///  Relative-coordinate version of cairo_curve_to(). All offsets are relative to the current point. Adds a cubic Bézier spline to the path from the current point to a point offset from the current point by (dx3, dy3), using points offset by (dx1, dy1) and (dx2, dy2) as the control points. After this call the current point will be offset by (dx3, dy3).
  ///
  /// Given a current point of (x, y), cairo_rel_curve_to(cr, dx1, dy1, dx2, dy2, dx3, dy3) is logically equivalent to cairo_curve_to(cr, x+dx1, y+dy1, x+dx2, y+dy2, x+dx3, y+dy3).
  ///
  /// It is an error to call this function with no current point. Doing so will cause cr to shutdown with a status of CAIRO_STATUS_NO_CURRENT_POINT.
  ///
  /// cr : a cairo context
  ///
  /// dx1 : the X offset to the first control point
  ///
  /// dy1 : the Y offset to the first control point
  ///
  /// dx2 : the X offset to the second control point
  ///
  /// dy2 : the Y offset to the second control point
  ///
  /// dx3 : the X offset to the end of the curve
  ///
  /// dy3 : the Y offset to the end of the curve
  ///
  /// Since 1.0
  pub fn rel_curve_to(&mut self, dx1: f64, dy1: f64, dx2: f64, dy2: f64, dx3: f64, dy3: f64) {
    unsafe {
      cairo_rel_curve_to(self.opaque, dx1, dy1, dx2, dy2, dx3, dy3);
    }
  }

  ///  Relative-coordinate version of cairo_line_to(). Adds a line to the path from the current point to a point that is offset from the current point by (dx, dy) in user space. After this call the current point will be offset by (dx, dy).
  ///
  /// Given a current point of (x, y), cairo_rel_line_to(cr, dx, dy) is logically equivalent to cairo_line_to(cr, x + dx, y + dy).
  ///
  /// It is an error to call this function with no current point. Doing so will cause cr to shutdown with a status of CAIRO_STATUS_NO_CURRENT_POINT.
  ///
  /// cr : a cairo context
  ///
  /// dx : the X offset to the end of the new line
  ///
  /// dy : the Y offset to the end of the new line
  ///
  /// Since 1.0
  pub fn rel_line_to(&mut self, dx: f64, dy: f64) {
    unsafe {
      cairo_rel_line_to(self.opaque, dx, dy);
    }
  }

  ///  Begin a new sub-path. After this call the current point will offset by (x, y).
  ///
  /// Given a current point of (x, y), cairo_rel_move_to(cr, dx, dy) is logically equivalent to cairo_move_to(cr, x + dx, y + dy).
  ///
  /// It is an error to call this function with no current point. Doing so will cause cr to shutdown with a status of CAIRO_STATUS_NO_CURRENT_POINT.
  ///
  /// cr : a cairo context
  ///
  /// dx : the X offset
  ///
  /// dy : the Y offset
  ///
  /// Since 1.0
  pub fn rel_move_to(&mut self, dx: f64, dy: f64) {
    unsafe {
      cairo_rel_move_to(self.opaque, dx, dy);
    }
  }

  ///  Computes a bounding box in user-space coordinates covering the points on the current path. If the current path is empty, returns an empty rectangle ((0,0), (0,0)). Stroke parameters, fill rule, surface dimensions and clipping are not taken into account.
  ///
  /// Contrast with cairo_fill_extents() and cairo_stroke_extents() which return the extents of only the area that would be "inked" by the corresponding drawing operations.
  ///
  /// The result of cairo_path_extents() is defined as equivalent to the limit of cairo_stroke_extents() with CAIRO_LINE_CAP_ROUND as the line width approaches 0.0, (but never reaching the empty-rectangle returned by cairo_stroke_extents() for a line width of 0.0).
  ///
  /// Specifically, this means that zero-area sub-paths such as cairo_move_to();cairo_line_to() segments, (even degenerate cases where the coordinates to both calls are identical), will be considered as contributing to the extents. However, a lone cairo_move_to() will not contribute to the results of cairo_path_extents().
  ///
  /// cr : a cairo context
  ///
  /// x1 : left of the resulting extents
  ///
  /// y1 : top of the resulting extents
  ///
  /// x2 : right of the resulting extents
  ///
  /// y2 : bottom of the resulting extents
  ///
  /// Since 1.6
  pub fn path_extents(&mut self) -> (f64, f64, f64, f64) {
    unsafe {
      let mut x1:f64 = std::intrinsics::init();
      let mut y1:f64 = std::intrinsics::init();
      let mut x2:f64 = std::intrinsics::init();
      let mut y2:f64 = std::intrinsics::init();
      cairo_path_extents(self.opaque, &mut x1, &mut y1, &mut x2, &mut y2);
      return (x1, y1, x2, y2);
    }
  }

  ///  Modifies the current transformation matrix (CTM) by translating the user-space origin by (tx, ty). This offset is interpreted as a user-space coordinate according to the CTM in place before the new call to cairo_translate(). In other words, the translation of the user-space origin takes place after any existing transformation.
  ///
  /// cr : a cairo context
  ///
  /// tx : amount to translate in the X direction
  ///
  /// ty : amount to translate in the Y direction
  ///
  /// Since 1.0
  pub fn translate(&mut self, tx: f64, ty: f64) {
    unsafe {
      cairo_translate(self.opaque, tx, ty);
    }
  }

  ///  Modifies the current transformation matrix (CTM) by scaling the X and Y user-space axes by sx and sy respectively. The scaling of the axes takes place after any existing transformation of user space.
  ///
  /// cr : a cairo context
  ///
  /// sx : scale factor for the X dimension
  ///
  /// sy : scale factor for the Y dimension
  ///
  /// Since 1.0
  pub fn scale(&mut self, sx: f64, sy: f64) {
    unsafe {
      cairo_scale(self.opaque, sx, sy);
    }
  }

  ///  Modifies the current transformation matrix (CTM) by rotating the user-space axes by angle radians. The rotation of the axes takes places after any existing transformation of user space. The rotation direction for positive angles is from the positive X axis toward the positive Y axis.
  ///
  /// cr : a cairo context
  ///
  /// angle : angle (in radians) by which the user-space axes will be rotated
  ///
  /// Since 1.0
  pub fn rotate(&mut self, angle: f64) {
    unsafe {
      cairo_rotate(self.opaque, angle);
    }
  }


  ///  Modifies the current transformation matrix (CTM) by applying matrix as an additional transformation. The new transformation of user space takes place after any existing transformation.
  ///
  /// cr : a cairo context
  ///
  /// matrix : a transformation to be applied to the user-space axes
  ///
  /// Since 1.0
  pub fn transform(&mut self, matrix: &matrix::Matrix) {
    unsafe {
      cairo_transform(self.opaque, matrix);
    }
  }

  ///  Modifies the current transformation matrix (CTM) by setting it equal to matrix.
  ///
  /// cr : a cairo context
  ///
  /// matrix : a transformation matrix from user space to device space
  ///
  /// Since 1.0
  pub fn set_matrix(&mut self, matrix: &matrix::Matrix) {
    unsafe {
      cairo_set_matrix(self.opaque, matrix);
    }
  }

  ///  Stores the current transformation matrix (CTM) into matrix.
  ///
  /// cr : a cairo context
  ///
  /// matrix : return value for the matrix
  ///
  /// Since 1.0
  pub fn get_matrix(&mut self) -> matrix::Matrix {
    unsafe {
      let mut matrix:matrix::Matrix = std::intrinsics::init();
      cairo_get_matrix(self.opaque, &mut matrix);
      return matrix;
    }
  }

  ///  Resets the current transformation matrix (CTM) by setting it equal to the identity matrix. That is, the user-space and device-space axes will be aligned and one user-space unit will transform to one device-space unit.
  ///
  /// cr : a cairo context
  ///
  /// Since 1.0
  pub fn identity_matrix(&mut self) {
    unsafe {
      cairo_identity_matrix(self.opaque);
    }
  }


  ///  Transform a coordinate from user space to device space by multiplying the given point by the current transformation matrix (CTM).
  ///
  /// cr : a cairo context
  ///
  /// x : X value of coordinate (in/out parameter)
  ///
  /// y : Y value of coordinate (in/out parameter)
  ///
  /// Since 1.0
  pub fn user_to_device(&mut self, x_r: f64, y_r: f64) -> (f64, f64) {
    unsafe {
      let mut x = x_r;
      let mut y = y_r;
      cairo_user_to_device(self.opaque, &mut x, &mut y);
      return (x, y);
    }
  }

  ///  Transform a distance vector from user space to device space. This function is similar to cairo_user_to_device() except that the translation components of the CTM will be ignored when transforming (dx,dy).
  ///
  /// cr : a cairo context
  ///
  /// dx : X component of a distance vector (in/out parameter)
  ///
  /// dy : Y component of a distance vector (in/out parameter)
  ///
  /// Since 1.0
  pub fn user_to_device_distance(&mut self, dx_r: f64, dy_r: f64) -> (f64, f64) {
    unsafe {
      let mut dx = dx_r;
      let mut dy = dy_r;
      cairo_user_to_device_distance(self.opaque, &mut dx, &mut dy);
      return (dx, dy);
    }
  }

  ///  Transform a coordinate from device space to user space by multiplying the given point by the inverse of the current transformation matrix (CTM).
  ///
  /// cr : a cairo
  ///
  /// x : X value of coordinate (in/out parameter)
  ///
  /// y : Y value of coordinate (in/out parameter)
  ///
  /// Since 1.0
  pub fn device_to_user(&mut self, x_r: f64, y_r: f64) -> (f64, f64) {
    unsafe {
      let mut x = x_r;
      let mut y = y_r;
      cairo_device_to_user(self.opaque, &mut x, &mut y);
      return (x, y);
    }
  }


  ///  Transform a distance vector from device space to user space. This function is similar to cairo_device_to_user() except that the translation components of the inverse CTM will be ignored when transforming (dx,dy).
  ///
  /// cr : a cairo context
  ///
  /// dx : X component of a distance vector (in/out parameter)
  ///
  /// dy : Y component of a distance vector (in/out parameter)
  ///
  /// Since 1.0
  pub fn device_to_user_distance(&mut self, dx_r: f64, dy_r: f64) -> (f64, f64) {
    unsafe {
      let mut dx = dx_r;
      let mut dy = dy_r;
      cairo_device_to_user_distance(self.opaque, &mut dx, &mut dy);
      return (dx, dy);
    }
  }

  ///  Replaces the current cairo_font_face_t object in the cairo_t with font_face. The replaced font face in the cairo_t will be destroyed if there are no other references to it.
  ///
  /// cr : a cairo_t
  ///
  /// font_face : a cairo_font_face_t, or NULL to restore to the default font
  ///
  /// Since 1.0
  pub fn select_font_face(&mut self, family: &str, slant: font::slant::Slant, weight: font::weight::Weight) {
    unsafe {
      use std::c_str::ToCStr;
      cairo_select_font_face(self.opaque, family.to_c_str().unwrap(), slant, weight);
    }
  }

  ///  Sets the current font matrix to a scale by a factor of size, replacing any font matrix previously set with cairo_set_font_size() or cairo_set_font_matrix(). This results in a font size of size user space units. (More precisely, this matrix will result in the font's em-square being a size by size square in user space.)
  ///
  /// If text is drawn without a call to cairo_set_font_size(), (nor cairo_set_font_matrix() nor cairo_set_scaled_font()), the default font size is 10.0.
  ///
  /// cr : a cairo_t
  ///
  /// size : the new font size, in user space units
  ///
  /// Since 1.0
  pub fn set_font_size(&mut self, size: f64) {
    unsafe {
      cairo_set_font_size(self.opaque, size);
    }
  }

  ///  Sets the current font matrix to matrix. The font matrix gives a transformation from the design space of the font (in this space, the em-square is 1 unit by 1 unit) to user space. Normally, a simple scale is used (see cairo_set_font_size()), but a more complex font matrix can be used to shear the font or stretch it unequally along the two axes
  ///
  /// cr : a cairo_t
  ///
  /// matrix : a cairo_matrix_t describing a transform to be applied to the current font.
  ///
  /// Since 1.0
  pub fn set_font_matrix(&mut self, size: &matrix::Matrix) {
    unsafe {
      cairo_set_font_matrix(self.opaque, size);
    }
  }

  ///  Stores the current font matrix into matrix. See cairo_set_font_matrix().
  ///
  /// cr : a cairo_t
  ///
  /// matrix : return value for the matrix
  ///
  /// Since 1.0
  pub fn get_font_matrix(&mut self) -> matrix::Matrix {
    unsafe {
      let mut matrix:matrix::Matrix = std::intrinsics::init();
      cairo_get_font_matrix(self.opaque, &mut matrix);
      return matrix;
    }
  }


  ///  Sets a set of custom font rendering options for the cairo_t. Rendering options are derived by merging these options with the options derived from underlying surface; if the value in options has a default value (like CAIRO_ANTIALIAS_DEFAULT), then the value from the surface is used.
  ///
  /// cr : a cairo_t
  ///
  /// options : font options to use
  ///
  /// Since 1.0
  pub fn set_font_options(&mut self, options: font::Options) {
    unsafe {
      cairo_set_font_options(self.opaque, options);
    }
  }

  ///  Retrieves font rendering options set via cairo_set_font_options. Note that the returned options do not include any options derived from the underlying surface; they are literally the options passed to cairo_set_font_options().
  ///
  /// cr : a cairo_t
  ///
  /// options : a cairo_font_options_t object into which to store the retrieved options. All existing values are overwritten
  ///
  /// Since 1.0
  pub fn get_font_options(&mut self, options: font::Options) {
    unsafe {
      cairo_get_font_options(self.opaque, options);
    }
  }

  ///  Replaces the current cairo_font_face_t object in the cairo_t with font_face. The replaced font face in the cairo_t will be destroyed if there are no other references to it.
  /// 
  /// cr : a cairo_t
  ///
  /// font_face : a cairo_font_face_t, or NULL to restore to the default font
  ///
  /// Since 1.0
  pub fn set_font_face(&mut self, font_face: font::FontFace) {
    unsafe {
      cairo_set_font_face(self.opaque, font_face);
    }
  }

  ///  Gets the current font face for a cairo_t.
  ///
  /// cr : a cairo_t
  ///
  /// Returns :
  /// 	the current font face. This object is owned by cairo. To keep a reference to it, you must call cairo_font_face_reference(). This function never returns NULL. If memory cannot be allocated, a special "nil" cairo_font_face_t object will be returned on which cairo_font_face_status() returns CAIRO_STATUS_NO_MEMORY. Using this nil object will cause its error state to propagate to other objects it is passed to, (for example, calling cairo_set_font_face() with a nil font will trigger an error that will shutdown the cairo_t object).
  /// 
  /// Since 1.0
  pub fn get_font_face(&mut self) -> font::FontFace {
    unsafe {
      let foreign_result = cairo_get_font_face(self.opaque);
      return foreign_result;
    }
  }

  ///  Replaces the current font face, font matrix, and font options in the cairo_t with those of the cairo_scaled_font_t. Except for some translation, the current CTM of the cairo_t should be the same as that of the cairo_scaled_font_t, which can be accessed using cairo_scaled_font_get_ctm().
  ///
  /// cr : a cairo_t
  ///
  /// scaled_font : a cairo_scaled_font_t
  ///
  /// Since 1.2
  pub fn set_scaled_font(&mut self, scaled_font: font::ScaledFont) {
    unsafe {
      cairo_set_scaled_font(self.opaque, scaled_font);
    }
  }

  ///  Gets the current scaled font for a cairo_t.
  ///
  /// cr : a cairo_t
  ///
  /// Returns :
  /// 	the current scaled font. This object is owned by cairo. To keep a reference to it, you must call cairo_scaled_font_reference(). This function never returns NULL. If memory cannot be allocated, a special "nil" cairo_scaled_font_t object will be returned on which cairo_scaled_font_status() returns CAIRO_STATUS_NO_MEMORY. Using this nil object will cause its error state to propagate to other objects it is passed to, (for example, calling cairo_set_scaled_font() with a nil font will trigger an error that will shutdown the cairo_t object).
  ///
  /// Since 1.4
  pub fn get_scaled_font(&mut self) -> font::ScaledFont {
    unsafe {
      let foreign_result = cairo_get_scaled_font(self.opaque);
      return foreign_result;
    }
  }

  ///  A drawing operator that generates the shape from a string of UTF-8 characters, rendered according to the current font_face, font_size (font_matrix), and font_options.
  ///
  /// This function first computes a set of glyphs for the string of text. The first glyph is placed so that its origin is at the current point. The origin of each subsequent glyph is offset from that of the previous glyph by the advance values of the previous glyph.
  ///
  /// After this call the current point is moved to the origin of where the next glyph would be placed in this same progression. That is, the current point will be at the origin of the final glyph offset by its advance values. This allows for easy display of a single logical string with multiple calls to cairo_show_text().
  ///
  /// Note: The cairo_show_text() function call is part of what the cairo designers call the "toy" text API. It is convenient for short demos and simple programs, but it is not expected to be adequate for serious text-using applications. See cairo_show_glyphs() for the "real" text display API in cairo.
  ///
  /// cr : a cairo context
  ///
  /// utf8 : a NUL-terminated string of text encoded in UTF-8, or NULL
  /// 
  /// Since 1.0
  pub fn show_text(&mut self, utf8: &str) {
    unsafe {
      cairo_show_text(self.opaque, utf8.to_c_str().unwrap());
    }
  }


  ///  A drawing operator that generates the shape from an array of glyphs, rendered according to the current font face, font size (font matrix), and font options.
  ///
  /// cr : a cairo context
  ///
  /// glyphs : array of glyphs to show
  ///
  /// num_glyphs : number of glyphs to show
  ///
  /// Since 1.0
  pub fn show_glyphs(&mut self, glyphs: &[font::Glyph]) {
    unsafe {
      cairo_show_glyphs(self.opaque, glyphs.as_ptr(), glyphs.len() as i32);
    }
  }

  ///  This operation has rendering effects similar to cairo_show_glyphs() but, if the target surface supports it, uses the provided text and cluster mapping to embed the text for the glyphs shown in the output. If the target does not support the extended attributes, this function acts like the basic cairo_show_glyphs() as if it had been passed glyphs and num_glyphs.
  ///
  /// The mapping between utf8 and glyphs is provided by an array of clusters. Each cluster covers a number of text bytes and glyphs, and neighboring clusters cover neighboring areas of utf8 and glyphs. The clusters should collectively cover utf8 and glyphs in entirety.
  ///
  /// The first cluster always covers bytes from the beginning of utf8. If cluster_flags do not have the CAIRO_TEXT_CLUSTER_FLAG_BACKWARD set, the first cluster also covers the beginning of glyphs, otherwise it covers the end of the glyphs array and following clusters move backward.
  ///
  /// See cairo_text_cluster_t for constraints on valid clusters.
  ///
  /// cr : a cairo context
  ///
  /// utf8 : a string of text encoded in UTF-8
  ///
  /// utf8_len : length of utf8 in bytes, or -1 if it is NUL-terminated
  ///
  /// glyphs : array of glyphs to show
  ///
  /// num_glyphs : number of glyphs to show
  ///
  /// clusters : array of cluster mapping information
  ///
  /// num_clusters : number of clusters in the mapping
  ///
  /// cluster_flags : cluster mapping flags
  ///
  /// Since 1.8
  pub fn show_text_glyphs(&mut self, utf8: &str, glyphs: &[font::Glyph], clusters: &[font::Cluster], cluster_flags: font::cluster_flags::ClusterFlags) {
    unsafe {
      cairo_show_text_glyphs(self.opaque, utf8.to_c_str().unwrap(), -1, glyphs.as_ptr(), glyphs.len() as i32, clusters.as_ptr(), clusters.len() as i32, cluster_flags);
    }
  }

  ///  Gets the font extents for the currently selected font.
  ///
  /// cr : a cairo_t
  ///
  /// extents : a cairo_font_extents_t object into which the results will be stored.
  ///
  /// Since 1.0
  pub fn font_extents(&mut self) -> font::FontExtents {
    unsafe {
      let mut extents:font::FontExtents = std::intrinsics::init();
      cairo_font_extents(self.opaque, &mut extents);
      return extents;
    }
  }

  ///  Gets the extents for a string of text. The extents describe a user-space rectangle that encloses the "inked" portion of the text, (as it would be drawn by cairo_show_text()). Additionally, the x_advance and y_advance values indicate the amount by which the current point would be advanced by cairo_show_text().
  ///
  /// Note that whitespace characters do not directly contribute to the size of the rectangle (extents.width and extents.height). They do contribute indirectly by changing the position of non-whitespace characters. In particular, trailing whitespace characters are likely to not affect the size of the rectangle, though they will affect the x_advance and y_advance values.
  ///
  /// cr : a cairo_t
  ///
  /// utf8 : a NUL-terminated string of text encoded in UTF-8, or NULL
  ///
  /// extents : a cairo_text_extents_t object into which the results will be stored
  ///
  /// Since 1.0
  pub fn text_extents(&mut self, utf8: &str) -> font::TextExtents {
    unsafe {
      let mut extents:font::TextExtents = std::intrinsics::init();
      cairo_text_extents(self.opaque, utf8.to_c_str().unwrap(), &mut extents);
      return extents;
    }
  }

  ///  Gets the extents for an array of glyphs. The extents describe a user-space rectangle that encloses the "inked" portion of the glyphs, (as they would be drawn by cairo_show_glyphs()). Additionally, the x_advance and y_advance values indicate the amount by which the current point would be advanced by cairo_show_glyphs().
  ///
  /// Note that whitespace glyphs do not contribute to the size of the rectangle (extents.width and extents.height).
  ///
  /// cr : a cairo_t
  ///
  /// glyphs : an array of cairo_glyph_t objects
  ///
  /// num_glyphs : the number of elements in glyphs
  ///
  /// extents : a cairo_text_extents_t object into which the results will be stored
  /// 
  /// Since 1.0
  pub fn glyph_extents(&mut self, glyphs: &[font::Glyph]) -> font::TextExtents {
    unsafe {
      let mut extents:font::TextExtents = std::intrinsics::init();
      cairo_glyph_extents(self.opaque, glyphs.as_ptr(), glyphs.len() as i32, &mut extents);
      return extents;
    }
  }
}

extern {
  fn cairo_create(surface: *mut std::libc::c_void) -> Cairo;
  fn cairo_status(self_value: *mut std::libc::c_void) -> Status;
  fn cairo_save(self_value: *mut std::libc::c_void);
  fn cairo_restore(self_value: *mut std::libc::c_void);
  fn cairo_get_target(self_value: *mut std::libc::c_void) -> surface::Surface;
  fn cairo_push_group(self_value: *mut std::libc::c_void);
  fn cairo_push_group_with_content(self_value: *mut std::libc::c_void, content: surface::content::Content);
  fn cairo_pop_group(self_value: *mut std::libc::c_void) -> pattern::Pattern;
  fn cairo_pop_group_to_source(self_value: *mut std::libc::c_void);
  fn cairo_get_group_target(self_value: *mut std::libc::c_void) -> surface::Surface;
  fn cairo_set_source_rgb(self_value: *mut std::libc::c_void, red: f64, green: f64, blue: f64);
  fn cairo_set_source_rgba(self_value: *mut std::libc::c_void, red: f64, green: f64, blue: f64, alpha: f64);
  fn cairo_set_source(self_value: *mut std::libc::c_void, source: *mut std::libc::c_void);
  fn cairo_set_source_surface(self_value: *mut std::libc::c_void, surface: *mut std::libc::c_void, x: f64, y: f64);
  fn cairo_get_source(self_value: *mut std::libc::c_void) -> pattern::Pattern;
  fn cairo_set_antialias(self_value: *mut std::libc::c_void, antialias: antialias::Antialias);
  fn cairo_get_antialias(self_value: *mut std::libc::c_void) -> antialias::Antialias;
  fn cairo_set_dash(self_value: *mut std::libc::c_void, dashes: *f64, dashes_length: i32, offset: f64);
  fn cairo_get_dash_count(self_value: *mut std::libc::c_void) -> i32;
  fn cairo_get_dash(self_value: *mut std::libc::c_void, dashes: *mut f64, offset: *mut f64);
  fn cairo_set_fill_rule(self_value: *mut std::libc::c_void, fill_rule: fill_rule::FillRule);
  fn cairo_get_fill_rule(self_value: *mut std::libc::c_void) -> fill_rule::FillRule;
  fn cairo_set_line_cap(self_value: *mut std::libc::c_void, line_cap: line_cap::LineCap);
  fn cairo_get_line_cap(self_value: *mut std::libc::c_void) -> line_cap::LineCap;
  fn cairo_set_line_join(self_value: *mut std::libc::c_void, line_join: line_join::LineJoin);
  fn cairo_get_line_join(self_value: *mut std::libc::c_void) -> line_join::LineJoin;
  fn cairo_set_line_width(self_value: *mut std::libc::c_void, width: f64);
  fn cairo_get_line_width(self_value: *mut std::libc::c_void) -> f64;
  fn cairo_set_miter_limit(self_value: *mut std::libc::c_void, limit: f64);
  fn cairo_get_miter_limit(self_value: *mut std::libc::c_void) -> f64;
  fn cairo_set_operator(self_value: *mut std::libc::c_void, operator: operator::Operator);
  fn cairo_get_operator(self_value: *mut std::libc::c_void) -> operator::Operator;
  fn cairo_set_tolerance(self_value: *mut std::libc::c_void, tolerance: f64);
  fn cairo_get_tolerance(self_value: *mut std::libc::c_void) -> f64;
  fn cairo_clip(self_value: *mut std::libc::c_void);
  fn cairo_clip_preserve(self_value: *mut std::libc::c_void);
  fn cairo_clip_extents(self_value: *mut std::libc::c_void, x1: *mut f64, y1: *mut f64, x2: *mut f64, y2: *mut f64);
  fn cairo_in_clip(self_value: *mut std::libc::c_void, x: f64, y: f64) -> i32;
  fn cairo_reset_clip(self_value: *mut std::libc::c_void);
  fn cairo_fill(self_value: *mut std::libc::c_void);
  fn cairo_fill_preserve(self_value: *mut std::libc::c_void);
  fn cairo_fill_extents(self_value: *mut std::libc::c_void, x1: *mut f64, y1: *mut f64, x2: *mut f64, y2: *mut f64);
  fn cairo_in_fill(self_value: *mut std::libc::c_void, x: f64, y: f64) -> i32;
  fn cairo_mask(self_value: *mut std::libc::c_void, pattern: *mut std::libc::c_void);
  fn cairo_mask_surface(self_value: *mut std::libc::c_void, surface: *mut std::libc::c_void, surface_x: f64, surface_y: f64);
  fn cairo_paint(self_value: *mut std::libc::c_void);
  fn cairo_paint_with_alpha(self_value: *mut std::libc::c_void, alpha: f64);
  fn cairo_stroke(self_value: *mut std::libc::c_void);
  fn cairo_stroke_preserve(self_value: *mut std::libc::c_void);
  fn cairo_stroke_extents(self_value: *mut std::libc::c_void, x1: *mut f64, y1: *mut f64, x2: *mut f64, y2: *mut f64);
  fn cairo_in_stroke(self_value: *mut std::libc::c_void, x: f64, y: f64) -> i32;
  fn cairo_copy_page(self_value: *mut std::libc::c_void);
  fn cairo_show_page(self_value: *mut std::libc::c_void);
  fn cairo_get_reference_count(self_value: *mut std::libc::c_void) -> i32;
  fn cairo_copy_path(self_value: *mut std::libc::c_void) -> path::Path;
  fn cairo_copy_path_flat(self_value: *mut std::libc::c_void) -> path::Path;
  fn cairo_append_path(self_value: *mut std::libc::c_void, path: *std::libc::c_void);
  fn cairo_has_current_point(self_value: *mut std::libc::c_void) -> i32;
  fn cairo_get_current_point(self_value: *mut std::libc::c_void, x: *mut f64, y: *mut f64);
  fn cairo_new_path(self_value: *mut std::libc::c_void);
  fn cairo_new_sub_path(self_value: *mut std::libc::c_void);
  fn cairo_close_path(self_value: *mut std::libc::c_void);
  fn cairo_arc(self_value: *mut std::libc::c_void, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64);
  fn cairo_arc_negative(self_value: *mut std::libc::c_void, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64);
  fn cairo_curve_to(self_value: *mut std::libc::c_void, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64);
  fn cairo_line_to(self_value: *mut std::libc::c_void, x: f64, y: f64);
  fn cairo_move_to(self_value: *mut std::libc::c_void, x: f64, y: f64);
  fn cairo_rectangle(self_value: *mut std::libc::c_void, x: f64, y: f64, width: f64, height: f64);
  fn cairo_glyph_path(self_value: *mut std::libc::c_void, glyphs: *font::Glyph, glyphs_length: i32);
  fn cairo_text_path(self_value: *mut std::libc::c_void, text_path: *std::libc::c_char);
  fn cairo_rel_curve_to(self_value: *mut std::libc::c_void, dx1: f64, dy1: f64, dx2: f64, dy2: f64, dx3: f64, dy3: f64);
  fn cairo_rel_line_to(self_value: *mut std::libc::c_void, dx: f64, dy: f64);
  fn cairo_rel_move_to(self_value: *mut std::libc::c_void, dx: f64, dy: f64);
  fn cairo_path_extents(self_value: *mut std::libc::c_void, x1: *mut f64, y1: *mut f64, x2: *mut f64, y2: *mut f64);
  fn cairo_translate(self_value: *mut std::libc::c_void, tx: f64, ty: f64);
  fn cairo_scale(self_value: *mut std::libc::c_void, sx: f64, sy: f64);
  fn cairo_rotate(self_value: *mut std::libc::c_void, angle: f64);
  fn cairo_transform(self_value: *mut std::libc::c_void, matrix: *matrix::Matrix);
  fn cairo_set_matrix(self_value: *mut std::libc::c_void, matrix: *matrix::Matrix);
  fn cairo_get_matrix(self_value: *mut std::libc::c_void, matrix: *mut matrix::Matrix);
  fn cairo_identity_matrix(self_value: *mut std::libc::c_void);
  fn cairo_user_to_device(self_value: *mut std::libc::c_void, x: &mut f64, y: &mut f64);
  fn cairo_user_to_device_distance(self_value: *mut std::libc::c_void, dx: &mut f64, dy: &mut f64);
  fn cairo_device_to_user(self_value: *mut std::libc::c_void, x: &mut f64, y: &mut f64);
  fn cairo_device_to_user_distance(self_value: *mut std::libc::c_void, dx: &mut f64, dy: &mut f64);
  fn cairo_select_font_face(self_value: *mut std::libc::c_void, family: *std::libc::c_char, slant: font::slant::Slant, weight: font::weight::Weight);
  fn cairo_set_font_size(self_value: *mut std::libc::c_void, size: f64);
  fn cairo_set_font_matrix(self_value: *mut std::libc::c_void, size: *matrix::Matrix);
  fn cairo_get_font_matrix(self_value: *mut std::libc::c_void, matrix: *mut matrix::Matrix);
  fn cairo_set_font_options(self_value: *mut std::libc::c_void, options: font::Options);
  fn cairo_get_font_options(self_value: *mut std::libc::c_void, options: font::Options);
  fn cairo_set_font_face(self_value: *mut std::libc::c_void, font_face: font::FontFace);
  fn cairo_get_font_face(self_value: *mut std::libc::c_void) -> font::FontFace;
  fn cairo_set_scaled_font(self_value: *mut std::libc::c_void, scaled_font: font::ScaledFont);
  fn cairo_get_scaled_font(self_value: *mut std::libc::c_void) -> font::ScaledFont;
  fn cairo_show_text(self_value: *mut std::libc::c_void, utf8: *std::libc::c_char);
  fn cairo_show_glyphs(self_value: *mut std::libc::c_void, glyphs: *font::Glyph, glyphs_length: i32);
  fn cairo_show_text_glyphs(self_value: *mut std::libc::c_void, utf8: *std::libc::c_char, utf8_len: i32, glyphs: *font::Glyph, glyphs_length: i32, clusters: *font::Cluster, clusters_length: i32, cluster_flags: font::cluster_flags::ClusterFlags);
  fn cairo_font_extents(self_value: *mut std::libc::c_void, extents: *mut font::FontExtents);
  fn cairo_text_extents(self_value: *mut std::libc::c_void, utf8: *std::libc::c_char, extents: *mut font::TextExtents);
  fn cairo_glyph_extents(self_value: *mut std::libc::c_void, glyphs: *font::Glyph, glyphs_length: i32, extents: *mut font::TextExtents);
}

impl std::clone::Clone for Cairo {
  fn clone(&self) -> Cairo {
    unsafe {
      let foreign_result = cairo_reference(self.opaque as *std::libc::c_void);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_reference(self_value: *std::libc::c_void) -> Cairo;
}

impl std::ops::Drop for Cairo {
  fn drop(&mut self) {
    unsafe {
      cairo_destroy(self.opaque);
    }
  }
}

extern {
  fn cairo_destroy(self_value: *mut std::libc::c_void);
}

pub mod antialias;
pub mod fill_rule;
pub mod line_cap;
pub mod line_join;
pub mod operator;
pub mod path;
pub mod pattern;
pub mod region;
pub mod font;
pub mod surface;
pub mod matrix;

