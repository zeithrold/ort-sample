use log::{info, warn};
use ndarray::array;
use ort::{
    execution_providers::{
        CPUExecutionProvider, CUDAExecutionProvider, CoreMLExecutionProvider,
        DirectMLExecutionProvider, ExecutionProvider,
    },
    session::{
        builder::{GraphOptimizationLevel, SessionBuilder}, Session
    },
};


pub fn create_session_builder() -> anyhow::Result<SessionBuilder> {
    Ok(Session::builder()?
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_intra_threads(4)?)
}

pub fn create_session(provider: Box<dyn ExecutionProvider>) -> anyhow::Result<Session> {
    info!("Creating Session...");
    let mut session = create_session_builder()?;
    provider.register(&mut session)?;
    info!("Session is created");
    let session = session.commit_from_file("model.onnx")?;
    Ok(session)
}

pub fn create_test_ndarray() -> ndarray::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::Dim<[usize; 2]>>
{
    array![[1.0f32, 2.0f32], [2.0f32, 3.0f32]]
}

pub fn session_test_run(session: Session) -> anyhow::Result<()> {
    info!("Running Session...");
    let input = create_test_ndarray();
    let onnx_input = ort::inputs!["input" => input.view()]?;
    info!("Input: {:?}", input);
    let output = session.run(onnx_input)?;
    info!("Output: {:?}", output);
    info!("Done");
    Ok(())
}

pub fn range_providers() -> anyhow::Result<()> {
    let providers: Vec<(Box<dyn ExecutionProvider>, String)> = vec![
        (
            Box::new(CUDAExecutionProvider::default()),
            "CUDA".to_string(),
        ),
        (
            Box::new(DirectMLExecutionProvider::default()),
            "DirectML".to_string(),
        ),
        (
            Box::new(CoreMLExecutionProvider::default()),
            "CoreML".to_string(),
        ),
        (Box::new(CPUExecutionProvider::default()), "CPU".to_string()),
    ];
    for (provider, name) in providers {
        let result = provider.is_available()?;
        if result {
            info!("{} is available", name);
            let session = match create_session(provider) {
                Ok(session) => {session},
                Err(err) => {
                    warn!("Failed to create session even with available provider: {}", name);
                    warn!("{}", err);
                    continue;
                },
            };
            match session_test_run(session) {
                Ok(_) => {},
                Err(err) => {
                    warn!("Failed to run session with provider: {}", name);
                    warn!("{}", err);
                },
            }
        } else {
            info!("{} is not available", name);
        }
    }
    Ok(())
}

pub fn run() -> anyhow::Result<()> {
    range_providers()?;
    Ok(())
}
