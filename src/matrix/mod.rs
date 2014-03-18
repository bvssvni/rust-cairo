//! Generic matrix operations

use std;

///  A cairo_matrix_t holds an affine transformation, such as a scale, rotation, shear, or a combination of those. The transformation of a point (x, y) is given by:
///
/// ```
/// x_new = xx * x + xy * y + x0;
/// y_new = yx * x + yy * y + y0;
/// ```
///
/// double xx; xx component of the affine transformation
///
/// double yx; yx component of the affine transformation
///
/// double xy; xy component of the affine transformation
///
/// double yy; yy component of the affine transformation
///
/// double x0; X translation component of the affine transformation
///
/// double y0; Y translation component of the affine transformation
///
/// Since 1.0
pub struct Matrix {
  xx: f64,
  yx: f64,
  xy: f64,
  yy: f64,
  x0: f64,
  y0: f64
}

impl Matrix {
  ///  Sets matrix to be the affine transformation given by xx, yx, xy, yy, x0, y0. The transformation is given by:
  ///
  /// ```
  /// x_new = xx * x + xy * y + x0;
  /// y_new = yx * x + yy * y + y0;
  /// ```
  ///
  /// matrix : a cairo_matrix_t
  ///
  /// xx : xx component of the affine transformation
  ///
  /// yx : yx component of the affine transformation
  ///
  /// xy : xy component of the affine transformation
  ///
  /// yy : yy component of the affine transformation
  ///
  /// x0 : X translation component of the affine transformation
  ///
  /// y0 : Y translation component of the affine transformation
  ///
  /// Since 1.0
  pub fn new(xx: f64, yx: f64, xy: f64, yy: f64, x0: f64, y0: f64) -> Matrix {
    unsafe {
      let mut this:Matrix = std::intrinsics::init();
      cairo_matrix_init(&mut this, xx, yx, xy, yy, x0, y0);
      return this;
    }
  }

  ///  Modifies matrix to be an identity transformation.
  /// 
  /// matrix : a cairo_matrix_t
  ///
  /// Since 1.0
  pub fn identity() -> Matrix {
    unsafe {
      let mut this:Matrix = std::intrinsics::init();
      cairo_matrix_init_identity(&mut this);
      return this;
    }
  }

  ///  Initializes matrix to a transformation that translates by tx and ty in the X and Y dimensions, respectively.
  ///
  /// matrix : a cairo_matrix_t
  ///
  /// tx : amount to translate in the X direction
  ///
  /// ty : amount to translate in the Y direction
  ///
  /// Since 1.0
  pub fn for_translation(x0: f64, y0: f64) -> Matrix {
    unsafe {
      let mut this:Matrix = std::intrinsics::init();
      cairo_matrix_init_translate(&mut this, x0, y0);
      return this;
    }
  }

  ///  Initializes matrix to a transformation that scales by sx and sy in the X and Y dimensions, respectively.
  ///
  /// matrix : a cairo_matrix_t
  ///
  /// sx : scale factor in the X direction
  ///
  /// sy : scale factor in the Y direction
  ///
  /// Since 1.0
  pub fn for_scale(sx: f64, sy: f64) -> Matrix {
    unsafe {
      let mut this:Matrix = std::intrinsics::init();
      cairo_matrix_init_scale(&mut this, sx, sy);
      return this;
    }
  }

  ///  Initialized matrix to a transformation that rotates by radians.
  ///
  /// matrix : a cairo_matrix_t
  ///
  /// radians : angle of rotation, in radians. The direction of rotation is defined such that positive angles rotate in the direction from the positive X axis toward the positive Y axis. With the default axis orientation of cairo, positive angles rotate in a clockwise direction.
  ///
  /// Since 1.0
  pub fn for_rotation(radians: f64) -> Matrix {
    unsafe {
      let mut this:Matrix = std::intrinsics::init();
      cairo_matrix_init_rotate(&mut this, radians);
      return this;
    }
  }

  ///  Multiplies the affine transformations in a and b together and stores the result in result. The effect of the resulting transformation is to first apply the transformation in a to the coordinates and then apply the transformation in b to the coordinates.
  ///
  /// It is allowable for result to be identical to either a or b.
  ///
  /// result : a cairo_matrix_t in which to store the result
  ///
  /// a : a cairo_matrix_t
  ///
  /// b : a cairo_matrix_t
  ///
  /// Since 1.0
  pub fn multiply(a: &Matrix, b: &Matrix) -> Matrix {
    unsafe {
      let mut this:Matrix = std::intrinsics::init();
      cairo_matrix_multiply(&mut this, a, b);
      return this;
    }
  }

