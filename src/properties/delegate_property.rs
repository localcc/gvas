use std::io::{Cursor, Read, Seek, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::Error,
};

use super::{PropertyOptions, PropertyTrait};

/// An Unreal script delegate
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Delegate {
    /// The object bound to this delegate
    pub object: String,
    /// Name of the function to call on the bound object
    pub function_name: String,
}

impl Delegate {
    /// Creates a new `Delegate` instance
    #[inline]
    pub fn new(object: String, function_name: String) -> Self {
        Delegate {
            object,
            function_name,
        }
    }

    #[inline]
    pub(crate) fn read<R: Read + Seek>(cursor: &mut R) -> Result<Self, Error> {
        let object = cursor.read_string()?;
        let function_name = cursor.read_string()?;
        Ok(Delegate {
            object,
            function_name,
        })
    }

    #[inline]
    pub(crate) fn write<W: Write>(
        &self,
        cursor: &mut W,
        _options: &mut PropertyOptions,
    ) -> Result<(), Error> {
        cursor.write_string(&self.object)?;
        cursor.write_string(&self.function_name)?;
        Ok(())
    }
}

/// Delegate property
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DelegateProperty {
    /// Delegate
    pub value: Delegate,
}

impl DelegateProperty {
    /// Creates a new `DelegateProperty` instance
    #[inline]
    pub fn new(value: Delegate) -> Self {
        DelegateProperty { value }
    }

    #[inline]
    pub(crate) fn read<R: Read + Seek>(
        cursor: &mut R,
        include_header: bool,
    ) -> Result<Self, Error> {
        if include_header {
            let _length = cursor.read_u64::<LittleEndian>()?;
            let separator = cursor.read_u8()?;
            assert_eq!(separator, 0);
        }
        let value = Delegate::read(cursor)?;
        Ok(DelegateProperty { value })
    }

    #[inline]
    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        options: &mut PropertyOptions,
    ) -> Result<(), Error> {
        self.value.write(cursor, options)?;
        Ok(())
    }
}

impl PropertyTrait for DelegateProperty {
    #[inline]
    fn write<W: Write>(
        &self,
        cursor: &mut W,
        include_header: bool,
        options: &mut PropertyOptions,
    ) -> Result<(), Error> {
        if !include_header {
            return self.write_body(cursor, options);
        }

        let buf = &mut Cursor::new(Vec::new());
        self.write_body(buf, options)?;
        let buf = buf.get_ref();

        cursor.write_string("DelegateProperty")?;
        cursor.write_u64::<LittleEndian>(buf.len() as u64)?;
        cursor.write_u8(0)?;
        cursor.write_all(buf)?;

        Ok(())
    }
}

/// Multicast script delegate
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MulticastScriptDelegate {
    /// Delegates
    pub delegates: Vec<Delegate>,
}

impl MulticastScriptDelegate {
    /// Creates a new `MulticastScriptDelegate` instance
    #[inline]
    pub fn new(delegates: Vec<Delegate>) -> Self {
        MulticastScriptDelegate { delegates }
    }

    #[inline]
    pub(crate) fn read<R: Read + Seek>(cursor: &mut R) -> Result<Self, Error> {
        let delegates_len = cursor.read_u32::<LittleEndian>()?;
        let mut delegates = Vec::with_capacity(delegates_len as usize);
        for _ in 0..delegates_len {
            delegates.push(Delegate::read(cursor)?);
        }

        Ok(MulticastScriptDelegate { delegates })
    }

    #[inline]
    pub(crate) fn write<W: Write>(
        &self,
        cursor: &mut W,
        options: &mut PropertyOptions,
    ) -> Result<(), Error> {
        cursor.write_u32::<LittleEndian>(self.delegates.len() as u32)?;

        for delegate in &self.delegates {
            delegate.write(cursor, options)?;
        }

        Ok(())
    }
}

/// Multicast inline delegate property
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MulticastInlineDelegateProperty {
    /// Delegate
    pub value: MulticastScriptDelegate,
}

impl MulticastInlineDelegateProperty {
    /// Creates a new `MulticastInlineDelegateProperty` instance
    #[inline]
    pub fn new(value: MulticastScriptDelegate) -> Self {
        MulticastInlineDelegateProperty { value }
    }

    #[inline]
    pub(crate) fn read<R: Read + Seek>(
        cursor: &mut R,
        include_header: bool,
    ) -> Result<Self, Error> {
        if include_header {
            let _length = cursor.read_u64::<LittleEndian>()?;
            let separator = cursor.read_u8()?;
            assert_eq!(separator, 0);
        }

        let value = MulticastScriptDelegate::read(cursor)?;

        Ok(MulticastInlineDelegateProperty { value })
    }

    #[inline]
    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        options: &mut PropertyOptions,
    ) -> Result<(), Error> {
        self.value.write(cursor, options)?;
        Ok(())
    }
}

impl PropertyTrait for MulticastInlineDelegateProperty {
    #[inline]
    fn write<W: Write>(
        &self,
        cursor: &mut W,
        include_header: bool,
        options: &mut PropertyOptions,
    ) -> Result<(), Error> {
        if !include_header {
            return self.write_body(cursor, options);
        }

        let buf = &mut Cursor::new(Vec::new());
        self.write_body(buf, options)?;
        let buf = buf.get_ref();

        cursor.write_string("MulticastInlineDelegateProperty")?;
        cursor.write_u64::<LittleEndian>(buf.len() as u64)?;
        cursor.write_u8(0)?;
        cursor.write_all(buf)?;

        Ok(())
    }
}

/// Multicast sparse delegate property
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MulticastSparseDelegateProperty {
    /// Delegate
    pub value: MulticastScriptDelegate,
}

impl MulticastSparseDelegateProperty {
    /// Creates a new `MulticastSparseDelegateProperty` instance
    #[inline]
    pub fn new(value: MulticastScriptDelegate) -> Self {
        MulticastSparseDelegateProperty { value }
    }

    #[inline]
    pub(crate) fn read<R: Read + Seek>(
        cursor: &mut R,
        include_header: bool,
    ) -> Result<Self, Error> {
        if include_header {
            let _length = cursor.read_u64::<LittleEndian>()?;
            let separator = cursor.read_u8()?;
            assert_eq!(separator, 0);
        }

        let value = MulticastScriptDelegate::read(cursor)?;

        Ok(MulticastSparseDelegateProperty { value })
    }

    #[inline]
    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        options: &mut PropertyOptions,
    ) -> Result<(), Error> {
        self.value.write(cursor, options)?;
        Ok(())
    }
}

impl PropertyTrait for MulticastSparseDelegateProperty {
    #[inline]
    fn write<W: Write>(
        &self,
        cursor: &mut W,
        include_header: bool,
        options: &mut PropertyOptions,
    ) -> Result<(), Error> {
        if !include_header {
            return self.write_body(cursor, options);
        }

        let buf = &mut Cursor::new(Vec::new());
        self.write_body(buf, options)?;
        let buf = buf.get_ref();

        cursor.write_string("MulticastSparseDelegateProperty")?;
        cursor.write_u64::<LittleEndian>(buf.len() as u64)?;
        cursor.write_u8(0)?;
        cursor.write_all(buf)?;

        Ok(())
    }
}
