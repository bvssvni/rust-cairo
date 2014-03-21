//! Sources for drawing

use std;

/// A pattern::Pattern represents a source when drawing onto a surface. There are different subtypes of pattern::Pattern, for different types of sources; for example, cairo_pattern_create_rgb() creates a pattern for a solid opaque color.
///
/// Other than various cairo_pattern_create_type() functions, some of the pattern types can be implicitly created using various cairo_set_source_type() functions; for example cairo_set_source_rgb().
/// 
/// The type of a pattern can be queried with cairo_pattern_get_type().
/// 
/// Memory management of pattern::Pattern is done with cairo_pattern_reference() and cairo_pattern_destroy().
/// 
/// Since 1.0
pub struct Pattern {
  /// Wraps Cairo pointer of pattern.
  opaque: *mut std::libc::c_void
}

impl Pattern {
  /// Adds an opaque color stop to a gradient pattern. The offset specifies the location along the gradient's control vector. For example, a linear gradient's control vector is from (x0,y0) to (x1,y1) while a radial gradient's control vector is from any point on the start circle to the corresponding point on the end circle.
  /// 
  /// The color is specified in the same way as in cairo_set_source_rgb().
  /// 
  /// If two (or more) stops are specified with identical offset values, they will be sorted according to the order in which the stops are added, (stops added earlier will compare less than stops added later). This can be useful for reliably making sharp color transitions instead of the typical blend.
  /// 
  /// Note: If the pattern is not a gradient pattern, (eg. a linear or radial pattern), then the pattern will be put into an error status with a status of CAIRO_STATUS_PATTERN_TYPE_MISMATCH.
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// offset : an offset in the range [0.0 .. 1.0]
  /// 
  /// red : red component of color
  /// 
  /// green : green component of color
  /// 
  /// blue : blue component of color
  /// 
  /// Since 1.0
  pub fn add_color_stop_rgb(&mut self, offset: f64, red: f64, green: f64, blue: f64) {
    unsafe {
      cairo_pattern_add_color_stop_rgb(self.opaque, offset, red, green, blue);
    }
  }

  /// Adds a translucent color stop to a gradient pattern. The offset specifies the location along the gradient's control vector. For example, a linear gradient's control vector is from (x0,y0) to (x1,y1) while a radial gradient's control vector is from any point on the start circle to the corresponding point on the end circle.
  ///
  /// The color is specified in the same way as in cairo_set_source_rgba().
  ///
  /// If two (or more) stops are specified with identical offset values, they will be sorted according to the order in which the stops are added, (stops added earlier will compare less than stops added later). This can be useful for reliably making sharp color transitions instead of the typical blend.
  ///
  /// Note: If the pattern is not a gradient pattern, (eg. a linear or radial pattern), then the pattern will be put into an error status with a status of CAIRO_STATUS_PATTERN_TYPE_MISMATCH.
  ///
  /// pattern : a pattern::Pattern
  ///
  /// offset : an offset in the range [0.0 .. 1.0]
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
  pub fn add_color_stop_rgba(&mut self, offset: f64, red: f64, green: f64, blue: f64, alpha: f64) {
    unsafe {
      cairo_pattern_add_color_stop_rgba(self.opaque, offset, red, green, blue, alpha);
    }
  }

  /// Gets the number of color stops specified in the given gradient pattern.
  ///
  /// pattern : a pattern::Pattern
  ///
  /// count : return value for the number of color stops, or NULL
  ///
  /// Returns : CAIRO_STATUS_SUCCESS, or CAIRO_STATUS_PATTERN_TYPE_MISMATCH if pattern is not a gradient pattern.
  ///
  /// Since 1.4
  pub fn get_color_stop_count(&mut self) -> (super::Status, i32) {
    unsafe {
      let mut stop_count:i32 = std::intrinsics::init();
      let foreign_result = cairo_pattern_get_color_stop_count(self.opaque, &mut stop_count);
      return (foreign_result, stop_count);
    }
  }

  /// Gets the color and offset information at the given index for a gradient pattern. Values of index are 0 to 1 less than the number returned by cairo_pattern_get_color_stop_count().
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// index : index of the stop to return data for
  /// 
  /// offset : return value for the offset of the stop, or NULL
  /// 
  /// red : return value for red component of color, or NULL
  /// 
  /// green : return value for green component of color, or NULL
  ///
  /// blue : return value for blue component of color, or NULL
  ///
  /// alpha : return value for alpha component of color, or NULL
  ///
  /// Returns : CAIRO_STATUS_SUCCESS, or CAIRO_STATUS_INVALID_INDEX if index is not valid for the given pattern. If the pattern is not a gradient pattern, CAIRO_STATUS_PATTERN_TYPE_MISMATCH is returned.
  /// 
  /// Since 1.4
  pub fn get_color_stop_rgba(&mut self, stop_count: i32) -> (super::Status, f64, f64, f64, f64, f64) {
    unsafe {
      let mut offset:f64 = std::intrinsics::init();
      let mut red:f64 = std::intrinsics::init();
      let mut green:f64 = std::intrinsics::init();
      let mut blue:f64 = std::intrinsics::init();
      let mut alpha:f64 = std::intrinsics::init();
      let foreign_result = cairo_pattern_get_color_stop_rgba(self.opaque, stop_count, &mut offset, &mut red, &mut green, &mut blue, &mut alpha);
      return (foreign_result, offset, red, green, blue, alpha);
    }
  }

