use byteorder::{ReadBytesExt, LittleEndian};
use serde::{ser};
use errors::{ShellItemError};
use utils;
use std::io::Cursor;
use std::io::Read;
use std::fmt;

#[derive(Debug)]
pub enum ShellContent {
    Raw(RawContent),
    None
}

#[derive(Debug)]
pub struct ShellData {
    pub class_type: u8,
    pub content: ShellContent
}
impl ShellData {
    pub fn new<R: Read>(mut reader: R) -> Result<ShellData,ShellItemError> {
        let class_type = reader.read_u8()?;
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let content: ShellContent = ShellContent::Raw(
            RawContent(buffer)
        );

        Ok(
            ShellData {
                class_type: class_type,
                content: content
            }
        )
    }
}

#[derive(Debug)]
pub struct ShellItem {
    pub size: u16,
    pub data: Option<ShellData>
}
impl ShellItem {
    pub fn new<R: Read>(mut reader: R) -> Result<ShellItem,ShellItemError> {
        let size = reader.read_u16::<LittleEndian>()?;

        let mut data: Option<ShellData> = None;
        if size > 0 {
            // subtract 2 from size to account for size already read
            let mut buffer = vec![0; (size - 2) as usize];
            reader.read_exact(buffer.as_mut_slice())?;
            data = Some(ShellData::new(Cursor::new(buffer))?);
        }

        Ok(
            ShellItem {
                size: size,
                data: data
            }
        )
    }

    pub fn get_size(&self) -> u16 {
        self.size
    }
}

// Raw Content will be used for unhandled shell item data
#[derive(Clone)]
pub struct RawContent(
    pub Vec<u8>
);
impl fmt::Debug for RawContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", utils::to_hex_string(&self.0))
    }
}
impl ser::Serialize for RawContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        serializer.serialize_str(
            &format!("{}",
            utils::to_hex_string(&self.0))
        )
    }
}
