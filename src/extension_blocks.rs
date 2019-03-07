use byteorder::{ReadBytesExt, LittleEndian};
use serde::{ser};
use errors::{ShellItemError};
use rwinstructs::timestamp::DosDateTime;
use rwinstructs::reference::MftReference;
use utils;
use std::io::Cursor;
use std::io::Read;
use std::io::{Seek,SeekFrom};
use std::fmt;

#[derive(Serialize, Clone, Debug)]
pub struct Beef0004 {
    #[serde(skip_serializing)]
    _offset: u64,
    creation: DosDateTime,
    last_access: DosDateTime,
    identifier: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_reference: Option<MftReference>,
    #[serde(skip_serializing)]
    long_string_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    long_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    localized_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_offset: Option<u16>
}
impl Beef0004 {
    pub fn new<Rs: Read+Seek>(mut reader: Rs, extention_version: u16) -> Result<Beef0004, ShellItemError> {
        let _offset = reader.seek(SeekFrom::Current(0))?;
        let creation = DosDateTime(reader.read_u32::<LittleEndian>()?);
        let last_access = DosDateTime(reader.read_u32::<LittleEndian>()?);
        let identifier = reader.read_u16::<LittleEndian>()?;
        let mut file_reference = None;
        let mut long_string_size = None;
        let mut name = None;
        let mut long_name = None;
        let mut localized_name = None;
        let mut version_offset = None;

        match extention_version {
            3 => {
                long_string_size = Some(reader.read_u16::<LittleEndian>()?);
                name = Some(
                    utils::read_string_u16_till_null(&mut reader)?
                );
                if long_string_size.unwrap() > 0 {
                    long_name = Some(
                        utils::read_string_u16_till_null(&mut reader)?
                    );
                }

                version_offset = Some(
                    reader.read_u16::<LittleEndian>()?
                );
            },
            8 => {
                let _unknown1 = reader.read_u16::<LittleEndian>()?;
                file_reference = Some(
                    MftReference(reader.read_u64::<LittleEndian>()?)
                );
                let _unknown2 = reader.read_u64::<LittleEndian>()?;
                long_string_size = Some(reader.read_u16::<LittleEndian>()?);
                let _unknown3 = reader.read_u32::<LittleEndian>()?;

                name = Some(
                    utils::read_string_u16_till_null(&mut reader)?
                );

                if long_string_size.unwrap() > 0 {
                    long_name = Some(
                        utils::read_string_u16_till_null(&mut reader)?
                    );
                }

                version_offset = Some(
                    reader.read_u16::<LittleEndian>()?
                );
            },
            9 => {
                let _unknown1 = reader.read_u16::<LittleEndian>()?;
                file_reference = Some(
                    MftReference(reader.read_u64::<LittleEndian>()?)
                );
                let _unknown2 = reader.read_u64::<LittleEndian>()?;
                long_string_size = Some(reader.read_u16::<LittleEndian>()?);
                let _unknown3 = reader.read_u32::<LittleEndian>()?;
                let _unknown4 = reader.read_u32::<LittleEndian>()?;

                name = Some(
                    utils::read_string_u16_till_null(&mut reader)?
                );

                if long_string_size.unwrap() > 0 {
                    long_name = Some(
                        utils::read_string_u16_till_null(&mut reader)?
                    );
                }

                version_offset = Some(
                    reader.read_u16::<LittleEndian>()?
                );
            },
            _ => {
                panic!(
                    format!("Unhandled extention_version {} for Beef0004",extention_version)
                );
            }
        }

        Ok(
            Beef0004 {
                _offset: _offset,
                creation: creation,
                last_access: last_access,
                identifier: identifier,
                file_reference: file_reference,
                long_string_size: long_string_size,
                name: name,
                long_name: long_name,
                localized_name: localized_name,
                version_offset: version_offset
            }
        )
    }
}