  /// Creates a new pattern::Pattern corresponding to an opaque color. The color components are floating point numbers in the range 0 to 1. If the values passed in are outside that range, they will be clamped.
  /// 
  /// red : red component of the color
  /// 
  /// green : green component of the color
  ///
  /// blue : blue component of the color
  ///
  /// Returns :
  /// 	the newly created pattern::Pattern if successful, or an error pattern in case of no memory. The caller owns the returned object and should call cairo_pattern_destroy() when finished with it. This function will always return a valid pointer, but if an error occurred the pattern status will be set to an error. To inspect the status of a pattern use cairo_pattern_status().
  /// 
  /// Since 1.0
  pub fn rgb(red: f64, green: f64, blue: f64) -> Pattern {
    unsafe {
      let foreign_result = cairo_pattern_create_rgb(red, green, blue);
      return foreign_result;
    }
  }

  /// Creates a new pattern::Pattern corresponding to a translucent color. The color components are floating point numbers in the range 0 to 1. If the values passed in are outside that range, they will be clamped.
  /// 
  /// red : red component of the color
  /// 
  /// green : green component of the color
  /// 
  /// blue : blue component of the color
  /// 
  /// alpha : alpha component of the color
  /// 
  /// Returns : the newly created pattern::Pattern if successful, or an error pattern in case of no memory. The caller owns the returned object and should call cairo_pattern_destroy() when finished with it. This function will always return a valid pointer, but if an error occurred the pattern status will be set to an error. To inspect the status of a pattern use cairo_pattern_status().
  /// 
  /// Since 1.0
  pub fn rgba(red: f64, green: f64, blue: f64, alpha: f64) -> Pattern {
    unsafe {
      let foreign_result = cairo_pattern_create_rgba(red, green, blue, alpha);
      return foreign_result;
    }
  }

  /// Gets the solid color for a solid color pattern.
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// red : return value for red component of color, or NULL
  /// 
  /// green : return value for green component of color, or NULL
  /// 
  /// blue : return value for blue component of color, or NULL
  /// 
  /// alpha : return value for alpha component of color, or NULL
  /// 
  /// Returns : CAIRO_STATUS_SUCCESS, or CAIRO_STATUS_PATTERN_TYPE_MISMATCH if the pattern is not a solid color pattern.
  ///
  /// Since 1.4
  pub fn get_rgba(&mut self) -> (super::Status, f64, f64, f64, f64) {
    unsafe {
      let mut red:f64 = std::intrinsics::init();
      let mut green:f64 = std::intrinsics::init();
      let mut blue:f64 = std::intrinsics::init();
      let mut alpha:f64 = std::intrinsics::init();
      let foreign_result = cairo_pattern_get_rgba(self.opaque, &mut red, &mut green, &mut blue, &mut alpha);
      return (foreign_result, red, green, blue, alpha);
    }
  }

  /// Gets the solid color for a solid color pattern.
  ///
  /// pattern : a pattern::Pattern
  ///
  /// red : return value for red component of color, or NULL
  ///
  /// green : return value for green component of color, or NULL
  /// 
  /// blue : return value for blue component of color, or NULL
  /// 
  /// alpha : return value for alpha component of color, or NULL
  /// 
  /// Returns : CAIRO_STATUS_SUCCESS, or CAIRO_STATUS_PATTERN_TYPE_MISMATCH if the pattern is not a solid color pattern.
  ///
  /// Since 1.4
  pub fn for_surface(surface: super::surface::Surface) -> Pattern {
    unsafe {
      let foreign_result = cairo_pattern_create_for_surface(surface);
      return foreign_result;
    }
  }

  /// Gets the surface of a surface pattern. The reference returned in surface is owned by the pattern; the caller should call cairo_surface_reference() if the surface is to be retained.
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// surface : return value for surface of pattern, or NULL
  /// 
  /// Returns : CAIRO_STATUS_SUCCESS, or CAIRO_STATUS_PATTERN_TYPE_MISMATCH if the pattern is not a surface pattern.
  ///
  /// Since 1.4
  pub fn get_surface(&mut self) -> (super::Status, super::surface::Surface) {
    unsafe {
      let mut surface:super::surface::Surface = std::intrinsics::init();
      let foreign_result = cairo_pattern_get_surface(self.opaque, &mut surface);
      return (foreign_result, surface);
    }
  }

  /// Create a new linear gradient pattern::Pattern along the line defined by (x0, y0) and (x1, y1). Before using the gradient pattern, a number of color stops should be defined using cairo_pattern_add_color_stop_rgb() or cairo_pattern_add_color_stop_rgba().
  /// 
  /// Note: The coordinates here are in pattern space. For a new pattern, pattern space is identical to user space, but the relationship between the spaces can be changed with cairo_pattern_set_matrix().
  /// 
  /// x0 : x coordinate of the start point
  /// 
  /// y0 : y coordinate of the start point
  /// 
  /// x1 : x coordinate of the end point
  /// 
  /// y1 : y coordinate of the end point
  /// 
  /// Returns : the newly created pattern::Pattern if successful, or an error pattern in case of no memory. The caller owns the returned object and should call cairo_pattern_destroy() when finished with it. This function will always return a valid pointer, but if an error occurred the pattern status will be set to an error. To inspect the status of a pattern use cairo_pattern_status().
  /// 
  /// Since 1.0
  pub fn linear(x0: f64, y0: f64, x1: f64, y1: f64) -> Pattern {
    unsafe {
      let foreign_result = cairo_pattern_create_linear(x0, y0, x1, y1);
      return foreign_result;
    }
  }

