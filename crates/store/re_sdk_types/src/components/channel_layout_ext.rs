use super::ChannelLayout;

// Channel bit positions matching WAVEFORMATEXTENSIBLE and Symphonia's Channels.
const FRONT_LEFT: u32 = 1 << 0;
const FRONT_RIGHT: u32 = 1 << 1;
const FRONT_CENTRE: u32 = 1 << 2;
const LFE1: u32 = 1 << 3;
const REAR_LEFT: u32 = 1 << 4;
const REAR_RIGHT: u32 = 1 << 5;
const SIDE_LEFT: u32 = 1 << 9;
const SIDE_RIGHT: u32 = 1 << 10;

impl ChannelLayout {
    /// Mono: Front Center only.
    pub const MONO: Self = Self(crate::datatypes::UInt32(FRONT_CENTRE));

    /// Stereo: Front Left + Front Right.
    pub const STEREO: Self = Self(crate::datatypes::UInt32(FRONT_LEFT | FRONT_RIGHT));

    /// 2.1: Stereo + LFE.
    pub const STEREO_LFE: Self = Self(crate::datatypes::UInt32(FRONT_LEFT | FRONT_RIGHT | LFE1));

    /// 5.1 Surround: Front L/R, Center, LFE, Rear L/R.
    pub const SURROUND_5_1: Self = Self(crate::datatypes::UInt32(
        FRONT_LEFT | FRONT_RIGHT | FRONT_CENTRE | LFE1 | REAR_LEFT | REAR_RIGHT,
    ));

    /// 7.1 Surround: 5.1 + Side L/R.
    pub const SURROUND_7_1: Self = Self(crate::datatypes::UInt32(
        FRONT_LEFT
            | FRONT_RIGHT
            | FRONT_CENTRE
            | LFE1
            | REAR_LEFT
            | REAR_RIGHT
            | SIDE_LEFT
            | SIDE_RIGHT,
    ));

    /// Returns the number of channels in this layout.
    #[inline]
    pub fn channel_count(&self) -> u32 {
        self.0 .0.count_ones()
    }

    /// Returns true if this layout contains the Front Left channel.
    #[inline]
    pub fn has_front_left(&self) -> bool {
        self.0 .0 & FRONT_LEFT != 0
    }

    /// Returns true if this layout contains the Front Right channel.
    #[inline]
    pub fn has_front_right(&self) -> bool {
        self.0 .0 & FRONT_RIGHT != 0
    }

    /// Returns true if this layout contains the Front Centre channel.
    #[inline]
    pub fn has_front_centre(&self) -> bool {
        self.0 .0 & FRONT_CENTRE != 0
    }

    /// Returns true if this layout contains the LFE (subwoofer) channel.
    #[inline]
    pub fn has_lfe(&self) -> bool {
        self.0 .0 & LFE1 != 0
    }

    /// Returns the raw bitmask value.
    #[inline]
    pub fn bits(&self) -> u32 {
        self.0 .0
    }

    /// Creates a layout from a raw bitmask.
    #[inline]
    pub fn from_bits(bits: u32) -> Self {
        Self(crate::datatypes::UInt32(bits))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_layout_stereo_has_two_channels() {
        assert_eq!(ChannelLayout::STEREO.channel_count(), 2);
    }

    #[test]
    fn channel_layout_5_1_has_six_channels() {
        assert_eq!(ChannelLayout::SURROUND_5_1.channel_count(), 6);
    }

    #[test]
    fn channel_layout_7_1_has_eight_channels() {
        assert_eq!(ChannelLayout::SURROUND_7_1.channel_count(), 8);
    }

    #[test]
    fn channel_layout_mono_has_centre() {
        assert!(ChannelLayout::MONO.has_front_centre());
        assert!(!ChannelLayout::MONO.has_front_left());
    }

    #[test]
    fn channel_layout_bitmask_roundtrip() {
        let bits = ChannelLayout::SURROUND_5_1.bits();
        let restored = ChannelLayout::from_bits(bits);
        assert_eq!(restored.bits(), bits);
    }
}
