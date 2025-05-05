# AI Autocorrect Development Plan

## Overview
This project aims to develop an on-premise, GPU-accelerated, context-aware AI autocorrect/autocomplete system for Linux, focusing on low latency and user privacy by running entirely locally. The primary audience includes power users, developers, and privacy-conscious writers. The MVP targets sentence-level next-token prediction integrated into CLI/editors like Neovim/Vim, using a Rust-based inference engine with CUDA support.

## 1. Project Setup
- [ ] Initialize Git repository with standard files (`README.md`, `LICENSE`, `.gitignore`).
- [ ] Define and create the project directory structure:
  ```
  ├── python/         # Model training & export scripts
  ├── rust/           # Inference daemon source code
  │   └── src/
  ├── plugins/        # Editor/CLI integration scripts
  ├── models/         # Default location for downloaded/exported models (add to .gitignore)
  └── docs/           # Documentation (PRD, Plan, TechSpec, etc.)
  ```
- [ ] Set up Python development environment (`venv` or `conda`).
  - [ ] Create `python/requirements.txt` (e.g., `transformers`, `torch`, `onnx`, `onnxruntime`).
  - [ ] Install Python dependencies.
- [ ] Initialize Rust project for the inference daemon (`cargo init --bin rust/inference_daemon`).
  - [ ] Define initial dependencies in `rust/Cargo.toml` (`onnxruntime` with `cuda` feature, `tokio`, `serde`, `toml`, `async-ipc`).
- [ ] Define initial structure for configuration file (`docs/config.example.toml`).

## 2. Backend Foundation (Rust Inference Engine Core)
- [ ] Implement configuration loading module (`rust/src/config.rs`).
  - [ ] Define `Config` struct mirroring `config.toml` structure.
  - [ ] Implement function to load config from `$XDG_CONFIG_HOME/ai_autocorrect/config.toml` or a default path.
- [ ] Implement basic model handler module (`rust/src/model.rs`).
  - [ ] Define `Model` struct.
  - [ ] Implement basic ONNX Runtime environment initialization.
  - [ ] Implement placeholder for model loading function.
  - [ ] Implement placeholder for prediction function.
- [ ] Set up main server module (`rust/src/main.rs` or `rust/src/server.rs`).
  - [ ] Initialize Tokio runtime.
  - [ ] Load configuration.
  - [ ] Initialize basic IPC listener (UNIX socket).
  - [ ] Implement basic request handling loop (placeholder).

## 3. Feature-specific Backend (MVP Features)
- [ ] **Model Loading:**
  - [ ] Implement loading of the ONNX model specified in the config (`model.rs`).
  - [ ] Configure ONNX session to use CUDA provider based on config/availability.
  - [ ] Handle potential errors during model loading.
- [ ] **Inference Logic:**
  - [ ] Implement the `predict` function in `model.rs` to take tokenized input.
  - [ ] Perform inference using the loaded ONNX session.
  - [ ] Handle dynamic input/output shapes (dynamic axes).
  - [ ] Process model output (logits) to extract next-token predictions.
  - [ ] Return prediction results.
- [ ] **IPC Implementation:**
  - [ ] Define IPC request/response format (e.g., JSON over socket).
  - [ ] Implement request parsing in the server loop (`server.rs`).
  - [ ] Call the `model.predict` function with data from the request.
  - [ ] Serialize and send the prediction response back to the client.
  - [ ] Implement asynchronous handling of multiple client connections.
- [ ] **(Phase 2) Beam Search:**
  - [ ] Research and choose a beam search implementation strategy compatible with the model/runtime.
  - [ ] Implement beam search logic within or alongside the `predict` function.
  - [ ] Add configuration option for `beam_width`.
- [ ] **(Phase 2) Quantized Model Support:**
  - [ ] Implement logic to detect and load quantized ONNX models.
  - [ ] Ensure compatibility with the chosen ONNX Runtime version and CUDA.
  - [ ] Add configuration option for `quantization`.

## 4. Frontend Foundation (Plugin Core)
- [ ] **Neovim Plugin:**
  - [ ] Create basic Lua plugin structure (`plugins/neovim.lua` or dedicated directory).
  - [ ] Implement utility function to connect to the Rust daemon's UNIX socket.
  - [ ] Implement basic function placeholders for triggering completion and receiving results.
- [ ] **CLI Script:**
  - [ ] Create shell script (`plugins/autocorrect.sh`).
  - [ ] Implement basic argument parsing (e.g., text input).
  - [ ] Implement basic connection logic using `nc` or `socat` to the UNIX socket.

## 5. Feature-specific Frontend (MVP Features)
- [ ] **Neovim Plugin Integration:**
  - [ ] Implement logic to get the current line or relevant context from the Neovim buffer.
  - [ ] Implement logic to serialize context and send it to the Rust daemon via IPC.
  - [ ] Implement logic to receive and parse the prediction response from the daemon.
  - [ ] Integrate received suggestions with Neovim's completion system (e.g., `nvim_complete`).
  - [ ] Add Neovim commands/mappings to trigger the completion.
- [ ] **CLI Script Functionality:**
  - [ ] Send input text to the Rust daemon via the chosen tool (`nc`/`socat`).
  - [ ] Receive the prediction response from the daemon.
  - [ ] Print the received suggestions to standard output.
