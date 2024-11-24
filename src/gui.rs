use log::warn;
use native_dialog::{MessageDialog, MessageType};
use anyhow::Result;

pub fn show_dialog(s: String) -> Result<()> {
    let dialog = MessageDialog::new()
        .set_title("ORT_SAMPLE")
        .set_text(&format!("{:#?}", s))
        .set_type(MessageType::Info)
        .show_alert();
    if dialog.is_err() {
        warn!("Failed to show dialog");
    }
    Ok(())
}
