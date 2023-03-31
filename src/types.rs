use std::{
    error::Error,
    fmt::{Debug, Display},
    str::FromStr,
};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Guid(pub [u8; 16]);

impl Guid {
    /// Create new instance of Guid struct from a [0u8; 16] byte array
    pub fn new(guid: [u8; 16]) -> Self {
        Guid(guid)
    }

    /// Create new instance of Guid struct from 4 u32 values
    pub fn from_4_ints(a: u32, b: u32, c: u32, d: u32) -> Self {
        Guid([
            (a & 0xff) as u8,
            ((a >> 8) & 0xff) as u8,
            ((a >> 16) & 0xff) as u8,
            ((a >> 24) & 0xff) as u8,
            (b & 0xff) as u8,
            ((b >> 8) & 0xff) as u8,
            ((b >> 16) & 0xff) as u8,
            ((b >> 24) & 0xff) as u8,
            (c & 0xff) as u8,
            ((c >> 8) & 0xff) as u8,
            ((c >> 16) & 0xff) as u8,
            ((c >> 24) & 0xff) as u8,
            (d & 0xff) as u8,
            ((d >> 8) & 0xff) as u8,
            ((d >> 16) & 0xff) as u8,
            ((d >> 24) & 0xff) as u8,
        ])
    }

    /// Convert Guid struct into 4 u32 values
    pub fn into_4_ints(&self) -> (u32, u32, u32, u32) {
        let a = self.0[0] as u32
            | ((self.0[1] as u32) << 8)
            | ((self.0[2] as u32) << 16)
            | ((self.0[3] as u32) << 24);
        let b = self.0[4] as u32
            | ((self.0[5] as u32) << 8)
            | ((self.0[6] as u32) << 16)
            | ((self.0[7] as u32) << 24);
        let c = self.0[8] as u32
            | ((self.0[9] as u32) << 8)
            | ((self.0[10] as u32) << 16)
            | ((self.0[11] as u32) << 24);
        let d = self.0[12] as u32
            | ((self.0[13] as u32) << 8)
            | ((self.0[14] as u32) << 16)
            | ((self.0[15] as u32) << 24);

        (a, b, c, d)
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
        if self.0.iter().all(|&x| x == 0) {
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