  /// Gets the gradient endpoints for a linear gradient.
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// x0 : return value for the x coordinate of the first point, or NULL
  /// 
  /// y0 : return value for the y coordinate of the first point, or NULL
  /// 
  /// x1 : return value for the x coordinate of the second point, or NULL
  /// 
  /// y1 : return value for the y coordinate of the second point, or NULL
  /// 
  /// Returns : CAIRO_STATUS_SUCCESS, or CAIRO_STATUS_PATTERN_TYPE_MISMATCH if pattern is not a linear gradient pattern.
  /// 
  /// Since 1.4
  pub fn get_linear_points(&mut self) -> (super::Status, f64, f64, f64, f64) {
    unsafe {
      let mut x0:f64 = std::intrinsics::init();
      let mut y0:f64 = std::intrinsics::init();
      let mut x1:f64 = std::intrinsics::init();
      let mut y1:f64 = std::intrinsics::init();
      let foreign_result = cairo_pattern_get_linear_points(self.opaque, &mut x0, &mut y0, &mut x1, &mut y1);
      return (foreign_result, x0, y0, x1, y1);
    }
  }

  /// Creates a new radial gradient pattern::Pattern between the two circles defined by (cx0, cy0, radius0) and (cx1, cy1, radius1). Before using the gradient pattern, a number of color stops should be defined using cairo_pattern_add_color_stop_rgb() or cairo_pattern_add_color_stop_rgba().
  /// 
  /// Note: The coordinates here are in pattern space. For a new pattern, pattern space is identical to user space, but the relationship between the spaces can be changed with cairo_pattern_set_matrix().
  /// 
  /// cx0 : x coordinate for the center of the start circle
  /// 
  /// cy0 : y coordinate for the center of the start circle
  /// 
  /// radius0 : radius of the start circle
  /// 
  /// cx1 : x coordinate for the center of the end circle
  /// 
  /// cy1 : y coordinate for the center of the end circle
  /// 
  /// radius1 : radius of the end circle
  ///
  /// Returns : the newly created pattern::Pattern if successful, or an error pattern in case of no memory. The caller owns the returned object and should call cairo_pattern_destroy() when finished with it. This function will always return a valid pointer, but if an error occurred the pattern status will be set to an error. To inspect the status of a pattern use cairo_pattern_status().
  ///
  /// Since 1.0
  pub fn radial(cx0: f64, cy0: f64, radius0: f64, cx1: f64, cy1: f64, radius1: f64) -> Pattern {
    unsafe {
      let foreign_result = cairo_pattern_create_radial(cx0, cy0, radius0, cx1, cy1, radius1);
      return foreign_result;
    }
  }

  /// Gets the gradient endpoint circles for a radial gradient, each specified as a center coordinate and a radius.
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// x0 : return value for the x coordinate of the center of the first circle, or NULL
  /// 
  /// y0 : return value for the y coordinate of the center of the first circle, or NULL
  /// 
  /// r0 : return value for the radius of the first circle, or NULL
  /// 
  /// x1 : return value for the x coordinate of the center of the second circle, or NULL
  /// 
  /// y1 : return value for the y coordinate of the center of the second circle, or NULL
  /// 
  /// r1 : return value for the radius of the second circle, or NULL
  ///
  /// Returns : CAIRO_STATUS_SUCCESS, or CAIRO_STATUS_PATTERN_TYPE_MISMATCH if pattern is not a radial gradient pattern.
  /// 
  /// Since 1.4
  pub fn get_radial_circles(&mut self) -> (super::Status, f64, f64, f64, f64, f64, f64) {
    unsafe {
      let mut x0:f64 = std::intrinsics::init();
      let mut y0:f64 = std::intrinsics::init();
      let mut r0:f64 = std::intrinsics::init();
      let mut x1:f64 = std::intrinsics::init();
      let mut y1:f64 = std::intrinsics::init();
      let mut r1:f64 = std::intrinsics::init();
      let foreign_result = cairo_pattern_get_radial_circles(self.opaque, &mut x0, &mut y0, &mut r0, &mut x1, &mut y1, &mut r1);
      return (foreign_result, x0, y0, r0, x1, y1, r1);
    }
  }

