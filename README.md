# ESP-RS Development Environment Template

## Introduction

This template provides a development environment for ESP32 with Rust, leveraging Nix for dependency management. It's designed to streamline the setup process for Rust-based ESP32 projects, offering a robust and reproducible development environment.

## Contents

- **`flake.nix`**: Configures the development environment, including toolchains and compilers necessary for ESP32 development in Rust.
- **`embuild.nix`**: Defines the build process for the `embuild` package, a crucial component for Rust development with ESP32.
- **`espup.nix`**: Sets up `espup`, a tool for managing ESP toolchains and dependencies.

### Features

- Automated setup of Rust toolchains and ESP compilers.
- Preconfigured build and management tools like `cargo-espflash`, `embuild`, and `espup`.
- A reproducible development environment managed by Nix.

## Getting Started

### Prerequisites

- Nix Package Manager
- Basic knowledge of Nix and Rust

### Installation

1. **Clone the Repository**: Clone this repository to your local machine.
2. **Enter the Development Shell**: Run `nix develop` in the root directory of this template to enter the Nix-managed development shell.
3. **Start Developing**: You're now ready to start your ESP32 project with Rust!

## Usage

- Use the development shell to access all the necessary tools and compilers for ESP32 development.
- Modify and extend the Nix expressions (`flake.nix`, `embuild.nix`, `espup.nix`) as per your project needs.

## Support & Contribution

- For support, open an issue in the repository.
- Contributions to this template are welcome. Please submit pull requests with your proposed changes.
