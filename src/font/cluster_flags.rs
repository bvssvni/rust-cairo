//! Describe font cluster flags

///  Specifies properties of a text cluster mapping.
/// 
/// Since 1.8
#[repr(i32)]
pub enum ClusterFlags {
  /// The clusters in the cluster array map to glyphs in the glyph array from start to end.
  Forwards = 0,
  /// The clusters in the cluster array map to glyphs in the glyph array from end to start. (Since 1.8)
  Backwards = 1
}

