use std::path::Path;
use tokio::net::UnixListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::{config::Config, model::Model};
use std::error::Error;
use onnxruntime::{GraphOptimizationLevel, LoggingLevel, environment::Environment, tensor::{OrtTensor}, session::{SessionBuilder, Session}, OrtError};
use tracing::{info, error};
use ndarray; // Add ndarray import

pub struct Server {
    listener: UnixListener,
    model: Model,
    config: Config,
}

impl Server {
    pub async fn new(config: Config) -> Result<Self, Box<dyn Error>> {
        // Remove old socket file if it exists
        if Path::new(&config.socket_path).exists() {
            std::fs::remove_file(&config.socket_path)?;
        }

        let listener = UnixListener::bind(&config.socket_path)?;
        let model = Model::new(config.clone())?;

        Ok(Self {
            listener,
            model,
            config,
        })
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let (mut stream, _) = self.listener.accept().await?;
            let model = &self.model;
            let config = self.config.clone();

            tokio::spawn(async move {
                let mut buf = vec![0; 1024];
                match stream.read(&mut buf).await {
                    Ok(n) if n > 0 => {
                        let request = String::from_utf8_lossy(&buf[..n]);
                        // Dummy input for now
                        let input_data: Vec<f32> = vec![0.0; 1];

                        // Use the public getter for the environment
                        let mut session_builder = model.get_env().new_session_builder().unwrap();
                        session_builder.with_optimization_level(GraphOptimizationLevel::Basic).unwrap();
                        let model_path = config.model_path.clone();
                        let mut session = session_builder.with_model_from_file(model_path).unwrap();

                        let input_shape = ndarray::IxDyn(&[1, input_data.len()]); // Assuming ndarray is used or needed
                        // NOTE: Assuming OrtTensor::from_array exists and takes env, shape, data
                        // We need access to model.env here, but it's private. This needs fixing later.
                        // For now, let's assume we can get the env somehow or tensor creation doesn't need it directly.
                        // Placeholder: let env = model.get_env(); // Hypothetical getter
                        // let input_tensor = OrtTensor::from_array(env.clone(), input_shape, &input_data)?; // Using reference
                        // Temporary workaround: Create tensor without env if possible, or handle error
                        // This part will likely fail until private access is resolved.
                        // Let's comment out the problematic lines for now to see other errors.
                        // let input_tensor = OrtTensor::from_array(/* ??? */)?;

                        // NOTE: The way inputs are passed to session.run might also differ in 0.0.14
                        // Commenting out run call as input_tensor is unavailable
                        // let outputs_result = session.run(vec![input_tensor]);
                        let outputs_result: Result<Vec<onnxruntime::tensor::OrtOwnedTensor<f32, ndarray::Dim<ndarray::IxDynImpl>>>, OrtError> = Err(OrtError::Session); // Placeholder Error (Corrected)


                        let outputs = match outputs_result {
                            Ok(outputs) => {
                                info!("Prediction successful");
                                outputs
                            }
                            Err(e) => {
                                error!("Failed to run prediction: {:?}", e);
                                let _ = stream.write_all(format!("Error: {:?}", e).as_bytes()).await;
                                return;
                            }
                        };
                        let output: Vec<f32> = outputs[0].try_into().unwrap();

                        let response = format!("Prediction: {:?}", output);
                        if let Err(e) = stream.write_all(response.as_bytes()).await {
                            eprintln!("Failed to send response: {}", e);
                        }
                    }
                    Ok(_) => println!("Client disconnected"),
                    Err(e) => eprintln!("Error reading from stream: {}", e),
                }
            });
        }
    }
}