  /// Create a new mesh pattern.
  /// 
  /// Mesh patterns are tensor-product patch meshes (type 7 shadings in PDF). Mesh patterns may also be used to create other types of shadings that are special cases of tensor-product patch meshes such as Coons patch meshes (type 6 shading in PDF) and Gouraud-shaded triangle meshes (type 4 and 5 shadings in PDF).
  /// 
  /// Mesh patterns consist of one or more tensor-product patches, which should be defined before using the mesh pattern. Using a mesh pattern with a partially defined patch as source or mask will put the context in an error status with a status of CAIRO_STATUS_INVALID_MESH_CONSTRUCTION.
  /// 
  /// A tensor-product patch is defined by 4 Bézier curves (side 0, 1, 2, 3) and by 4 additional control points (P0, P1, P2, P3) that provide further control over the patch and complete the definition of the tensor-product patch. The corner C0 is the first point of the patch.
  /// 
  /// Degenerate sides are permitted so straight lines may be used. A zero length line on one side may be used to create 3 sided patches.
  ///
  /// ``` 
  ///       C1     Side 1       C2
  ///        +---------------+
  ///        |               |
  ///        |  P1       P2  |
  ///        |               |
  /// Side 0 |               | Side 2
  ///        |               |
  ///        |               |
  ///        |  P0       P3  |
  ///        |               |
  ///        +---------------+
  ///      C0     Side 3        C3
  /// ```
  ///  
  /// Each patch is constructed by first calling cairo_mesh_pattern_begin_patch(), then cairo_mesh_pattern_move_to() to specify the first point in the patch (C0). Then the sides are specified with calls to cairo_mesh_pattern_curve_to() and cairo_mesh_pattern_line_to().
  /// 
  /// The four additional control points (P0, P1, P2, P3) in a patch can be specified with cairo_mesh_pattern_set_control_point().
  /// 
  /// At each corner of the patch (C0, C1, C2, C3) a color may be specified with cairo_mesh_pattern_set_corner_color_rgb() or cairo_mesh_pattern_set_corner_color_rgba(). Any corner whose color is not explicitly specified defaults to transparent black.
  /// 
  /// A Coons patch is a special case of the tensor-product patch where the control points are implicitly defined by the sides of the patch. The default value for any control point not specified is the implicit value for a Coons patch, i.e. if no control points are specified the patch is a Coons patch.
  /// 
  /// A triangle is a special case of the tensor-product patch where the control points are implicitly defined by the sides of the patch, all the sides are lines and one of them has length 0, i.e. if the patch is specified using just 3 lines, it is a triangle. If the corners connected by the 0-length side have the same color, the patch is a Gouraud-shaded triangle.
  ///
  /// Patches may be oriented differently to the above diagram. For example the first point could be at the top left. The diagram only shows the relationship between the sides, corners and control points. Regardless of where the first point is located, when specifying colors, corner 0 will always be the first point, corner 1 the point between side 0 and side 1 etc.
  /// 
  /// Calling cairo_mesh_pattern_end_patch() completes the current patch. If less than 4 sides have been defined, the first missing side is defined as a line from the current point to the first point of the patch (C0) and the other sides are degenerate lines from C0 to C0. The corners between the added sides will all be coincident with C0 of the patch and their color will be set to be the same as the color of C0.
  /// 
  /// Additional patches may be added with additional calls to cairo_mesh_pattern_begin_patch()/cairo_mesh_pattern_end_patch().
  /// 	
  /// ```
  /// pattern::Pattern *pattern = cairo_pattern_create_mesh ();
  /// 
  /// /* Add a Coons patch */
  /// cairo_mesh_pattern_begin_patch (pattern);
  /// cairo_mesh_pattern_move_to (pattern, 0, 0);
  /// cairo_mesh_pattern_curve_to (pattern, 30, -30,  60,  30, 100, 0);
  /// cairo_mesh_pattern_curve_to (pattern, 60,  30, 130,  60, 100, 100);
  /// cairo_mesh_pattern_curve_to (pattern, 60,  70,  30, 130,   0, 100);
  /// cairo_mesh_pattern_curve_to (pattern, 30,  70, -30,  30,   0, 0);
  /// cairo_mesh_pattern_set_corner_color_rgb (pattern, 0, 1, 0, 0);
  /// cairo_mesh_pattern_set_corner_color_rgb (pattern, 1, 0, 1, 0);
  /// cairo_mesh_pattern_set_corner_color_rgb (pattern, 2, 0, 0, 1);
  /// cairo_mesh_pattern_set_corner_color_rgb (pattern, 3, 1, 1, 0);
  /// cairo_mesh_pattern_end_patch (pattern);
  /// 
  /// /* Add a Gouraud-shaded triangle */
  /// cairo_mesh_pattern_begin_patch (pattern)
  /// cairo_mesh_pattern_move_to (pattern, 100, 100);
  /// cairo_mesh_pattern_line_to (pattern, 130, 130);
  /// cairo_mesh_pattern_line_to (pattern, 130,  70);
  /// cairo_mesh_pattern_set_corner_color_rgb (pattern, 0, 1, 0, 0);
  /// cairo_mesh_pattern_set_corner_color_rgb (pattern, 1, 0, 1, 0);
  /// cairo_mesh_pattern_set_corner_color_rgb (pattern, 2, 0, 0, 1);
  /// cairo_mesh_pattern_end_patch (pattern)
  /// ```
  ///  
  /// When two patches overlap, the last one that has been added is drawn over the first one.
  /// 
  /// When a patch folds over itself, points are sorted depending on their parameter coordinates inside the patch. The v coordinate ranges from 0 to 1 when moving from side 3 to side 1; the u coordinate ranges from 0 to 1 when going from side 0 to side 2. Points with higher v coordinate hide points with lower v coordinate. When two points have the same v coordinate, the one with higher u coordinate is above. This means that points nearer to side 1 are above points nearer to side 3; when this is not sufficient to decide which point is above (for example when both points belong to side 1 or side 3) points nearer to side 2 are above points nearer to side 0.
  /// 
  /// For a complete definition of tensor-product patches, see the PDF specification (ISO32000), which describes the parametrization in detail.
  /// 
  /// Note: The coordinates are always in pattern space. For a new pattern, pattern space is identical to user space, but the relationship between the spaces can be changed with cairo_pattern_set_matrix().
  /// 
  /// Returns : the newly created pattern::Pattern if successful, or an error pattern in case of no memory. The caller owns the returned object and should call cairo_pattern_destroy() when finished with it. This function will always return a valid pointer, but if an error occurred the pattern status will be set to an error. To inspect the status of a pattern use cairo_pattern_status().
  /// 
  /// Since 1.12
  pub fn mesh() -> Pattern {
    unsafe {
      let foreign_result = cairo_pattern_create_mesh();
      return foreign_result;
    }
  }

