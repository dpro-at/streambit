# StreamBit

**High-Performance Rust Library for AI Data Loading**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/python-3.8%2B-blue.svg)](https://www.python.org/)

StreamBit is a zero-copy, multi-modal data ingestion engine designed to eliminate bottlenecks in AI/ML workflows. Built in Rust with Python bindings, it provides blazing-fast data loading for images, video, audio, text, and tabular data.

## 🚀 Features

- **Multi-Modal Support**: Images, Video, Audio, Text (PDF/DOCX), CSV/Parquet
- **Zero-Copy Python Integration**: Direct memory sharing with NumPy/PyTorch/TensorFlow
- **Parallel Processing**: Automatic CPU parallelization with Rayon
- **Async Streaming**: Process data while downloading from URLs
- **Pure Rust**: Minimal external dependencies (FFmpeg only for video)
- **Memory Efficient**: Memory-mapped file search, streaming decoders
- **Professional CLI**: Beautiful progress bars and performance metrics
- **Interactive Dashboard**: Streamlit GUI with live performance graphs

## 📊 Performance Targets

| Operation | Target Performance |
|-----------|-------------------|
| Image Loading | >1000 images/second (224x224 resize) |
| Video Processing | >60 FPS frame extraction |
| Audio Decoding | >100x realtime for MP3 |
| Text Extraction | >50 PDFs/second |
| File Search | <100ms for 1000+ files |

## 🏗️ Architecture

StreamBit is organized as a modular Rust workspace:

```
streambit/
├── streambit-core/       # Shared utilities and types
├── streambit-vision/     # Image and video processing
├── streambit-audio/      # Audio decoding and spectrograms
├── streambit-text/       # Document extraction and search
├── streambit-tabular/    # CSV/Parquet support
├── streambit-streaming/  # Async HTTP streaming
├── streambit-cli/        # CLI with progress bars
├── streambit-python/     # PyO3 Python bindings
├── dashboard/            # Streamlit GUI dashboard
└── benchmarks/           # Performance comparisons
```

## 📦 Installation

### Prerequisites

**System Requirements:**
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs))
- Python 3.8+ with pip
- FFmpeg (for video processing only)

**Install FFmpeg:**

```bash
# Ubuntu/Debian
sudo apt-get install ffmpeg libavcodec-dev libavformat-dev libavutil-dev libswscale-dev

# macOS
brew install ffmpeg

# Windows
# Download from https://ffmpeg.org/download.html
```

### Install StreamBit (Coming Soon)

```bash
pip install streambit
```

Or build from source:

```bash
git clone https://github.com/Dpro-at/StreamBit.git
cd StreamBit/streambit-python
pip install maturin
maturin develop --release
```

## 🎯 Quick Start

### Image Processing

```python
import streambit
import torch

# Load and resize images in parallel
processor = streambit.ImageProcessor()
images = processor.load_batch(
    ['img1.jpg', 'img2.jpg', 'img3.jpg'],
    resize=(224, 224),
    num_threads=8
)

# Zero-copy conversion to PyTorch
tensor = torch.from_numpy(images)  # Shape: (3, 3, 224, 224) - CHW format
```

### Audio Processing

```python
import streambit

# Decode audio and convert to Mel-spectrogram
audio_processor = streambit.AudioProcessor()
spectrogram = audio_processor.load_mel_spectrogram(
    'audio.mp3',
    sample_rate=22050,
    n_mels=128
)
```

### Text Extraction

```python
import streambit

# Extract text from documents
text_extractor = streambit.TextExtractor()
text = text_extractor.extract('document.pdf')

# Batch processing
texts = text_extractor.extract_batch(['doc1.pdf', 'doc2.docx', 'doc3.txt'])
```

### Instant Search

```python
import streambit

# Search across 1000+ files in milliseconds
search_engine = streambit.SearchEngine()
search_engine.index_directory('./data')
results = search_engine.search('machine learning', max_results=10)
```

## 🖥️ CLI Usage

```bash
# Run benchmarks with progress bars
streambit benchmark

# Process files with live progress
streambit process images/*.jpg --resize 224x224

# Search files instantly
streambit search "pattern" --path ./data
```

## 📊 Dashboard

Launch the interactive Streamlit dashboard:

```bash
cd dashboard
pip install -r requirements.txt
streamlit run app.py
```

Features:
- 📈 Live performance graphs (StreamBit vs Python libraries)
- 🖼️ File preview gallery with processing stats
- 🔍 Instant search demo (1000+ files)
- 🌙 Dark mode with StreamBit branding

## 🔧 Development

### Build All Crates

```bash
cargo build --workspace --release
```

### Run Tests

```bash
cargo test --workspace
```

### Run Benchmarks

```bash
cargo bench --workspace
```

### Build Python Bindings

```bash
cd streambit-python
maturin develop --release
pytest tests/
```

## 📖 Documentation

- [Implementation Plan](./doc/Implementation/IMPLEMENTATION_PLAN.md) - Complete technical roadmap
- [Task Checklist](./doc/task/TASKS.md) - Detailed task breakdown
- [Project Metadata](./doc/PROJECT_METADATA.md) - Author and license information

## 🗺️ Roadmap

- [x] **Stage 1**: Core infrastructure and vision module (In Progress)
- [ ] **Stage 2**: Streaming and Python bindings
- [ ] **Stage 3**: Search engine and medical imaging
- [ ] **Stage 4**: CLI, dashboard, and benchmarks
- [ ] **Stage 5**: Documentation and publishing

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

Licensed under the [MIT License](LICENSE).

**Copyright (c) 2025 Mohamed ALarade (Dpro GmbH)**

Permission is granted to use, modify, and distribute this software freely, with the requirement to include attribution to the original source.

## 🙏 Acknowledgments

StreamBit builds on excellent Rust crates:
- [image](https://github.com/image-rs/image) - Image decoding
- [symphonia](https://github.com/pdeljanov/Symphonia) - Audio decoding
- [rayon](https://github.com/rayon-rs/rayon) - Data parallelism
- [PyO3](https://github.com/PyO3/pyo3) - Python bindings
- [polars](https://github.com/pola-rs/polars) - DataFrame operations

## 📧 Contact

**Author**: Mohamed ALarade  
**Email**: m.alarade@dpro.at  
**Company**: Dpro GmbH  
**GitHub**: [@9mtm](https://github.com/9mtm) | [@Dpro-at](https://github.com/Dpro-at)

---

**⭐ If you find StreamBit useful, please give it a star on GitHub!**
