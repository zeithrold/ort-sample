use anyhow::{Ok, Result};
use log::info;
use ort::{
    CUDAExecutionProvider, CoreMLExecutionProvider, DirectMLExecutionProvider, ExecutionProvider,
};

pub struct ProviderCheckResult {
    cuda: bool,
    directml: bool,
    coreml: bool,
}

impl ProviderCheckResult {
    pub fn to_message(&self) -> String {
        format!(
            "cuda_supported: {}, directml_supported: {}, coreml_supported: {}",
            self.cuda, self.directml, self.coreml
        )
    }
}

pub fn check_provider() -> Result<ProviderCheckResult> {
    info!("Checking Providers...");
    let cuda = CUDAExecutionProvider::default().is_available()?;
    let directml = DirectMLExecutionProvider::default().is_available()?;
    let coreml = CoreMLExecutionProvider::default().is_available()?;
    let result = ProviderCheckResult {
        cuda,
        directml,
        coreml,
    };
    info!("{}", result.to_message());
    Ok(result)
}
