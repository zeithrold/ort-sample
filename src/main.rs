use anyhow::Result;
use ort_sample::{log, runtime, gui};

fn main() -> Result<()> {
    log::init_log()?;
    runtime::run()?;
    gui::show_dialog("Successfully run sample program.".into())?;
    // gui::show_dialog(result)?;
    Ok(())
}
