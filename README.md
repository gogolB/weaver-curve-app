# Weaver Curve Plotter

## Overview

A user-friendly application designed for physicians to quickly plot and analyze Weaver curves.

## Key Features

- Easy data input
- Real-time curve generation
- Export options for reports and presentations

## Benefits

- Streamlines patient growth assessment
- Enhances clinical decision-making
- Improves visual communication with patients and families

## Development

### Prerequisites

- Node.js (LTS version)
- Rust (latest stable)
- System dependencies for Tauri development:
  - Linux: `libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf libgtk-3-dev`
  - macOS: Xcode command line tools
  - Windows: Visual Studio Build Tools

### Setup

```bash
# Install frontend dependencies
npm install

# Install Tauri CLI (if not already installed)
npm install -g @tauri-apps/cli@latest
```

### Development Commands

```bash
# Run in development mode
npm run dev

# Run type checking
npm run check

# Run tests
npm run test

# Build for production
npm run build

# Run Rust tests
cd src-tauri && cargo test
```

### Testing

This project includes comprehensive testing:

- **Frontend tests**: Vitest with utility function tests
- **Backend tests**: Rust unit tests for core calculation functions
- **CI/CD**: GitHub Actions workflow runs tests on every push and PR

Run all tests with:
```bash
npm run test:run  # Frontend tests
cd src-tauri && cargo test  # Backend tests
```

## Technical Details

- **Frontend**: SvelteKit with TypeScript
- **Backend**: Rust with Tauri framework
- **Platform**: Apple (x86 and Apple Silicon), Windows, and Linux
- **Updates**: Regular feature enhancements based on user feedback

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