  /// Begin a patch in a mesh pattern.
  /// 
  /// After calling this function, the patch shape should be defined with cairo_mesh_pattern_move_to(), cairo_mesh_pattern_line_to() and cairo_mesh_pattern_curve_to().
  /// 
  /// After defining the patch, cairo_mesh_pattern_end_patch() must be called before using pattern as a source or mask.
  /// 
  /// Note: If pattern is not a mesh pattern then pattern will be put into an error status with a status of CAIRO_STATUS_PATTERN_TYPE_MISMATCH. If pattern already has a current patch, it will be put into an error status with a status of CAIRO_STATUS_INVALID_MESH_CONSTRUCTION.
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// Since 1.12
  pub fn begin_patch(&mut self) {
    unsafe {
      cairo_mesh_pattern_begin_patch(self.opaque);
    }
  }

  /// Indicates the end of the current patch in a mesh pattern.
  /// 
  /// If the current patch has less than 4 sides, it is closed with a straight line from the current point to the first point of the patch as if cairo_mesh_pattern_line_to() was used.
  /// 
  /// Note: If pattern is not a mesh pattern then pattern will be put into an error status with a status of CAIRO_STATUS_PATTERN_TYPE_MISMATCH. If pattern has no current patch or the current patch has no current point, pattern will be put into an error status with a status of CAIRO_STATUS_INVALID_MESH_CONSTRUCTION.
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// Since 1.12
  pub fn end_patch(&mut self) {
    unsafe {
      cairo_mesh_pattern_end_patch(self.opaque);
    }
  }

  /// Define the first point of the current patch in a mesh pattern.
  /// 
  /// After this call the current point will be (x, y).
  /// 
  /// Note: If pattern is not a mesh pattern then pattern will be put into an error status with a status of CAIRO_STATUS_PATTERN_TYPE_MISMATCH. If pattern has no current patch or the current patch already has at least one side, pattern will be put into an error status with a status of CAIRO_STATUS_INVALID_MESH_CONSTRUCTION.
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// x : the X coordinate of the new position
  /// 
  /// y : the Y coordinate of the new position
  /// 
  /// Since 1.12
  pub fn move_to(&mut self, x: f64, y: f64) {
    unsafe {
      cairo_mesh_pattern_move_to(self.opaque, x, y);
    }
  }

  /// Adds a line to the current patch from the current point to position (x, y) in pattern-space coordinates.
  /// 
  /// If there is no current point before the call to cairo_mesh_pattern_line_to() this function will behave as cairo_mesh_pattern_move_to(pattern, x, y).
  /// 
  /// After this call the current point will be (x, y).
  /// 
  /// Note: If pattern is not a mesh pattern then pattern will be put into an error status with a status of CAIRO_STATUS_PATTERN_TYPE_MISMATCH. If pattern has no current patch or the current patch already has 4 sides, pattern will be put into an error status with a status of CAIRO_STATUS_INVALID_MESH_CONSTRUCTION.
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// x : the X coordinate of the end of the new line
  /// 
  /// y : the Y coordinate of the end of the new line
  /// 
  /// Since 1.12
  pub fn line_to(&mut self, x: f64, y: f64) {
    unsafe {
      cairo_mesh_pattern_line_to(self.opaque, x, y);
    }
  }