  ///  Applies a translation by tx, ty to the transformation in matrix. The effect of the new transformation is to first translate the coordinates by tx and ty, then apply the original transformation to the coordinates.
  ///
  /// matrix : a cairo_matrix_t
  ///
  /// tx : amount to translate in the X direction
  ///
  /// ty : amount to translate in the Y direction
  ///
  /// Since 1.0
  pub fn translate(&mut self, x0: f64, y0: f64) {
    unsafe {
      cairo_matrix_translate(self, x0, y0);
    }
  }

  ///  Applies scaling by sx, sy to the transformation in matrix. The effect of the new transformation is to first scale the coordinates by sx and sy, then apply the original transformation to the coordinates.
  ///
  /// matrix : a cairo_matrix_t
  ///
  /// sx : scale factor in the X direction
  ///
  /// sy : scale factor in the Y direction
  ///
  /// Since 1.0
  pub fn scale(&mut self, sx: f64, sy: f64) {
    unsafe {
      cairo_matrix_scale(self, sx, sy);
    }
  }

  ///  Applies rotation by radians to the transformation in matrix. The effect of the new transformation is to first rotate the coordinates by radians, then apply the original transformation to the coordinates.
  ///
  /// matrix : a cairo_matrix_t
  ///
  /// radians : angle of rotation, in radians. The direction of rotation is defined such that positive angles rotate in the direction from the positive X axis toward the positive Y axis. With the default axis orientation of cairo, positive angles rotate in a clockwise direction.
  ///
  /// Since 1.0
  pub fn rotate(&mut self, radians: f64) {
    unsafe {
      cairo_matrix_rotate(self, radians);
    }
  }

  ///  Transforms the distance vector (dx,dy) by matrix. This is similar to cairo_matrix_transform_point() except that the translation components of the transformation are ignored. The calculation of the returned vector is as follows:
  ///
  /// ```
  /// dx2 = dx1 * a + dy1 * c;
  /// dy2 = dx1 * b + dy1 * d;
  /// ```
  ///
  /// Affine transformations are position invariant, so the same vector always transforms to the same vector. If (x1,y1) transforms to (x2,y2) then (x1+dx1,y1+dy1) will transform to (x1+dx2,y1+dy2) for all values of x1 and x2.
  ///
  /// matrix : a cairo_matrix_t
  ///
  /// dx : X component of a distance vector. An in/out parameter
  ///
  /// dy : Y component of a distance vector. An in/out parameter
  ///
  /// Since 1.0
  pub fn transform_distance(&self, dx_r: f64, dy_r: f64) -> (f64, f64) {
    unsafe {
      let mut dx = dx_r;
      let mut dy = dy_r;
      cairo_matrix_transform_distance(self, &mut dx, &mut dy);
      return (dx, dy);
    }
  }

  /// Transforms the point (x, y) by matrix.
  ///
  /// matrix : a cairo_matrix_t
  ///
  /// x : X position. An in/out parameter
  ///
  /// y : Y position. An in/out parameter
  ///
  /// Since 1.0
  pub fn transform_point(&self, x_r: f64, y_r: f64) -> (f64, f64) {
    unsafe {
      let mut x = x_r;
      let mut y = y_r;
      cairo_matrix_transform_point(self, &mut x, &mut y);
      return (x, y);
    }
  }

  /// Changes matrix to be the inverse of its original value. Not all transformation matrices have inverses; if the matrix collapses points together (it is degenerate), then it has no inverse and this function will fail.
  ///
  /// matrix : a cairo_matrix_t
  ///
  /// Returns : If matrix has an inverse, modifies matrix to be the inverse matrix and returns CAIRO_STATUS_SUCCESS. Otherwise, returns CAIRO_STATUS_INVALID_MATRIX.
  ///
  /// Since 1.0
  pub fn invert(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_matrix_invert(self);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_matrix_init(this: *mut Matrix, xx: f64, yx: f64, xy: f64, yy: f64, x0: f64, y0: f64);
  fn cairo_matrix_init_identity(this: *mut Matrix);
  fn cairo_matrix_init_translate(this: *mut Matrix, x0: f64, y0: f64);
  fn cairo_matrix_init_scale(this: *mut Matrix, sx: f64, sy: f64);
  fn cairo_matrix_init_rotate(this: *mut Matrix, radians: f64);
  fn cairo_matrix_multiply(this: *mut Matrix, a: *Matrix, b: *Matrix);
  fn cairo_matrix_translate(self_value: *mut Matrix, x0: f64, y0: f64);
  fn cairo_matrix_scale(self_value: *mut Matrix, sx: f64, sy: f64);
  fn cairo_matrix_rotate(self_value: *mut Matrix, radians: f64);
  fn cairo_matrix_transform_distance(self_value: *Matrix, dx: &mut f64, dy: &mut f64);
  fn cairo_matrix_transform_point(self_value: *Matrix, x: &mut f64, y: &mut f64);
  fn cairo_matrix_invert(self_value: *mut Matrix) -> super::Status;
}