// Raw Content will be used for unhandled shell item data
#[derive(Clone)]
pub struct RawExtensionContent(
    pub Vec<u8>
);
impl fmt::Debug for RawExtensionContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", utils::to_hex_string(&self.0))
    }
}
impl ser::Serialize for RawExtensionContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        serializer.serialize_str(
            &format!("{}",
            utils::to_hex_string(&self.0))
        )
    }
}

#[derive(Clone)]
pub struct ExtensionSignature(
    u32
);
impl ExtensionSignature{
    pub fn new(value: u32) -> ExtensionSignature {
        ExtensionSignature(value)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}
impl fmt::Display for ExtensionSignature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"0x{:08X}",self.0)
    }
}
impl fmt::Debug for ExtensionSignature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"0x{:08X}",self.0)
    }
}
impl ser::Serialize for ExtensionSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        serializer.serialize_str(
            &format!("{}", self)
        )
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ExtensionContent {
    FileEntry(Beef0004),
    Raw(RawExtensionContent),
    None
}

#[derive(Serialize, Clone, Debug)]
pub struct ExtensionHeader {
    #[serde(skip_serializing)]
    _offset: u64,
    version: u16,
    signature: ExtensionSignature
}
impl ExtensionHeader {
    pub fn new<Rs: Read+Seek>(mut reader: Rs) -> Result<ExtensionHeader, ShellItemError> {
        let _offset = reader.seek(SeekFrom::Current(0))?;
        let version = reader.read_u16::<LittleEndian>()?;
        let signature = ExtensionSignature(reader.read_u32::<LittleEndian>()?);

        Ok(
            ExtensionHeader {
                _offset: _offset,
                version: version,
                signature: signature
            }
        )
    }

    pub fn get_signature_u32(&self) -> u32 {
        self.signature.as_u32()
    }

    pub fn get_version_u32(&self) -> u16 {
        self.version
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct ExtensionBlock {
    #[serde(skip_serializing)]
    _offset: u64,
    #[serde(skip_serializing)]
    size: u16,
    header: Option<ExtensionHeader>,
    content: Option<ExtensionContent>
}
impl ExtensionBlock {
    pub fn new<Rs: Read+Seek>(mut reader: Rs) -> Result<ExtensionBlock, ShellItemError> {
        let _offset = reader.seek(SeekFrom::Current(0))?;
        let size = reader.read_u16::<LittleEndian>()?;

        let mut header_opt = None;
        let mut content = None;

        if size > 0 {
            header_opt = Some(
                ExtensionHeader::new(&mut reader)?
            );
        }

        match header_opt {
            Some(ref header) => {
                // // subtarct size of 8 for header
                // let mut buffer = vec![0; (size - 8) as usize];
                // reader.read_exact(&mut buffer)?;

                match header.get_signature_u32() {
                    0xBEEF0004 => {
                        content = Some(
                            ExtensionContent::FileEntry(
                                Beef0004::new(
                                    // Cursor::new(buffer),
                                    &mut reader,
                                    header.get_version_u32()
                                )?
                            )
                        );
                    }
                    _ => {
                        let mut buffer = vec![0; (size - 8) as usize];
                        reader.read_exact(&mut buffer)?;
                        content = Some(
                            ExtensionContent::Raw(
                                RawExtensionContent(buffer)
                            )
                        );
                    }
                }
            },
            None => {}
        }

        Ok(
            ExtensionBlock {
                _offset: _offset,
                size: size,
                header: header_opt,
                content: content
            }
        )
    }

    pub fn get_size(&self) -> u16 {
        self.size
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct ExtensionList(
    Vec<ExtensionBlock>
);

impl ExtensionList {
    pub fn new<Rs: Read+Seek>(mut reader: Rs) -> Result<ExtensionList, ShellItemError> {
        let mut extension_blocks: Vec<ExtensionBlock> = Vec::new();
        loop {
            let extension_block = ExtensionBlock::new(
                &mut reader
            )?;
            let size = extension_block.get_size();

            if size == 0 {
                // Null shell item is terminator
                break
            }

            extension_blocks.push(
                extension_block
            );

            break
        }

        Ok(
            ExtensionList(extension_blocks)
        )
    }
}
