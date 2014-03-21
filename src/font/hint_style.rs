//! Describe font hint style

/// Specifies the type of hinting to do on font outlines. Hinting is the process of fitting outlines to the pixel grid in order to improve the appearance of the result. Since hinting outlines involves distorting them, it also reduces the faithfulness to the original outline shapes. Not all of the outline hinting styles are supported by all font backends.
/// 
/// New entries may be added in future versions.
/// 
/// Since 1.0
#[repr(i32)]
pub enum HintStyle {
  /// Use the default hint style for font backend and target device, since 1.0
  Default = 0,
  /// Do not hint outlines, since 1.0
  None = 1,
  /// Hint outlines slightly to improve contrast while retaining good fidelity to the original shapes, since 1.0
  Slight = 2,
  /// Hint outlines with medium strength giving a compromise between fidelity to the original shapes and contrast, since 1.0
  Medium = 3,
  /// Hint outlines to maximize contrast, since 1.0
  Full = 4
}

