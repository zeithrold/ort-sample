use native_dialog::{MessageDialog, MessageType};
use anyhow::Result;

pub fn show_dialog(s: String) -> Result<()> {
    MessageDialog::new()
        .set_title("Result of Execution Provider Detect")
        .set_text(&format!("{:#?}", s))
        .set_type(MessageType::Info)
        .show_alert()?;
    Ok(())
}