- [ ] **Configuration Handling (UX):**
  - [ ] Ensure plugins gracefully handle cases where the daemon is not running or the socket is unavailable.
  - [ ] Document the configuration file location and parameters for users.
- [ ] **(Phase 2) System-wide IME:**
  - [ ] Research Linux IME frameworks (e.g., IBus, Fcitx).
  - [ ] Design architecture for an IME service interacting with the Rust daemon.
  - [ ] Implement IME plugin (requires significant effort, likely separate sub-project).

## 6. Integration
- [ ] Connect Neovim plugin to the running Rust inference daemon.
- [ ] Verify context is sent correctly and predictions are received/displayed in Neovim.
- [ ] Test the CLI script interaction with the running Rust daemon.
- [ ] Verify input is sent and output is received correctly via the shell.
- [ ] Perform end-to-end testing of the complete workflow (typing -> request -> inference -> suggestion).

## 7. Testing
- [ ] **Model Development (Python):**
  - [ ] Write tests for data preprocessing and tokenization.
  - [ ] Validate the ONNX export process, check model structure and dynamic axes.
  - [ ] Test quantized model export and loading (if applicable).
- [ ] **Inference Engine (Rust):**
  - [ ] Write unit tests for configuration loading (`config.rs`).
  - [ ] Write unit tests for model loading (mocking file system/ONNX runtime if needed) (`model.rs`).
  - [ ] Write unit tests for basic prediction logic (using dummy model or data).
  - [ ] Write integration tests for IPC communication (client <-> server).
- [ ] **Plugins:**
  - [ ] Write unit tests for Neovim Lua functions (using a testing framework like `busted`).
  - [ ] Test shell script with various inputs and edge cases.
- [ ] **End-to-End Testing:**
  - [ ] Create automated E2E tests simulating user interaction in Neovim/CLI and verifying daemon response.
- [ ] **Performance Testing:**
  - [ ] Benchmark inference latency per token under different loads/context lengths.
  - [ ] Measure daemon resource usage (CPU, GPU VRAM, RAM).
  - [ ] Compare performance with/without quantization.
- [ ] **Model Quality Testing:**
  - [ ] Evaluate the fine-tuned model's perplexity on a held-out test set.
  - [ ] Perform qualitative analysis of prediction quality.
- [ ] **Security Testing:**
  - [ ] Review IPC mechanism for potential vulnerabilities (if exposed beyond local user).
  - [ ] Analyze dependencies for known security issues.

## 8. Documentation
- [ ] **User Documentation:**
  - [ ] Installation guide (compiling Rust daemon, setting up Python for potential model tasks, installing plugins).
  - [ ] Configuration guide (explaining `config.toml` options).
  - [ ] Usage guide for Neovim plugin and CLI script.
  - [ ] Troubleshooting common issues.
- [ ] **Developer Documentation:**
  - [ ] `README.md` update with project overview, build instructions, and contribution guidelines.
  - [ ] Code comments explaining complex logic in Rust and Lua.
  - [ ] Architecture overview document (`docs/Architecture.md`).
  - [ ] API documentation for the IPC interface.
- [ ] **Model Documentation:**
  - [ ] Details about the base model used for fine-tuning.
  - [ ] Information on the dataset used.
  - [ ] Instructions on how to fine-tune or export custom models.

## 9. Deployment
- [ ] **Build Process:**
  - [ ] Create release build script for the Rust daemon (`cargo build --release`).
- [ ] **Packaging:**
  - [ ] Decide on distribution format (e.g., tarball, `.deb`/`.rpm` package, AUR package).
  - [ ] Package the compiled Rust binary, default config, plugin files, and necessary licenses.
  - [ ] Include instructions for installing the required model file(s).
- [ ] **CI/CD Pipeline:**
  - [ ] Set up GitHub Actions (or similar) workflow.
  - [ ] Automate building the Rust binary for Linux.
  - [ ] Automate running tests (unit, integration).
  - [ ] (Optional) Automate creating release packages/artifacts.
- [ ] **Service Management:**
  - [ ] Create a `systemd` service unit file for running the inference daemon in the background.
  - [ ] Provide instructions for enabling/starting the service.
- [ ] **Monitoring:**
  - [ ] Implement structured logging in the Rust daemon (e.g., using `tracing` or `log` crates).
  - [ ] Document how users can view logs for troubleshooting.

## 10. Maintenance
- [ ] **Bug Tracking:**
  - [ ] Set up issue tracker (e.g., GitHub Issues).
  - [ ] Establish process for reporting and prioritizing bugs.
- [ ] **Update Process:**
  - [ ] Define strategy for releasing updates (daemon, plugins, models).
  - [ ] Ensure backward compatibility where possible or provide clear migration paths.
- [ ] **Dependency Management:**
  - [ ] Regularly review and update dependencies (Rust crates, Python packages) for security patches and new features.
- [ ] **Performance Monitoring:**
  - [ ] Gather user feedback on performance in real-world usage.
  - [ ] Periodically re-run benchmarks to catch regressions.
- [ ] **Model Retraining/Updates:**
  - [ ] Plan for potential retraining of the model with new data or improved architectures. 