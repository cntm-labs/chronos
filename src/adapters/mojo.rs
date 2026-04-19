use crate::domain::error::ChronosError;
use crate::domain::models::CognitiveNode;
use crate::ports::compute::NeuralComputePort;
use async_trait::async_trait;
use std::process::Command;

pub struct MojoAdapter {
    binary_path: String,
}

impl MojoAdapter {
    pub fn new(binary_path: String) -> Self {
        Self { binary_path }
    }
}

#[async_trait]
impl NeuralComputePort for MojoAdapter {
    async fn predict(&self, _context: Vec<CognitiveNode>) -> Result<String, ChronosError> {
        // Since we are developing on local hardware that doesn't support Mojo,
        // we prepare the call structure for the CI/Production environment.

        let output = Command::new(&self.binary_path).output().map_err(|e| {
            ChronosError::ComputeError(format!("Failed to execute Mojo binary: {}", e))
        })?;

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(result)
        } else {
            let error = String::from_utf8_lossy(&output.stderr).to_string();
            Err(ChronosError::ComputeError(format!(
                "Mojo execution failed: {}",
                error
            )))
        }
    }
}
