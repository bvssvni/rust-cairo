use std;
use super::DeepClone;

pub struct Options {
  opaque: *mut std::libc::c_void
}

pub struct FontFace {
  opaque: *mut std::libc::c_void
}

pub struct ScaledFont {
  opaque: *mut std::libc::c_void
}

pub struct Glyph {
  index: i64,
  x: f64,
  y: f64
}

pub struct Cluster {
  num_bytes: i32,
  num_glyphs: i32
}

pub struct FontExtents {
  ascent: f64,
  descent: f64,
  height: f64,
  max_x_advance: f64,
  max_y_advance: f64
}

pub struct TextExtents {
  x_bearing: f64,
  y_bearing: f64,
  width: f64,
  height: f64,
  max_x_advance: f64,
  max_y_advance: f64
}

impl Options {
  pub fn new() -> Options {
    unsafe {
      let foreign_result = cairo_font_options_create();
      return foreign_result;
    }
  }

  pub fn status(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_font_options_status(self.opaque);
      return foreign_result;
    }
  }

  pub fn merge(&mut self, other: &Options) {
    unsafe {
      cairo_font_options_merge(self.opaque, other);
    }
  }

  pub fn hash(&mut self) -> i64 {
    unsafe {
      let foreign_result = cairo_font_options_hash(self.opaque);
      return foreign_result;
    }
  }

  pub fn equal(&mut self, other: &Options) -> bool {
    unsafe {
      let foreign_result = cairo_font_options_equal(self.opaque, other.opaque as *std::libc::c_void);
      return foreign_result != 0;
    }
  }

  pub fn set_antialias(&mut self, antialias: super::antialias::Antialias) {
    unsafe {
      cairo_font_options_set_antialias(self.opaque, antialias);
    }
  }

  pub fn get_antialias(&mut self) -> super::antialias::Antialias {
    unsafe {
      let foreign_result = cairo_font_options_get_antialias(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_subpixel_order(&mut self, subpixel_order: subpixel_order::SubpixelOrder) {
    unsafe {
      cairo_font_options_set_subpixel_order(self.opaque, subpixel_order);
    }
  }

  pub fn get_subpixel_order(&mut self) -> subpixel_order::SubpixelOrder {
    unsafe {
      let foreign_result = cairo_font_options_get_subpixel_order(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_hint_style(&mut self, hint_style: hint_style::HintStyle) {
    unsafe {
      cairo_font_options_set_hint_style(self.opaque, hint_style);
    }
  }

  pub fn get_hint_style(&mut self) -> hint_style::HintStyle {
    unsafe {
      let foreign_result = cairo_font_options_get_hint_style(self.opaque);
      return foreign_result;
    }
  }

  pub fn set_hint_metrics(&mut self, hint_metrics: hint_metrics::HintMetrics) {
    unsafe {
      cairo_font_options_set_hint_metrics(self.opaque, hint_metrics);
    }
  }

  pub fn get_hint_metrics(&mut self) -> hint_metrics::HintMetrics {
    unsafe {
      let foreign_result = cairo_font_options_get_hint_metrics(self.opaque);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_font_options_create() -> Options;
  fn cairo_font_options_status(self_value: *mut std::libc::c_void) -> super::Status;
  fn cairo_font_options_merge(self_value: *mut std::libc::c_void, other: *Options);
  fn cairo_font_options_hash(self_value: *mut std::libc::c_void) -> i64;
  fn cairo_font_options_equal(self_value: *mut std::libc::c_void, other: *std::libc::c_void) -> i32;
  fn cairo_font_options_set_antialias(self_value: *mut std::libc::c_void, antialias: super::antialias::Antialias);
  fn cairo_font_options_get_antialias(self_value: *mut std::libc::c_void) -> super::antialias::Antialias;
  fn cairo_font_options_set_subpixel_order(self_value: *mut std::libc::c_void, subpixel_order: subpixel_order::SubpixelOrder);
  fn cairo_font_options_get_subpixel_order(self_value: *mut std::libc::c_void) -> subpixel_order::SubpixelOrder;
  fn cairo_font_options_set_hint_style(self_value: *mut std::libc::c_void, hint_style: hint_style::HintStyle);
  fn cairo_font_options_get_hint_style(self_value: *mut std::libc::c_void) -> hint_style::HintStyle;
  fn cairo_font_options_set_hint_metrics(self_value: *mut std::libc::c_void, hint_metrics: hint_metrics::HintMetrics);
 fn cairo_font_options_get_hint_metrics(self_value: *mut std::libc::c_void) -> hint_metrics::HintMetrics;
}

impl std::clone::Clone for Options {
  fn clone(&self) -> Options {
    unsafe {
      let foreign_result = cairo_font_options_copy(self.opaque as *std::libc::c_void);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_font_options_copy(self_value: *std::libc::c_void) -> Options;
}

impl DeepClone for Options {
  fn deep_clone(&self) -> Options {
    unsafe {
      let foreign_result = cairo_font_options_copy(self.opaque as *std::libc::c_void);
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
  fn cairo_font_options_destroy(self_value: *mut std::libc::c_void);
}

impl FontFace {
  pub fn toy(family: &str, slant: slant::Slant, weight: weight::Weight) -> FontFace {
    unsafe {
      use std::c_str::ToCStr;
      let foreign_result = cairo_toy_font_face_create(family.to_c_str().unwrap(), slant, weight);
      return foreign_result;
    }
  }

  pub fn toy_get_family(&mut self) -> std::c_str::CString {
    unsafe {
      let foreign_result = cairo_toy_font_face_get_family(self.opaque);
      return std::c_str::CString::new(foreign_result, false);
    }
  }

  pub fn toy_get_slant(&mut self) -> slant::Slant {
    unsafe {
      let foreign_result = cairo_toy_font_face_get_slant(self.opaque);
      return foreign_result;
    }
  }

  pub fn toy_get_weight(&mut self) -> slant::Slant {
    unsafe {
      let foreign_result = cairo_toy_font_face_get_weight(self.opaque);
      return foreign_result;
    }
  }

  pub fn status(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_font_face_status(self.opaque);
      return foreign_result;
    }
  }

  pub fn get_type(&mut self) -> font_type::FontType {
    unsafe {
      let foreign_result = cairo_font_face_get_type(self.opaque);
      return foreign_result;
    }
  }

  pub fn reference_count(&mut self) -> i32 {
    unsafe {
      let foreign_result = cairo_font_face_get_reference_count(self.opaque);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_toy_font_face_create(family: *std::libc::c_char, slant: slant::Slant, weight: weight::Weight) -> FontFace;
  fn cairo_toy_font_face_get_family(self_value: *mut std::libc::c_void) -> *i8;
  fn cairo_toy_font_face_get_slant(self_value: *mut std::libc::c_void) -> slant::Slant;
  fn cairo_toy_font_face_get_weight(self_value: *mut std::libc::c_void) -> slant::Slant;
  fn cairo_font_face_status(self_value: *mut std::libc::c_void) -> super::Status;
  fn cairo_font_face_get_type(self_value: *mut std::libc::c_void) -> font_type::FontType;
  fn cairo_font_face_get_reference_count(self_value: *mut std::libc::c_void) -> i32;
}

impl std::clone::Clone for FontFace {
  fn clone(&self) -> FontFace {
    unsafe {
      let foreign_result = cairo_font_face_reference(self.opaque as *std::libc::c_void);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_font_face_reference(self_value: *std::libc::c_void) -> FontFace;
}

impl std::ops::Drop for FontFace {
  fn drop(&mut self) {
    unsafe {
      cairo_font_face_destroy(self.opaque);
    }
  }
}

extern {
  fn cairo_font_face_destroy(self_value: *mut std::libc::c_void);
}

impl ScaledFont {
  pub fn new(font_face: &mut FontFace, font_matrix: &super::matrix::Matrix, ctm: &super::matrix::Matrix, options: &mut Options) -> ScaledFont {
    unsafe {
      let foreign_result = cairo_scaled_font_create(font_face, font_matrix, ctm, options);
      return foreign_result;
    }
  }

  pub fn status(&mut self) -> super::Status {
    unsafe {
      let foreign_result = cairo_scaled_font_status(self.opaque);
      return foreign_result;
    }
  }

  pub fn font_extents(&mut self) -> FontExtents {
    unsafe {
      let mut extents:FontExtents = std::intrinsics::init();
      cairo_scaled_font_extents(self.opaque, &mut extents);
      return extents;
    }
  }

  pub fn text_extents(&mut self, utf8: &str) -> TextExtents {
    unsafe {
      let mut extents:TextExtents = std::intrinsics::init();
      cairo_scaled_font_text_extents(self.opaque, utf8.to_c_str().unwrap(), &mut extents);
      return extents;
    }
  }

  pub fn glyph_extents(&mut self, glyphs: &[Glyph]) -> TextExtents {
    unsafe {
      let mut extents:TextExtents = std::intrinsics::init();
      cairo_scaled_font_glyph_extents(self.opaque, glyphs.as_ptr(), glyphs.len() as i32, &mut extents);
      return extents;
    }
  }

  pub fn get_font_face(&mut self) -> FontFace {
    unsafe {
      let foreign_result = cairo_scaled_font_get_font_face(self.opaque);
      return foreign_result;
    }
  }

  pub fn get_font_options(&mut self, options: FontExtents) {
    unsafe {
      cairo_scaled_font_get_font_options(self.opaque, options);
    }
  }

  pub fn get_font_matrix(&mut self) -> super::matrix::Matrix {
    unsafe {
      let mut font_matrix:super::matrix::Matrix = std::intrinsics::init();
      cairo_scaled_font_get_font_matrix(self.opaque, &mut font_matrix);
      return font_matrix;
    }
  }

  pub fn get_ctm(&mut self) -> super::matrix::Matrix {
    unsafe {
      let mut ctm:super::matrix::Matrix = std::intrinsics::init();
      cairo_scaled_font_get_ctm(self.opaque, &mut ctm);
      return ctm;
    }
  }

  pub fn get_scale_matrix(&mut self) -> super::matrix::Matrix {
    unsafe {
      let mut scale_matrix:super::matrix::Matrix = std::intrinsics::init();
      cairo_scaled_font_get_scale_matrix(self.opaque, &mut scale_matrix);
      return scale_matrix;
    }
  }

  pub fn get_type(&mut self) -> font_type::FontType {
    unsafe {
      let foreign_result = cairo_scaled_font_get_type(self.opaque);
      return foreign_result;
    }
  }

  pub fn reference_count(&mut self) -> i32 {
    unsafe {
      let foreign_result = cairo_scaled_font_get_reference_count(self.opaque);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_scaled_font_create(font_face: *mut FontFace, font_matrix: *super::matrix::Matrix, ctm: *super::matrix::Matrix, options: *mut Options) -> ScaledFont;
  fn cairo_scaled_font_status(self_value: *mut std::libc::c_void) -> super::Status;
  fn cairo_scaled_font_extents(self_value: *mut std::libc::c_void, extents: *mut FontExtents);
  fn cairo_scaled_font_text_extents(self_value: *mut std::libc::c_void, utf8: *std::libc::c_char, extents: *mut TextExtents);
  fn cairo_scaled_font_glyph_extents(self_value: *mut std::libc::c_void, glyphs: *Glyph, glyphs_length: i32, extents: *mut TextExtents);
  fn cairo_scaled_font_get_font_face(self_value: *mut std::libc::c_void) -> FontFace;
  fn cairo_scaled_font_get_font_options(self_value: *mut std::libc::c_void, options: FontExtents);
  fn cairo_scaled_font_get_font_matrix(self_value: *mut std::libc::c_void, font_matrix: *mut super::matrix::Matrix);
  fn cairo_scaled_font_get_ctm(self_value: *mut std::libc::c_void, ctm: *mut super::matrix::Matrix);
  fn cairo_scaled_font_get_scale_matrix(self_value: *mut std::libc::c_void, scale_matrix: *mut super::matrix::Matrix);
  fn cairo_scaled_font_get_type(self_value: *mut std::libc::c_void) -> font_type::FontType;
  fn cairo_scaled_font_get_reference_count(self_value: *mut std::libc::c_void) -> i32;
}

impl std::clone::Clone for ScaledFont {
  fn clone(&self) -> ScaledFont {
    unsafe {
      let foreign_result = cairo_scaled_font_reference(self.opaque as *std::libc::c_void);
      return foreign_result;
    }
  }
}

extern {
  fn cairo_scaled_font_reference(self_value: *std::libc::c_void) -> ScaledFont;
}

impl std::ops::Drop for ScaledFont {
  fn drop(&mut self) {
    unsafe {
      cairo_scaled_font_destroy(self.opaque);
    }
  }
}

extern {
  fn cairo_scaled_font_destroy(self_value: *mut std::libc::c_void);
}

pub mod cluster_flags;
pub mod font_type;
pub mod slant;
pub mod weight;
pub mod subpixel_order;
pub mod hint_style;
pub mod hint_metrics;

