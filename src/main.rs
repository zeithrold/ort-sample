use anyhow::Result;
use ort_sample::{log, runtime, gui};

fn main() -> Result<()> {
    log::init_log()?;
    let result = runtime::check_provider()?;
    gui::show_dialog(result.to_message())?;
    let result = runtime::run()?;
    gui::show_dialog(result)?;
    Ok(())
}
