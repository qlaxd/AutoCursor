# Product Requirements Document (PRD) — AI Autocorrect

---

## 1. Purpose & Vision

**Objective**:  
On-premise, GPU-accelerated, context-aware AI autocorrect/autocomplete for Linux.

**Why**:  
Eliminate API latency and privacy concerns — everything runs locally.

**Audience**:  
Power users, developers, and privacy-conscious writers who need sub-50 ms per-token latency.

---

## 2. Scope

**MVP (Phase 1)**:
- Sentence-level next-token prediction.
- CLI/editor plugin (Neovim/Vim).
- Rust-based inference with CUDA via ONNX Runtime or `tch-rs`.

**Phase 2**:
- Beam-search autocorrect suggestions.
- Quantized model support (e.g., `gguf`, ONNX quantization).
- System-wide IME integration (GTK/Qt).

---

## 3. Key Goals & Success Metrics

| Goal                  | Metric                  | Target             |
|-----------------------|-------------------------|--------------------|
| Low-latency inference | Avg. latency per token  | ≤ 50 ms            |
| High prediction quality | Perplexity on test set | ≤ 25               |
| Compact deployment    | Binary + model size     | ≤ 50 MB (quantized)|
| User satisfaction     | Survey rating           | ≥ 4 / 5            |

---

## 4. User Stories

- As a **developer**, I want inline sentence completion in my editor so I can code faster.
- As a **writer**, I want real-time autocorrect suggestions to minimize typos.
- As a **privacy advocate**, I demand 100% local processing—no external API calls.

---

## 5. Features & Requirements

### Model Training & Export
- Fine-tune a medium transformer on domain-specific data.
- Export to ONNX with dynamic axes for varying context lengths.
- Provide quantized variants for low-memory GPUs.

### Inference Engine
- Rust daemon using `onnxruntime-rs` (CUDA) or `tch-rs`.
- Async IPC (UNIX socket) + optional HTTP REST fallback.

### Editor/CLI Integration
- Neovim plugin (Lua) calls the local daemon over socket.
- Fallback shell script for plain CLI usage.

### Configuration & UX
- Configuration path: `$XDG_CONFIG_HOME/ai_autocorrect/config.toml`.
- Parameters:
  - `context_window`
  - `beam_width`
  - `quantization`
  - `gpu_device`

---

## 6. Assumptions & Constraints

- **Hardware**: NVIDIA GPU with CUDA 11.7+; ≥ 8 GB VRAM (quantization target: ≥ 4 GB).
- **OS**: Linux kernel ≥ 5.15, distro-agnostic.
- **Toolchains**:
  - Rust ≥ 1.65
  - Python 3.9+
- **Licenses**: Models must allow local use (e.g., GPT-2, LLaMA-style under permissive terms).

---

## 7. Timeline & Milestones

| Week    | Deliverable                            |
|---------|----------------------------------------|
| 1–2     | PyTorch prototype + ONNX export script |
| 3–4     | Rust inference daemon (CPU → GPU)      |
| 5       | CLI/editor plugin + IPC layer          |
| 6       | Quantization, benchmarks, e2e tests    |
| 7       | Beta release, docs, user feedback      |

---
