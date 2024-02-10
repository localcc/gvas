use std::{
    error::Error,
    fmt::{Debug, Display},
    str::FromStr,
};

/// Stores a 128-bit guid (globally unique identifier)
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct Guid(pub [u8; 16]);

impl Guid {
    /// Create a guid from an array of sixteen u8s
    #[inline]
    pub const fn from_u8(value: [u8; 16]) -> Self {
        Guid(value)
    }

    /// Create a guid from an array of four u32s
    #[inline]
    pub const fn from_u32(value: [u32; 4]) -> Self {
        Guid(transmute_4u32_16u8(value))
    }

    /// Create a guid from a u128
    #[inline]
    pub const fn from_u128(value: u128) -> Self {
        Guid(u128::to_le_bytes(value))
    }

    /// Returns true if the guid is zero.
    #[inline]
    pub const fn is_zero(&self) -> bool {
        Guid::to_u128(self) == 0
    }

    /// Create an array of sixteen u8s from a guid
    #[inline]
    pub const fn to_u8(&self) -> [u8; 16] {
        self.0
    }

    /// Create an array of four u32s from a guid
    #[inline]
    pub const fn to_u32(&self) -> [u32; 4] {
        transmute_16u8_4u32(self.0)
    }

    /// Create a u128 from a guid
    #[inline]
    pub const fn to_u128(&self) -> u128 {
        u128::from_le_bytes(self.0)
    }
}

#[inline]
const fn transmute_4u32_16u8(value: [u32; 4]) -> [u8; 16] {
    unsafe { std::mem::transmute(value) }
}

#[inline]
const fn transmute_16u8_4u32(src: [u8; 16]) -> [u32; 4] {
    unsafe { std::mem::transmute(src) }
}

impl From<[u32; 4]> for Guid {
    #[inline]
    fn from(value: [u32; 4]) -> Self {
        Self::from_u32(value)
    }
}

impl From<Guid> for [u32; 4] {
    #[inline]
    fn from(value: Guid) -> Self {
        Guid::to_u32(&value)
    }
}

impl From<u128> for Guid {
    #[inline]
    fn from(value: u128) -> Self {
        Self::from_u128(value)
    }
}

impl From<Guid> for u128 {
    #[inline]
    fn from(value: Guid) -> Self {
        Guid::to_u128(&value)
    }
}

impl Debug for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let guid = self.to_string();
        write!(f, "Guid({})", &guid)
    }
}

impl Display for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_zero() {
            write!(f, "0")?;
            return Ok(());
        }

        write!(f, "{:02X}", self.0[0])?;
        write!(f, "{:02X}", self.0[1])?;
        write!(f, "{:02X}", self.0[2])?;
        write!(f, "{:02X}", self.0[3])?;

        write!(f, "-")?;

        write!(f, "{:02X}", self.0[4])?;
        write!(f, "{:02X}", self.0[5])?;

        write!(f, "-")?;

        write!(f, "{:02X}", self.0[6])?;
        write!(f, "{:02X}", self.0[7])?;

        write!(f, "-")?;

        write!(f, "{:02X}", self.0[8])?;
        write!(f, "{:02X}", self.0[9])?;

        write!(f, "-")?;

        write!(f, "{:02X}", self.0[10])?;
        write!(f, "{:02X}", self.0[11])?;
        write!(f, "{:02X}", self.0[12])?;
        write!(f, "{:02X}", self.0[13])?;
        write!(f, "{:02X}", self.0[14])?;
        write!(f, "{:02X}", self.0[15])?;
        Ok(())
    }
}

/// An error ocurred while parsing a Guid
#[derive(Debug)]
pub struct ParseGuidError;

impl Display for ParseGuidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid GUID syntax")
    }
}

impl Error for ParseGuidError {}

impl FromStr for Guid {
    type Err = ParseGuidError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cleaned = s.replace('-', "");
        let cleaned = cleaned.trim();
        let cleaned = cleaned.strip_prefix('{').unwrap_or(cleaned);
        let cleaned = cleaned.strip_suffix('}').unwrap_or(cleaned);

        if cleaned.len() == 1 && cleaned == "0" {
            return Ok(Guid([0u8; 16]));
        }

        if cleaned.len() != 32 {
            Err(ParseGuidError)?;
        }
        let mut guid = Guid(Default::default());
        for i in 0..16 {
            guid.0[i] =
                u8::from_str_radix(&cleaned[i * 2..i * 2 + 2], 16).map_err(|_| ParseGuidError)?;
        }
        Ok(guid)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Guid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Guid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Guid::from_str(&s).map_err(serde::de::Error::custom)
    }
}
