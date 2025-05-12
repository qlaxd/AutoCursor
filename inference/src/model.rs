use onnxruntime::{
    environment::Environment,
    session::{Session, SessionBuilder},
    GraphOptimizationLevel, LoggingLevel,
    tensor::{OrtTensor},
};
use crate::config::Config;
use std::sync::Arc;
use onnxruntime::OrtError;
use ndarray; // Add ndarray import

pub struct Model {
    env: Arc<Environment>,
    config: Config
}

unsafe impl Send for Model {}
unsafe impl Sync for Model {}

impl Model {
    pub fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let env = Arc::new(
            Environment::builder()
                .with_name("onnx_inference")
                .with_log_level(LoggingLevel::Warning)
                .build()?
        );

        Ok(Self {
            env,
            config
        })
    }

    // Add public getter for the environment
    pub fn get_env(&self) -> &Arc<Environment> {
        &self.env
    }

    pub fn predict(&self, input: &[f32]) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        let model_path = self.config.model_path.clone();

        // Attempt to create SessionBuilder from Environment
        let mut session_builder = self.env.new_session_builder()?;
        session_builder.with_optimization_level(GraphOptimizationLevel::Basic)?;

        let mut session = session_builder.with_model_from_file(model_path)?;

        let input_shape = ndarray::IxDyn(&[1, input.len()]); // Assuming ndarray is used or needed
        // NOTE: Assuming OrtTensor::from_array exists and takes env, shape, data
        let input_tensor = OrtTensor::from_array(self.env.clone(), input_shape, input)?;

        // NOTE: The way inputs are passed to session.run might also differ in 0.0.14
        let outputs = session.run(vec![input_tensor])?;

        let output: Vec<f32> = outputs
            .get(0)
            .and_then(|output| {
                let tensor = output.try_into_tensor().ok()?;
                let tensor_slice = tensor.as_slice().ok()?;
                Some(tensor_slice.to_vec())
            })
            .ok_or("Failed to get output tensor")?;

        Ok(output)
    }
}