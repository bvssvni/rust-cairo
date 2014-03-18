///  cairo_operator_t is used to set the compositing operator for all cairo drawing operations.
///
/// The default operator is CAIRO_OPERATOR_OVER.
///
/// The operators marked as unbounded modify their destination even outside of the mask layer (that is, their effect is not bound by the mask layer). However, their effect can still be limited by way of clipping.
///
/// To keep things simple, the operator descriptions here document the behavior for when both source and destination are either fully transparent or fully opaque. The actual implementation works for translucent layers too. For a more detailed explanation of the effects of each operator, including the mathematical definitions, see http://cairographics.org/operators/.
///
/// CAIRO_OPERATOR_CLEAR
///	clear destination layer (bounded) (Since 1.0)
///
/// CAIRO_OPERATOR_SOURCE
/// 	replace destination layer (bounded) (Since 1.0)
/// 
/// CAIRO_OPERATOR_OVER
/// 	draw source layer on top of destination layer (bounded) (Since 1.0)
/// 
/// CAIRO_OPERATOR_IN
/// 	draw source where there was destination content (unbounded) (Since 1.0)
/// 
/// CAIRO_OPERATOR_OUT
/// 	draw source where there was no destination content (unbounded) (Since 1.0)
/// 
/// CAIRO_OPERATOR_ATOP
/// 	draw source on top of destination content and only there (Since 1.0)
/// 
/// CAIRO_OPERATOR_DEST
/// 	ignore the source (Since 1.0)
/// 
/// CAIRO_OPERATOR_DEST_OVER
/// 	draw destination on top of source (Since 1.0)
/// 
/// CAIRO_OPERATOR_DEST_IN
/// 	leave destination only where there was source content (unbounded) (Since 1.0)
/// 
/// CAIRO_OPERATOR_DEST_OUT
///	leave destination only where there was no source content (Since 1.0)
///
/// CAIRO_OPERATOR_DEST_ATOP
/// 	leave destination on top of source content and only there (unbounded) (Since 1.0)
/// 
/// CAIRO_OPERATOR_XOR
/// 	source and destination are shown where there is only one of them (Since 1.0)
/// 
/// CAIRO_OPERATOR_ADD
/// 	source and destination layers are accumulated (Since 1.0)
/// 
/// CAIRO_OPERATOR_SATURATE
/// 	like over, but assuming source and dest are disjoint geometries (Since 1.0)
/// 
/// CAIRO_OPERATOR_MULTIPLY
/// 	source and destination layers are multiplied. This causes the result to be at least as dark as the darker inputs. (Since 1.10)
/// 
/// CAIRO_OPERATOR_SCREEN
/// 	source and destination are complemented and multiplied. This causes the result to be at least as light as the lighter inputs. (Since 1.10)
/// 
/// CAIRO_OPERATOR_OVERLAY
/// 	multiplies or screens, depending on the lightness of the destination color. (Since 1.10)
/// 
/// CAIRO_OPERATOR_DARKEN
/// 	replaces the destination with the source if it is darker, otherwise keeps the source. (Since 1.10)
/// 
/// CAIRO_OPERATOR_LIGHTEN
/// 	replaces the destination with the source if it is lighter, otherwise keeps the source. (Since 1.10)
/// 
/// CAIRO_OPERATOR_COLOR_DODGE
/// 	brightens the destination color to reflect the source color. (Since 1.10)
/// 
/// CAIRO_OPERATOR_COLOR_BURN
/// 	darkens the destination color to reflect the source color. (Since 1.10)
/// 
/// CAIRO_OPERATOR_HARD_LIGHT
/// 	Multiplies or screens, dependent on source color. (Since 1.10)
/// 
/// CAIRO_OPERATOR_SOFT_LIGHT
/// 	Darkens or lightens, dependent on source color. (Since 1.10)
/// 
/// CAIRO_OPERATOR_DIFFERENCE
/// 	Takes the difference of the source and destination color. (Since 1.10)
/// 
/// CAIRO_OPERATOR_EXCLUSION
/// 	Produces an effect similar to difference, but with lower contrast. (Since 1.10)
/// 
/// CAIRO_OPERATOR_HSL_HUE
/// 	Creates a color with the hue of the source and the saturation and luminosity of the target. (Since 1.10)
/// 
/// CAIRO_OPERATOR_HSL_SATURATION
/// 	Creates a color with the saturation of the source and the hue and luminosity of the target. Painting with this mode onto a gray area produces no change. (Since 1.10)
/// 
/// CAIRO_OPERATOR_HSL_COLOR
/// 	Creates a color with the hue and saturation of the source and the luminosity of the target. This preserves the gray levels of the target and is useful for coloring monochrome images or tinting color images. (Since 1.10)
/// 
/// CAIRO_OPERATOR_HSL_LUMINOSITY
/// 	Creates a color with the luminosity of the source and the hue and saturation of the target. This produces an inverse effect to CAIRO_OPERATOR_HSL_COLOR. (Since 1.10)
/// 
/// Since 1.0
#[repr(i32)]
pub enum Operator {
  Clear = 0,
  Source = 1,
  Over = 2,
  In = 3,
  Out = 4,
  Atop = 5,
  Dest = 6,
  DestOver = 7,
  DestIn = 8,
  DestOut = 9,
  DestAtop = 10,
  Xor = 11,
  Add = 12,
  Saturate = 13,
  Multiply = 14,
  Screen = 15,
  Overlay = 16,
  Darken = 17,
  Lighten = 18,
  ColorDodge = 19,
  ColorBurn = 20,
  HardLight = 21,
  SoftLight = 22,
  Difference = 23,
  Exclusion = 24,
  HSLHue = 25,
  HSLSaturation = 26,
  HSLColor = 27,
  HSLLuminosity = 28
}

