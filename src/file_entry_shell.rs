use byteorder::{ReadBytesExt, LittleEndian};
use rwinstructs::timestamp::{DosDateTime};
use errors::{ShellItemError};
use std::io::Read;
use std::fmt;
use serde::{ser};
use shellitem::{ClassType};
use extension_blocks::{ExtensionBlock};
use utils;

pub static mut FLAGS_AS_INT: bool = false;

bitflags! {
    pub struct FileAttributeFlags: u16 {
        const FILE_ATTRIBUTE_READONLY               = 0x0001;
        const FILE_ATTRIBUTE_HIDDEN                 = 0x0002;
        const FILE_ATTRIBUTE_SYSTEM                 = 0x0004;
        const FILE_ATTRIBUTE_VOLUME                 = 0x0008;
        const FILE_ATTRIBUTE_DIRECTORY              = 0x0010;
        const FILE_ATTRIBUTE_ARCHIVE                = 0x0020;
        const FILE_ATTRIBUTE_DEVICE                 = 0x0040;
        const FILE_ATTRIBUTE_NORMAL                 = 0x0080;
        const FILE_ATTRIBUTE_TEMPORARY              = 0x0100;
        const FILE_ATTRIBUTE_SPARSE_FILE            = 0x0200;
        const FILE_ATTRIBUTE_REPARSE_POINT          = 0x0400;
        const FILE_ATTRIBUTE_COMPRESSED             = 0x0800;
        const FILE_ATTRIBUTE_OFFLINE                = 0x1000;
        const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED    = 0x2000;
        const FILE_ATTRIBUTE_ENCRYPTED              = 0x4000;
        const FILE_ATTRIBUTE_INTEGRITY_STREAM       = 0x8000;
    }
}
impl fmt::Display for FileAttributeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.bits())
    }
}
impl ser::Serialize for FileAttributeFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        if unsafe{FLAGS_AS_INT} {
            serializer.serialize_u16(self.bits())
        } else {
            serializer.serialize_str(&format!("{:?}", self))
        }
    }
}

bitflags! {
    pub struct FileEntryItemFlags: u8 {
        const DIRECTORY            = 0x01;
        const FILE                 = 0x02;
        const IS_UNICODE           = 0x04;
        const UNKOWN1              = 0x08;
    }
}
impl fmt::Display for FileEntryItemFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.bits())
    }
}
impl ser::Serialize for FileEntryItemFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        if unsafe{FLAGS_AS_INT} {
            serializer.serialize_u8(self.bits())
        } else {
            serializer.serialize_str(&format!("{:?}", self))
        }
    }
}


//https://github.com/libyal/libfwsi/blob/master/documentation/Windows%20Shell%20Item%20format.asciidoc#34-file-entry-shell-item
#[derive(Serialize, Clone, Debug)]
pub struct FileEntryShellItem {
    pub sub_flags: FileEntryItemFlags,
    pub file_size: u32,
    pub last_modification: DosDateTime,
    pub flags: FileAttributeFlags,
    pub name: String,
    pub extention_block: ExtensionBlock
}
impl FileEntryShellItem {
    pub fn new<R: Read>(mut reader: R, class_type: &ClassType) -> Result<FileEntryShellItem,ShellItemError> {
        let sub_flags = FileEntryItemFlags::from_bits_truncate(
            class_type.get_minor()
        );
        let file_size = reader.read_u32::<LittleEndian>()?;
        let last_modification = DosDateTime(
            reader.read_u32::<LittleEndian>()?
        );
        let flags = FileAttributeFlags::from_bits_truncate(
            reader.read_u16::<LittleEndian>()?
        );

        // Get name
        let mut name = String::new();
        if sub_flags.contains(IS_UNICODE) {
            // unicode
            name = utils::read_string_u16_till_null(
                &mut reader
            )?;
        } else {
            name = utils::read_string_u8_till_null(
                &mut reader
            )?;
            // Add 1 to name length to account for null byte
            if (name.len() + 1) % 2 > 0 {
                // throw away align byte
                reader.read_u8()?;
            }
        }

        // Get extention block
        let extention_block = ExtensionBlock::new(
            &mut reader
        )?;

        Ok(
            FileEntryShellItem {
                sub_flags: sub_flags,
                file_size: file_size,
                last_modification: last_modification,
                flags: flags,
                name: name,
                extention_block: extention_block
            }
        )
    }
}

#[test]
fn test_file_entry_item() {
    let buffer: &[u8] = &[
        0xFA,0x0A,0x01,0x00,0x68,0x40,0x6E,0xB1,0x20,0x00,0x43,0x4F,0x50,0x59,0x4F,0x46,
        0x7E,0x31,0x2E,0x58,0x4C,0x53,0x00,0x00,0x64,0x00,0x03,0x00,0x04,0x00,0xEF,0xBE,
        0x68,0x40,0x6E,0xB1,0x70,0x40,0x70,0xA0,0x14,0x00,0x00,0x00,0x43,0x00,0x6F,0x00,
        0x70,0x00,0x79,0x00,0x20,0x00,0x6F,0x00,0x66,0x00,0x20,0x00,0x4D,0x00,0x65,0x00,
        0x74,0x00,0x61,0x00,0x6C,0x00,0x20,0x00,0x41,0x00,0x6C,0x00,0x6C,0x00,0x6F,0x00,
        0x79,0x00,0x20,0x00,0x4C,0x00,0x69,0x00,0x73,0x00,0x74,0x00,0x20,0x00,0x52,0x00,
        0x65,0x00,0x73,0x00,0x65,0x00,0x61,0x00,0x72,0x00,0x63,0x00,0x68,0x00,0x2E,0x00,
        0x78,0x00,0x6C,0x00,0x73,0x00,0x78,0x00,0x00,0x00,0x1C,0x00
    ];

    let file_entry = FileEntryShellItem::new(
        buffer,
        &ClassType::new(0x32)
    ).unwrap();
    assert_eq!(file_entry.file_size,68346);
}
