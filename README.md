# Pitch-Detector

Pitch-Detector is a Rust-based application designed to analyze audio signals and determine their fundamental frequency (pitch).
This tool can be utilized in various applications, such as musical tuning, speech analysis, and audio research.

## Features

- **Pitch Detection**: Accurately identifies the fundamental frequency of an audio signal.
- **Real-Time Processing**: Capable of analyzing audio input in real-time for immediate feedback.
- **Cross-Platform**: Developed in Rust, ensuring compatibility across multiple operating systems.

## Installation

To build and run Pitch-Detector, ensure you have the latest stable version of Rust installed.

1. **Clone the repository**:

   ```bash
   git clone https://github.com/cyptrix12/Pitch-Detector.git
   cd Pitch-Detector
   ```

2. **Build the project**:

   ```bash
   cargo build --release
   ```

3. **Run the application**:

   ```bash
   cargo run --release
   ```

## Usage

Upon running the application, it will use your default microphone.
The program will then analyze the audio signal and display the detected pitch in Hertz (Hz) and nearest note.
