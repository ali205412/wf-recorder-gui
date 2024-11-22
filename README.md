# WF Recorder GUI

[![CI](https://github.com/cline/wf-recorder-gui/actions/workflows/ci.yml/badge.svg)](https://github.com/cline/wf-recorder-gui/actions/workflows/ci.yml)
[![Release](https://github.com/cline/wf-recorder-gui/actions/workflows/release.yml/badge.svg)](https://github.com/cline/wf-recorder-gui/actions/workflows/release.yml)
[![AUR version](https://img.shields.io/aur/version/wf-recorder-gui)](https://aur.archlinux.org/packages/wf-recorder-gui/)

A modern, minimal, and sleek GUI for wf-recorder, the Wayland screen recorder. Built with GTK4 and Rust, featuring an intuitive interface and efficient workflow.

## Features

- Modern GTK4 native interface
- Clean, minimal design
- Full screen and region capture
- Multiple audio source options:
  - System audio
  - Microphone
  - No audio
- 📹 Multiple output formats:
  - WebM
  - MP4
  - MKV
- Custom save location
- Hardware acceleration support
- Wayland native

## Installation

### Arch Linux (AUR)

```bash
yay -S wf-recorder-gui
```
or
```bash
paru -S wf-recorder-gui
```

### Debian/Ubuntu

Download the latest .deb package from the [releases page](https://github.com/cline/wf-recorder-gui/releases) and install it:
```bash
sudo dpkg -i wf-recorder-gui_*.deb
sudo apt-get install -f  # Install dependencies if needed
```

### Building from Source

Requirements:
- Rust (1.70.0 or later)
- GTK4 development files
- wf-recorder

```bash
# Install dependencies (Arch Linux)
sudo pacman -S gtk4 libadwaita wf-recorder base-devel

# Install dependencies (Debian/Ubuntu)
sudo apt-get install libgtk-4-dev libadwaita-1-dev wf-recorder build-essential

# Clone the repository
git clone https://github.com/cline/wf-recorder-gui.git
cd wf-recorder-gui

# Build and run
cargo build --release
./target/release/wf-recorder-gui
```

## Usage

1. Launch the application
2. Choose your recording options:
   - Select output format (WebM/MP4/MKV)
   - Choose capture mode (Full Screen/Region)
   - Select audio source (System/Microphone/None)
   - Set save location
3. Click Record to start
4. Click Stop when finished

## Development

### Project Structure

```
src/
├── audio/       # Audio handling
├── config/      # Configuration management
├── recorder/    # Recording functionality
├── ui/         # User interface components
└── main.rs     # Application entry point
```

### CI/CD Workflows

The project uses GitHub Actions for:
- Continuous Integration (CI)
  - Building and testing on each push
  - Code formatting checks
  - Clippy linting
  - Security audits
- Release automation
  - Building Debian packages
  - Creating GitHub releases
- Automated AUR updates
  - Publishing and updating the AUR package

### Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please make sure to:
- Follow the existing code style
- Add tests if applicable
- Update documentation as needed

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [wf-recorder](https://github.com/ammen99/wf-recorder) - The underlying screen recording utility
- [GTK](https://gtk.org/) - The GUI toolkit
- All contributors and users of this project

## Support

If you encounter any issues or have suggestions:
1. Check the [Issues](https://github.com/cline/wf-recorder-gui/issues) page
2. Open a new issue if needed
3. Provide as much detail as possible:
   - System information
   - Steps to reproduce
   - Expected vs actual behavior
