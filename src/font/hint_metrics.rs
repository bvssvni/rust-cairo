//! Describe font hint metrics

/// Specifies whether to hint font metrics; hinting font metrics means quantizing them so that they are integer values in device space. Doing this improves the consistency of letter and line spacing, however it also means that text will be laid out differently at different zoom factors.
/// 
/// Since 1.0
#[repr(i32)]
pub enum HintMetrics {
  /// Hint metrics in the default manner for the font backend and target device, since 1.0
  Default = 0,
  /// Do not hint font metrics, since 1.0
  Off = 1,
  /// Hint font metrics, since 1.0
  On = 2
}