  /// Adds a cubic Bézier spline to the current patch from the current point to position (x3, y3) in pattern-space coordinates, using (x1, y1) and (x2, y2) as the control points.
  /// 
  /// If the current patch has no current point before the call to cairo_mesh_pattern_curve_to(), this function will behave as if preceded by a call to cairo_mesh_pattern_move_to(pattern, x1, y1).
  /// 
  /// After this call the current point will be (x3, y3).
  /// 
  /// Note: If pattern is not a mesh pattern then pattern will be put into an error status with a status of CAIRO_STATUS_PATTERN_TYPE_MISMATCH. If pattern has no current patch or the current patch already has 4 sides, pattern will be put into an error status with a status of CAIRO_STATUS_INVALID_MESH_CONSTRUCTION.
  /// 
  /// pattern : a pattern::Pattern
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
  /// Since 1.12
  pub fn curve_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
    unsafe {
      cairo_mesh_pattern_curve_to(self.opaque, x1, y1, x2, y2, x3, y3);
    }
  }

  /// Set an internal control point of the current patch.
  /// 
  /// Valid values for point_num are from 0 to 3 and identify the control points as explained in cairo_pattern_create_mesh().
  /// 
  /// Note: If pattern is not a mesh pattern then pattern will be put into an error status with a status of CAIRO_STATUS_PATTERN_TYPE_MISMATCH. If point_num is not valid, pattern will be put into an error status with a status of CAIRO_STATUS_INVALID_INDEX. If pattern has no current patch, pattern will be put into an error status with a status of CAIRO_STATUS_INVALID_MESH_CONSTRUCTION.
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// point_num : the control point to set the position for
  /// 
  /// x : the X coordinate of the control point
  /// 
  /// y : the Y coordinate of the control point
  /// 
  /// Since 1.12
  pub fn set_control_point(&mut self, point_num: i32, x: f64, y: f64) {
    unsafe {
      cairo_mesh_pattern_set_control_point(self.opaque, point_num, x, y);
    }
  }

  /// Sets the color of a corner of the current patch in a mesh pattern.
  /// 
  /// The color is specified in the same way as in cairo_set_source_rgb().
  /// 
  /// Valid values for corner_num are from 0 to 3 and identify the corners as explained in cairo_pattern_create_mesh().
  /// 
  /// Note: If pattern is not a mesh pattern then pattern will be put into an error status with a status of CAIRO_STATUS_PATTERN_TYPE_MISMATCH. If corner_num is not valid, pattern will be put into an error status with a status of CAIRO_STATUS_INVALID_INDEX. If pattern has no current patch, pattern will be put into an error status with a status of CAIRO_STATUS_INVALID_MESH_CONSTRUCTION.
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// corner_num : the corner to set the color for
  /// 
  /// red : red component of color
  /// 
  /// green : green component of color
  /// 
  /// blue : blue component of color
  /// 
  /// Since 1.12
  pub fn set_corner_color_rgb(&mut self, corner_num: i32, red: f64, green: f64, blue: f64) {
    unsafe {
      cairo_mesh_pattern_set_corner_color_rgb(self.opaque, corner_num, red, green, blue);
    }
  }

  /// Sets the color of a corner of the current patch in a mesh pattern.
  /// 
  /// The color is specified in the same way as in cairo_set_source_rgba().
  /// 
  /// Valid values for corner_num are from 0 to 3 and identify the corners as explained in cairo_pattern_create_mesh().
  /// 
  /// Note: If pattern is not a mesh pattern then pattern will be put into an error status with a status of CAIRO_STATUS_PATTERN_TYPE_MISMATCH. If corner_num is not valid, pattern will be put into an error status with a status of CAIRO_STATUS_INVALID_INDEX. If pattern has no current patch, pattern will be put into an error status with a status of CAIRO_STATUS_INVALID_MESH_CONSTRUCTION.
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// corner_num :
  /// the corner to set the color for
  /// 
  /// red : red component of color
  /// 
  /// green : green component of color
  /// 
  /// blue : blue component of color
  /// 
  /// alpha : alpha component of color
  /// 
  /// Since 1.12
  pub fn set_corner_color_rgba(&mut self, corner_num: i32, red: f64, green: f64, blue: f64, alpha: f64) {
    unsafe {
      cairo_mesh_pattern_set_corner_color_rgba(self.opaque, corner_num, red, green, blue, alpha);
    }
  }

  /// Gets the number of patches specified in the given mesh pattern.
  /// 
  /// The number only includes patches which have been finished by calling cairo_mesh_pattern_end_patch(). For example it will be 0 during the definition of the first patch.
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// count : return value for the number patches, or NULL
  /// 
  /// Returns : CAIRO_STATUS_SUCCESS, or CAIRO_STATUS_PATTERN_TYPE_MISMATCH if pattern is not a mesh pattern.
  /// 
  /// Since 1.12
  pub fn get_patch_count(&mut self) -> (super::Status, i32) {
    unsafe {
      let mut count:i32 = std::intrinsics::init();
      let foreign_result = cairo_mesh_pattern_get_patch_count(self.opaque, &mut count);
      return (foreign_result, count);
    }
  }

  /// Gets path defining the patch patch_num for a mesh pattern.
  /// 
  /// patch_num can range 0 to 1 less than the number returned by cairo_mesh_pattern_get_patch_count().
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// patch_num : the patch number to return data for
  /// 
  /// Returns : the path defining the patch, or a path with status CAIRO_STATUS_INVALID_INDEX if patch_num or point_num is not valid for pattern. If pattern is not a mesh pattern, a path with status CAIRO_STATUS_PATTERN_TYPE_MISMATCH is returned.
  /// 
  /// Since 1.12
  pub fn get_path(&mut self, patch_num: i32) -> super::path::Path {
    unsafe {
      let foreign_result = cairo_mesh_pattern_get_path(self.opaque, patch_num);
      return foreign_result;
    }
  }

  /// Gets the control point point_num of patch patch_num for a mesh pattern.
  /// 
  /// patch_num can range 0 to 1 less than the number returned by cairo_mesh_pattern_get_patch_count().
  /// 
  /// Valid values for point_num are from 0 to 3 and identify the control points as explained in cairo_pattern_create_mesh().
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// patch_num : the patch number to return data for
  /// 
  /// point_num : the control point number to return data for
  ///
  /// x : return value for the x coordinate of the control point, or NULL
  /// 
  /// y : return value for the y coordinate of the control point, or NULL
  /// 
  /// Returns : CAIRO_STATUS_SUCCESS, or CAIRO_STATUS_INVALID_INDEX if patch_num or point_num is not valid for pattern. If pattern is not a mesh pattern, CAIRO_STATUS_PATTERN_TYPE_MISMATCH is returned.
  /// 
  /// Since 1.12
  pub fn get_control_point(&mut self, patch_num: i32, pointer_num: i32) -> (super::Status, f64, f64) {
    unsafe {
      let mut x:f64 = std::intrinsics::init();
      let mut y:f64 = std::intrinsics::init();
      let foreign_result = cairo_mesh_pattern_get_control_point(self.opaque, patch_num, pointer_num, &mut x, &mut y);
      return (foreign_result, x, y);
    }
  }

  /// Gets the color information in corner corner_num of patch patch_num for a mesh pattern.
  /// 
  /// patch_num can range 0 to 1 less than the number returned by cairo_mesh_pattern_get_patch_count().
  /// 
  /// Valid values for corner_num are from 0 to 3 and identify the corners as explained in cairo_pattern_create_mesh().
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// patch_num : the patch number to return data for
  /// 
  /// corner_num : the corner number to return data for
  /// 
  /// red : return value for red component of color, or NULL
  /// 
  /// green : return value for green component of color, or NULL
  /// 
  /// blue : return value for blue component of color, or NULL
  /// 
  /// alpha : return value for alpha component of color, or NULL
  /// 
  /// Returns : CAIRO_STATUS_SUCCESS, or CAIRO_STATUS_INVALID_INDEX if patch_num or corner_num is not valid for pattern. If pattern is not a mesh pattern, CAIRO_STATUS_PATTERN_TYPE_MISMATCH is returned.
  /// 
  /// Since 1.12
  pub fn get_corner_color_rgba(&mut self, patch_num: i32, pointer_num: i32) -> (super::Status, f64, f64, f64, f64) {
    unsafe {
      let mut red:f64 = std::intrinsics::init();
      let mut green:f64 = std::intrinsics::init();
      let mut blue:f64 = std::intrinsics::init();
      let mut alpha:f64 = std::intrinsics::init();
      let foreign_result = cairo_mesh_pattern_get_corner_color_rgba(self.opaque, patch_num, pointer_num, &mut red, &mut green, &mut blue, &mut alpha);
      return (foreign_result, red, green, blue, alpha);
    }
  }

  /// Checks whether an error has previously occurred for this pattern.
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// Returns : CAIRO_STATUS_SUCCESS, CAIRO_STATUS_NO_MEMORY, CAIRO_STATUS_INVALID_MATRIX, CAIRO_STATUS_PATTERN_TYPE_MISMATCH, or CAIRO_STATUS_INVALID_MESH_CONSTRUCTION.
  /// 
  /// Since 1.0
  pub fn status(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_pattern_status(self.opaque);
      return foreign_result;
    }
  }

  /// Sets the mode to be used for drawing outside the area of a pattern. See pattern::extend::Extend for details on the semantics of each extend strategy.
  /// 
  /// The default extend mode is CAIRO_EXTEND_NONE for surface patterns and CAIRO_EXTEND_PAD for gradient patterns.
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// extend : a pattern::extend::Extend describing how the area outside of the pattern will be drawn
  /// 
  /// Since 1.0
  pub fn set_extend(&mut self, extend: extend::Extend) {
    unsafe {
      cairo_pattern_set_extend(self.opaque, extend);
    }
  }


  /// Gets the current extend mode for a pattern. See pattern::extend::Extend for details on the semantics of each extend strategy.
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// Returns : the current extend strategy used for drawing the pattern.
  /// 
  /// Since 1.0
  pub fn get_extend(&mut self) -> extend::Extend {
    unsafe {
      let foreign_result = cairo_pattern_get_extend(self.opaque);
      return foreign_result;
    }
  }

  /// Sets the filter to be used for resizing when using this pattern. See pattern::filter::Filter for details on each filter.
  /// 
  /// * Note that you might want to control filtering even when you do not have an explicit pattern::Pattern object, (for example when using cairo_set_source_surface()). In these cases, it is convenient to use cairo_get_source() to get access to the pattern that cairo creates implicitly. For example:
  /// 	
  /// ```
  /// cairo_set_source_surface (cr, image, x, y);
  /// cairo_pattern_set_filter (cairo_get_source (cr), CAIRO_FILTER_NEAREST);
  /// ```
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// filter : a pattern::filter::Filter describing the filter to use for resizing the pattern
  /// 
  /// Since 1.0
  pub fn set_filter(&mut self, filter: filter::Filter) {
    unsafe {
      cairo_pattern_set_filter(self.opaque, filter);
    }
  }

  /// Gets the current filter for a pattern. See pattern::filter::Filter for details on each filter.
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// Returns : the current filter used for resizing the pattern.
  /// 
  /// Since 1.0
  pub fn get_filter(&mut self) -> filter::Filter {
    unsafe {
      let foreign_result = cairo_pattern_get_filter(self.opaque);
      return foreign_result;
    }
  }

  /// Sets the pattern's transformation matrix to matrix. This matrix is a transformation from user space to pattern space.
  /// 
  /// When a pattern is first created it always has the identity matrix for its transformation matrix, which means that pattern space is initially identical to user space.
  /// 
  /// Important: Please note that the direction of this transformation matrix is from user space to pattern space. This means that if you imagine the flow from a pattern to user space (and on to device space), then coordinates in that flow will be transformed by the inverse of the pattern matrix.
  /// 
  /// For example, if you want to make a pattern appear twice as large as it does by default the correct code to use is:
  /// 	
  /// ```
  /// cairo_matrix_init_scale (&matrix, 0.5, 0.5);
  /// cairo_pattern_set_matrix (pattern, &matrix);
  /// ```
  /// 
  /// Meanwhile, using values of 2.0 rather than 0.5 in the code above would cause the pattern to appear at half of its default size.
  /// 
  /// Also, please note the discussion of the user-space locking semantics of cairo_set_source().
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// matrix : a matrix::Matrix
  /// 
  /// Since 1.0
  pub fn set_matrix(&mut self, matrix: super::matrix::Matrix) {
    unsafe {
      cairo_pattern_set_matrix(self.opaque, matrix);
    }
  }

  /// Stores the pattern's transformation matrix into matrix.
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// matrix : return value for the matrix
  /// 
  /// Since 1.0
  pub fn get_matrix(&mut self) -> super::matrix::Matrix {
    unsafe {
      let foreign_result = cairo_pattern_get_matrix(self.opaque);
      return foreign_result;
    }
  }

  /// This function returns the type a pattern. See pattern::pattern_type::PatternType for available types.
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// Returns : The type of pattern.
  /// 
  /// Since 1.2
  pub fn get_type(&mut self) -> pattern_type::PatternType {
    unsafe {
      let foreign_result = cairo_pattern_get_type(self.opaque);
      return foreign_result;
    }
  }

  /// Returns the current reference count of pattern.
  /// 
  /// pattern : a pattern::Pattern
  /// 
  /// Returns : the current reference count of pattern. If the object is a nil object, 0 will be returned.
  /// 
  /// Since 1.4
  pub fn reference_count(&mut self) -> i32 {
    unsafe {
      let foreign_result = cairo_pattern_get_reference_count(self.opaque);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_pattern_add_color_stop_rgb(self_value: *mut std::libc::c_void, offset: f64, red: f64, green: f64, blue: f64);
  fn cairo_pattern_add_color_stop_rgba(self_value: *mut std::libc::c_void, offset: f64, red: f64, green: f64, blue: f64, alpha: f64);
  fn cairo_pattern_get_color_stop_count(self_value: *mut std::libc::c_void, stop_count: *mut i32) -> super::Status;
  fn cairo_pattern_get_color_stop_rgba(self_value: *mut std::libc::c_void, stop_count: i32, offset: *mut f64, red: *mut f64, green: *mut f64, blue: *mut f64, alpha: *mut f64) -> super::Status;
  fn cairo_pattern_create_rgb(red: f64, green: f64, blue: f64) -> Pattern;
  fn cairo_pattern_create_rgba(red: f64, green: f64, blue: f64, alpha: f64) -> Pattern;
  fn cairo_pattern_get_rgba(self_value: *mut std::libc::c_void, red: *mut f64, green: *mut f64, blue: *mut f64, alpha: *mut f64) -> super::Status;
  fn cairo_pattern_create_for_surface(surface: super::surface::Surface) -> Pattern;
  fn cairo_pattern_get_surface(self_value: *mut std::libc::c_void, surface: *mut super::surface::Surface) -> super::Status;
  fn cairo_pattern_create_linear(x0: f64, y0: f64, x1: f64, y1: f64) -> Pattern;
  fn cairo_pattern_get_linear_points(self_value: *mut std::libc::c_void, x0: *mut f64, y0: *mut f64, x1: *mut f64, y1: *mut f64) -> super::Status;
  fn cairo_pattern_create_radial(cx0: f64, cy0: f64, radius0: f64, cx1: f64, cy1: f64, radius1: f64) -> Pattern;
  fn cairo_pattern_get_radial_circles(self_value: *mut std::libc::c_void, x0: *mut f64, y0: *mut f64, r0: *mut f64, x1: *mut f64, y1: *mut f64, r1: *mut f64) -> super::Status;
  fn cairo_pattern_create_mesh() -> Pattern;
  fn cairo_mesh_pattern_begin_patch(self_value: *mut std::libc::c_void);
  fn cairo_mesh_pattern_end_patch(self_value: *mut std::libc::c_void);
  fn cairo_mesh_pattern_move_to(self_value: *mut std::libc::c_void, x: f64, y: f64);
  fn cairo_mesh_pattern_line_to(self_value: *mut std::libc::c_void, x: f64, y: f64);
  fn cairo_mesh_pattern_curve_to(self_value: *mut std::libc::c_void, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64);
  fn cairo_mesh_pattern_set_control_point(self_value: *mut std::libc::c_void, point_num: i32, x: f64, y: f64);
  fn cairo_mesh_pattern_set_corner_color_rgb(self_value: *mut std::libc::c_void, corner_num: i32, red: f64, green: f64, blue: f64);
  fn cairo_mesh_pattern_set_corner_color_rgba(self_value: *mut std::libc::c_void, corner_num: i32, red: f64, green: f64, blue: f64, alpha: f64);
  fn cairo_mesh_pattern_get_patch_count(self_value: *mut std::libc::c_void, count: *mut i32) -> super::Status;
  fn cairo_mesh_pattern_get_path(self_value: *mut std::libc::c_void, patch_num: i32) -> super::path::Path;
  fn cairo_mesh_pattern_get_control_point(self_value: *mut std::libc::c_void, patch_num: i32, pointer_num: i32, x: *mut f64, y: *mut f64) -> super::Status;
  fn cairo_mesh_pattern_get_corner_color_rgba(self_value: *mut std::libc::c_void, patch_num: i32, pointer_num: i32, red: *mut f64, green: *mut f64, blue: *mut f64, alpha: *mut f64) -> super::Status;
  fn cairo_pattern_status(self_value: *mut std::libc::c_void) -> super::Status;
  fn cairo_pattern_set_extend(self_value: *mut std::libc::c_void, extend: extend::Extend);
  fn cairo_pattern_get_extend(self_value: *mut std::libc::c_void) -> extend::Extend;
  fn cairo_pattern_set_filter(self_value: *mut std::libc::c_void, filter: filter::Filter);
  fn cairo_pattern_get_filter(self_value: *mut std::libc::c_void) -> filter::Filter;
  fn cairo_pattern_set_matrix(self_value: *mut std::libc::c_void, matrix: super::matrix::Matrix);
  fn cairo_pattern_get_matrix(self_value: *mut std::libc::c_void) -> super::matrix::Matrix;
  fn cairo_pattern_get_type(self_value: *mut std::libc::c_void) -> pattern_type::PatternType;
  fn cairo_pattern_get_reference_count(self_value: *mut std::libc::c_void) -> i32;
}

impl std::clone::Clone for Pattern {
  fn clone(&self) -> Pattern {
    unsafe {
      let foreign_result = cairo_pattern_reference(self.opaque as *std::libc::c_void);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_pattern_reference(self_value: *std::libc::c_void) -> Pattern;
}

impl std::ops::Drop for Pattern {
  fn drop(&mut self) {
    unsafe {
      cairo_pattern_destroy(self.opaque);
    }
  }
}

extern {
  fn cairo_pattern_destroy(self_value: *mut std::libc::c_void);
}

pub mod extend;
pub mod filter;
pub mod pattern_type;

