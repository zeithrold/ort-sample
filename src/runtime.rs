use log::{info, warn};
use ort::{
    execution_providers::{
        CPUExecutionProvider, CUDAExecutionProvider, CoreMLExecutionProvider,
        DirectMLExecutionProvider, ExecutionProvider,
    },
    session::{
        builder::{GraphOptimizationLevel, SessionBuilder},
        Session,
    },
};

use ndarray::prelude::*;

pub struct ResultByProvider {
    cuda: bool,
    directml: bool,
    coreml: bool,
    cpu: bool,
}

impl ResultByProvider {
    pub fn to_message(&self) -> String {
        format!(
            "cuda_supported: {}, directml_supported: {}, coreml_supported: {}, cpu_supported: {}",
            self.cuda, self.directml, self.coreml, self.cpu
        )
    }
}

pub fn check_provider() -> anyhow::Result<ResultByProvider> {
    info!("Checking Providers...");
    let cuda = CUDAExecutionProvider::default().is_available()?;
    let directml = DirectMLExecutionProvider::default().is_available()?;
    let coreml = CoreMLExecutionProvider::default().is_available()?;
    let cpu = CPUExecutionProvider::default().is_available()?;
    let result = ResultByProvider {
        cuda,
        directml,
        coreml,
        cpu,
    };
    info!("{}", result.to_message());
    Ok(result)
}

pub fn create_providers(session: &mut SessionBuilder) -> anyhow::Result<()> {
    info!("Creating Providers...");
    let cuda = CUDAExecutionProvider::default();
    if cuda.is_available()? {
        if cuda.register(session).is_err() {
            warn!("Failed to register CUDAExecutionProvider");
        } else {
            info!("CUDAExecutionProvider is registered");
        }
    }
    let directml = DirectMLExecutionProvider::default();
    if directml.is_available()? {
        if directml.register(session).is_err() {
            warn!("Failed to register DirectMLExecutionProvider");
        } else {
            info!("DirectMLExecutionProvider is registered");
        }
    }
    let coreml = CoreMLExecutionProvider::default();
    if coreml.is_available()? {
        if coreml.register(session).is_err() {
            warn!("Failed to register CoreMLExecutionProvider");
        } else {
            info!("CoreMLExecutionProvider is registered");
        }
    }
    let cpu = CPUExecutionProvider::default();
    if cpu.is_available()? {
        if cpu.register(session).is_err() {
            warn!("Failed to register CPUExecutionProvider");
        } else {
            info!("CPUExecutionProvider is registered");
        }
    }
    Ok(())
}

pub fn create_session() -> anyhow::Result<Session> {
    let mut model = Session::builder()?
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_intra_threads(4)?;
    create_providers(&mut model)?;
    Ok(model.commit_from_file("model.onnx")?)
}

pub fn run() -> anyhow::Result<String> {
    info!("Running...");
    info!("Creating Session...");
    let session = create_session()?;

    // Use f32
    let input = array![[1.0f32, 2.0f32], [2.0f32, 3.0f32]];
    let onnx_input = ort::inputs!["input" => input.view()]?;
    info!("Input: {:?}", input);
    info!("Running Session...");
    let output = session.run(onnx_input)?;
    info!("Output: {:?}", output);
    info!("Done");
    Ok(format!("{:?}", output))
}
