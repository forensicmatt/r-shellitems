use byteorder::{ReadBytesExt, LittleEndian};
use serde::{ser};
use errors::{ShellItemError};
use file_entry_shell::{FileEntryShellItem};
use utils;
use std::io::Cursor;
use std::io::Read;
use std::fmt;

#[derive(Serialize, Clone, Debug)]
pub enum ShellContent {
    Raw(RawContent),
    FileEntry(FileEntryShellItem),
    None
}

#[derive(Clone)]
pub struct ClassType(
    u8
);
impl ClassType{
    pub fn new(value: u8) -> ClassType {
        ClassType(value)
    }

    pub fn get_major(&self) -> u8 {
        self.0 & 0xF0
    }

    pub fn get_minor(&self) -> u8 {
        self.0 & 0x0F
    }

    pub fn get_type(&self) -> u8 {
        self.0
    }
}
impl fmt::Display for ClassType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"0x{:02X}",self.0)
    }
}
impl fmt::Debug for ClassType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"0x{:02X}",self.0)
    }
}
impl ser::Serialize for ClassType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        serializer.serialize_str(
            &format!("{}", self)
        )
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct ShellData {
    pub class_type: ClassType,
    pub unknown: u8,
    pub content: ShellContent
}
impl ShellData {
    pub fn new<R: Read>(mut reader: R) -> Result<ShellData,ShellItemError> {
        let class_type = ClassType(reader.read_u8()?);
        let unknown = reader.read_u8()?;

        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let mut content = ShellContent::None;

        match class_type.get_type() {
            0x30...0x3F => {
                content = ShellContent::FileEntry(
                    FileEntryShellItem::new(
                        Cursor::new(buffer),
                        &class_type
                    )?
                );
            },
            _ => {
                content = ShellContent::Raw(
                    RawContent(buffer)
                );
            }
        }

        Ok(
            ShellData {
                class_type: class_type,
                unknown: unknown,
                content: content
            }
        )
    }
}

#[derive(Serialize, Clone, Debug)]
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
            data = Some(
                ShellData::new(Cursor::new(buffer))?
            );
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
