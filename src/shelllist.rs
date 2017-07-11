use shellitem::{ShellItem};
use errors::{ShellItemError};
use std::io::Read;

#[derive(Serialize, Clone, Debug)]
pub struct ShellList(
    Vec<ShellItem>
);

impl ShellList {
    pub fn new<R: Read>(mut reader: R) -> Result<ShellList, ShellItemError> {
        let mut shell_items: Vec<ShellItem> = Vec::new();
        loop {
            let shell_item = ShellItem::new(&mut reader)?;
            let size = shell_item.get_size();

            if size == 0 {
                // Null shell item is terminator
                break
            }

            shell_items.push(shell_item);
        }

        Ok(
            ShellList(shell_items)
        )
    }
}
