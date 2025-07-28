# WF Recorder GUI

[![CI](https://github.com/ali205412/wf-recorder-gui/actions/workflows/ci.yml/badge.svg)](https://github.com/ali205412/wf-recorder-gui/actions/workflows/ci.yml)
[![Release](https://github.com/ali205412/wf-recorder-gui/actions/workflows/release.yml/badge.svg)](https://github.com/ali205412/wf-recorder-gui/actions/workflows/release.yml)
[![AUR version](https://img.shields.io/aur/version/wf-recorder-gui)](https://aur.archlinux.org/packages/wf-recorder-gui/)

A modern, minimal, and sleek GUI for wf-recorder, the Wayland screen recorder. Built with iced and Rust, featuring a lightweight native interface with persistent settings.

## Features

- Lightweight iced-based native interface
- Clean, minimal design
- Full screen and region capture
- Multiple audio source options:
  - System audio
  - Microphone
  - No audio
- Multiple output formats:
  - WebM
  - MP4
  - MKV
- Custom save location with persistent settings
- Hardware acceleration support
- Wayland native

## Installation

### Arch Linux (Recommended)

Install from AUR:
```bash
yay -S wf-recorder-gui
```
or
```bash
paru -S wf-recorder-gui
```

### Other Distributions

Build from source:

1. Install dependencies (package names may vary):
   - wf-recorder
   - Rust toolchain

2. Build and install:
```bash
git clone https://github.com/ali205412/wf-recorder-gui.git
cd wf-recorder-gui
cargo build --release
sudo install -Dm755 target/release/wf-recorder-gui /usr/bin/wf-recorder-gui
sudo install -Dm644 wf-recorder-gui.desktop /usr/share/applications/wf-recorder-gui.desktop
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

### Requirements

- Arch Linux (recommended for development)
- Dependencies:
```bash
sudo pacman -S wf-recorder base-devel rust
```

### Project Structure

```
src/
├── audio/       # Audio handling
├── config/      # Configuration management with persistence
├── recorder/    # Recording functionality
└── main.rs     # Application entry point with iced UI
```

### CI/CD Workflows

The project uses GitHub Actions for:
- Continuous Integration (CI)
  - Building and testing on Arch Linux
  - Code formatting checks
  - Clippy linting
  - Security audits
- Release automation
  - Building Arch packages
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
1. Check the [Issues](https://github.com/ali205412/wf-recorder-gui/issues) page
2. Open a new issue if needed
3. Provide as much detail as possible:
   - System information
   - Steps to reproduce
   - Expected vs actual behavior
