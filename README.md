# AI Autocursor

> **On-premise, GPU-accelerated, context-aware AI autocorrect/autocomplete for Linux.**

---

## Overview
AI Autocursor is a privacy-first, low-latency AI autocorrect and autocomplete system designed for Linux. It runs entirely locally, leveraging GPU acceleration for sub-50ms per-token inference. The system is ideal for developers, writers, and privacy advocates who want fast, context-aware suggestions in their CLI or editor—without sending data to the cloud.

---

## Features
- **Local Inference**: No external API calls; all processing is on your machine.
- **GPU Acceleration**: Uses ONNX Runtime with CUDA for fast predictions.
- **Editor & CLI Integration**: Plugins for Neovim and a shell script for CLI usage.
- **Configurable**: Flexible `config.toml` for model path, context window, beam width, quantization, and more.
- **Extensible**: Modular Python (training/export) and Rust (inference) codebases.
- **Privacy-Respecting**: No telemetry, no data leaves your device.

---

## Architecture
```
[Training (Python)] ──> [ONNX Exporter] ──> [Rust Inference Daemon]
       │                                       │
       └── fine-tuning + quantization         └── IPC (UNIX socket / HTTP)
                                               └── Editor/CLI plugin
```
- **Python**: Model training, fine-tuning, and ONNX export.
- **Rust**: Inference daemon, configuration, and IPC server.
- **Plugins**: Neovim (Lua) and CLI (shell script) clients.

---

## Quickstart
### 1. Requirements
- **Linux** (kernel ≥ 5.15)
- **NVIDIA GPU** (CUDA 11.7+ recommended)
- **Rust** (≥ 1.65)
- **Python** (≥ 3.9)

### 2. Setup
#### Python (Model Training/Export)
```bash
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
# Train and export your model (see training/ directory)
```

#### Rust (Inference Daemon)
```bash
cd inference
cargo build --release
```

#### Configuration
Copy and edit the example config:
```bash
cp docs/config.example.toml configs/default.toml
# Edit configs/default.toml to set model_path, context_window, etc.
```

#### Start the Daemon
```bash
./target/release/inference_daemon
```

#### Neovim Plugin
See `plugins/neovim/README.md` for installation and usage.

#### CLI Script
See `plugins/cli/README.md` for usage, or run:
```bash
plugins/cli/autocursor.sh "your text here"
```

---

## Configuration Example
```toml
model_path = "/path/to/your/model.onnx"
context_window = 128
beam_width = 1
quantization = false
socket_path = "/tmp/ai_autocorrect.sock"
# gpu_device = 0
# log_level = "info"
```

---

## Directory Structure
```
├── training/         # Python scripts for training/export
├── inference/        # Rust inference daemon
├── plugins/          # Editor and CLI plugins
├── models/           # ONNX and quantized models
├── configs/          # Configuration files
├── docs/             # Documentation (PRD, plan, tech spec)
├── scripts/          # Helper scripts
├── tests/            # Unit, integration, and e2e tests
```

---

## Contributing
Contributions are welcome! Please see `docs/development-steps.md` and open issues or pull requests for discussion.

---

## License
See [LICENSE](LICENSE) for details.

---

## References
- [Product Requirements Document](docs/PRD.md)
- [Development Plan](docs/plan.md)
- [Technical Implementation](docs/Technical-Implementation.md)
- [Example Config](docs/config.example.toml)
