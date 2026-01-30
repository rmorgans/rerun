use super::SampleFormat;

impl SampleFormat {
    /// Returns the number of bytes per sample for this format.
    #[inline]
    pub fn bytes_per_sample(self) -> u32 {
        match self {
            Self::U8 | Self::S8 => 1,
            Self::U16 | Self::S16 => 2,
            Self::U24 | Self::S24 => 3,
            Self::U32 | Self::S32 | Self::F32 => 4,
            Self::F64 => 8,
        }
    }

    /// Returns the number of bits per sample for this format.
    #[inline]
    pub fn bits_per_sample(self) -> u32 {
        self.bytes_per_sample() * 8
    }

    /// Returns true if this is a signed integer format.
    #[inline]
    pub fn is_signed_int(self) -> bool {
        matches!(self, Self::S8 | Self::S16 | Self::S24 | Self::S32)
    }

    /// Returns true if this is an unsigned integer format.
    #[inline]
    pub fn is_unsigned_int(self) -> bool {
        matches!(self, Self::U8 | Self::U16 | Self::U24 | Self::U32)
    }

    /// Returns true if this is a floating point format.
    #[inline]
    pub fn is_float(self) -> bool {
        matches!(self, Self::F32 | Self::F64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_format_bytes_per_sample() {
        assert_eq!(SampleFormat::S16.bytes_per_sample(), 2);
        assert_eq!(SampleFormat::S24.bytes_per_sample(), 3);
        assert_eq!(SampleFormat::F32.bytes_per_sample(), 4);
        assert_eq!(SampleFormat::F64.bytes_per_sample(), 8);
    }

    #[test]
    fn sample_format_bits_per_sample() {
        assert_eq!(SampleFormat::S16.bits_per_sample(), 16);
        assert_eq!(SampleFormat::S24.bits_per_sample(), 24);
        assert_eq!(SampleFormat::F32.bits_per_sample(), 32);
    }

    #[test]
    fn sample_format_type_checks() {
        assert!(SampleFormat::S16.is_signed_int());
        assert!(!SampleFormat::S16.is_float());

        assert!(SampleFormat::U16.is_unsigned_int());
        assert!(!SampleFormat::U16.is_signed_int());

        assert!(SampleFormat::F32.is_float());
        assert!(!SampleFormat::F32.is_signed_int());
    }
}
