//! Describe composition order for drawing operations

///  cairo_operator_t is used to set the compositing operator for all cairo drawing operations.
///
/// The default operator is CAIRO_OPERATOR_OVER.
///
/// The operators marked as unbounded modify their destination even outside of the mask layer (that is, their effect is not bound by the mask layer). However, their effect can still be limited by way of clipping.
///
/// To keep things simple, the operator descriptions here document the behavior for when both source and destination are either fully transparent or fully opaque. The actual implementation works for translucent layers too. For a more detailed explanation of the effects of each operator, including the mathematical definitions, see http://cairographics.org/operators/.
///
/// Since 1.0
#[repr(i32)]
pub enum Operator {
  /// clear destination layer (bounded) (Since 1.0)
  Clear = 0,
  /// replace destination layer (bounded) (Since 1.0)
  Source = 1,
  /// draw source layer on top of destination layer (bounded) (Since 1.0)
  Over = 2,
  /// draw source where there was destination content (unbounded) (Since 1.0)
  In = 3,
  /// draw source where there was no destination content (unbounded) (Since 1.0)
  Out = 4,
  /// draw source on top of destination content and only there (Since 1.0)
  Atop = 5,
  /// ignore the source (Since 1.0)
  Dest = 6,
  /// draw destination on top of source (Since 1.0)
  DestOver = 7,
  /// leave destination only where there was source content (unbounded) (Since 1.0)
  DestIn = 8,
  /// leave destination only where there was no source content (Since 1.0)
  DestOut = 9,
  /// leave destination on top of source content and only there (unbounded) (Since 1.0)
  DestAtop = 10,
  /// source and destination are shown where there is only one of them (Since 1.0)
  Xor = 11,
  /// source and destination layers are accumulated (Since 1.0)
  Add = 12,
  /// like over, but assuming source and dest are disjoint geometries (Since 1.0)
  Saturate = 13,
  /// source and destination layers are multiplied. This causes the result to be at least as dark as the darker inputs. (Since 1.10)
  Multiply = 14,
  /// source and destination are complemented and multiplied. This causes the result to be at least as light as the lighter inputs. (Since 1.10)
  Screen = 15,
  /// multiplies or screens, depending on the lightness of the destination color. (Since 1.10)
  Overlay = 16,
  /// replaces the destination with the source if it is darker, otherwise keeps the source. (Since 1.10)
  Darken = 17,
  /// replaces the destination with the source if it is lighter, otherwise keeps the source. (Since 1.10)
  Lighten = 18,
  /// brightens the destination color to reflect the source color. (Since 1.10)
  ColorDodge = 19,
  /// darkens the destination color to reflect the source color. (Since 1.10)
  ColorBurn = 20,
  /// Multiplies or screens, dependent on source color. (Since 1.10)
  HardLight = 21,
  /// Darkens or lightens, dependent on source color. (Since 1.10)
  SoftLight = 22,
  /// Takes the difference of the source and destination color. (Since 1.10)
  Difference = 23,
  /// Produces an effect similar to difference, but with lower contrast. (Since 1.10)
  Exclusion = 24,
  /// Creates a color with the hue of the source and the saturation and luminosity of the target. (Since 1.10)
  HSLHue = 25,
  /// Creates a color with the saturation of the source and the hue and luminosity of the target. Painting with this mode onto a gray area produces no change. (Since 1.10)
  HSLSaturation = 26,
  /// Creates a color with the hue and saturation of the source and the luminosity of the target. This preserves the gray levels of the target and is useful for coloring monochrome images or tinting color images. (Since 1.10)
  HSLColor = 27,
  /// Creates a color with the luminosity of the source and the hue and saturation of the target. This produces an inverse effect to CAIRO_OPERATOR_HSL_COLOR. (Since 1.10)
  HSLLuminosity = 28
}

