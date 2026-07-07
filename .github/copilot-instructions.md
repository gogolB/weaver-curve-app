# Weaver Curve Application - GitHub Copilot Instructions

**ALWAYS follow these instructions first and fallback to additional search and context gathering only if the information here is incomplete or found to be in error.**

## Application Overview
Weaver Curve App is a cross-platform desktop application for medical providers to generate and analyze Weaver growth curves with gestational age correction. Built with:
- **Frontend**: SvelteKit with TypeScript, Vite, TailwindCSS, DaisyUI
- **Backend**: Rust with Tauri framework v2.x
- **PDF Generation**: Typst templating system with embedded chart generation
- **Chart Rendering**: Charming library for SVG/PNG chart output
- **Platform Support**: Windows, macOS (Intel & Apple Silicon), Linux

## Prerequisites & Environment Setup

### Required System Dependencies (Linux/CI)
```bash
# Install Linux dependencies for Tauri development
sudo apt-get update
sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
```

### Verify Environment
```bash
# Check required tools (should all be available)
node --version    # Expected: v20.19.5+
npm --version     # Expected: 10.8.2+
rustc --version   # Expected: 1.89.0+
cargo --version   # Expected: 1.89.0+
npx tauri --version  # Expected: 2.8.4+ (installed via npm)
```

## Build & Development Commands

### Install Dependencies
```bash
# Install Node.js dependencies (takes ~15 seconds)
npm install
```

### Frontend Development & Building
```bash
# Build frontend only (takes ~5 seconds)
npm run build

# Type check Svelte components (takes ~4 seconds)
npm run check

# Watch mode for type checking
npm run check:watch

# Development server for frontend only
npm run dev  # Runs on http://localhost:1420
```

### Full Application Build
```bash
# NEVER CANCEL: Full Tauri build - takes 13-15 minutes. Set timeout to 20+ minutes.
# This includes frontend build + Rust compilation + packaging
time npx tauri build
```

**CRITICAL BUILD TIMING:**
- Frontend build: ~5 seconds
- Rust compilation: 8-10 minutes (first build) or 2-5 minutes (incremental)
- Packaging: 2-3 minutes
- **Total initial build time: 13-15 minutes**
- **NEVER CANCEL builds - they may appear to hang but are processing normally**

### Development Mode
```bash
# NEVER CANCEL: Development mode with hot reload - takes 30-60 seconds for compilation
# Set timeout to 5+ minutes for first run
# NOTE: Will fail in headless environments (CI) due to missing GUI - this is expected
npx tauri dev
```

## Testing & Quality Assurance

### Rust Testing & Linting
```bash
# NEVER CANCEL: Compile tests (takes ~2.5 minutes) - Set timeout to 5+ minutes
cd src-tauri && cargo test --no-run

# Run Rust tests (if any exist)
cd src-tauri && cargo test

# Check Rust code formatting
cd src-tauri && cargo fmt --check

# Apply Rust formatting
cd src-tauri && cargo fmt

# NEVER CANCEL: Run Rust linting (takes 1-3 minutes) - Set timeout to 5+ minutes  
cd src-tauri && cargo clippy
```

### Frontend Testing & Linting
```bash
# Type checking for Svelte/TypeScript (takes ~4 seconds)
npm run check

# No additional linting or testing tools are configured
# Use npm run check as the primary validation step
```

## Application Validation & Testing

### Manual Functionality Testing
After making changes, ALWAYS test the application functionality:

1. **Build and run the application:**
   ```bash
   npx tauri dev  # For local development with GUI
   # OR for headless environments:
   npx tauri build  # Then install and test the generated package
   ```

2. **Test core workflow with sample data:**
   ```
   Sample Patient Data:
   - Child age: 24 months
   - Child head circumference: 48.5 cm
   - Mother head circumference: 55.2 cm  
   - Father head circumference: 57.8 cm
   - Gender: Select either Male or Female
   - Date of birth: Optional (e.g., "2022-01-15")
   - Premature conception: 0 weeks, 0 days (for normal term)
   
   Expected Results:
   - Child score: ~-0.5 to -1.0 (within normal range)
   - Parent average: ~0.0 to 0.5
   - Should plot within ±2 SD lines
   ```

3. **Alternative sample for gestational age correction testing:**
   ```
   Premature Birth Sample:
   - Child age: 12 months  
   - Child head circumference: 45.0 cm
   - Mother head circumference: 54.0 cm
   - Father head circumference: 56.0 cm
   - Gender: Male
   - Premature conception: 4 weeks, 0 days (36 weeks gestation)
   
   Expected Results:
   - Shows both original and corrected scores
   - Corrected age: ~11.1 months
   - Both points plotted on chart
   ```
   
4. **Verify calculation and visualization:**
   - Click to calculate scores
   - Verify Weaver plot displays correctly with:
     - Parental average line (blue)
     - ±2 SD lines (orange, dashed)
     - Child score point plotted
     - Corrected score point (if premature data entered)

5. **Test PDF generation:**
   - Click "Export to PDF" button
   - Select save location in file dialog
   - Verify PDF file is created with:
     - Patient demographics table
     - Clinical measurements
     - Score calculations table with color coding
     - Embedded Weaver chart matching the on-screen plot

### CI/Build Validation Steps
Before committing changes, ALWAYS run:
```bash
# Frontend validation
npm run check

# Rust validation  
cd src-tauri && cargo fmt --check
cd src-tauri && cargo clippy

# Full build test
npx tauri build
```

## Key File Locations & Architecture

### Frontend (SvelteKit)
- **Main page**: `src/routes/+page.svelte` - Main application UI with form inputs and chart display
- **Components**: `src/components/` 
  - `weaver_plot.svelte` - D3.js-based chart visualization component with PDF export
  - `score_card.svelte` - Score display component with color-coded results
- **Configuration**: 
  - `vite.config.js` - Vite bundler config (port 1420, Tauri integration)
  - `svelte.config.js` - SvelteKit configuration with static adapter
  - `tailwind.config.js` - TailwindCSS styling with DaisyUI plugin
  - `tsconfig.json` - TypeScript configuration extending SvelteKit

### Backend (Tauri/Rust)
- **Main logic**: `src-tauri/src/main.rs` - Contains core application functions:
  - `calculate_scores()` - Weaver curve z-score calculations with gestational age correction
  - `make_pdf()` - PDF generation using Typst templating with embedded fonts and charts
  - `generate_chart()` - Chart rendering using Charming library for PNG output
  - Error handling with custom Error enum and serde serialization
- **Configuration**: 
  - `src-tauri/Cargo.toml` - Rust dependencies (Tauri v2, Typst, Charming, etc.)
  - `src-tauri/tauri.conf.json` - Tauri app configuration (window size, bundle settings)
  - `src-tauri/build.rs` - Build script for Tauri code generation

### Resources & Templates
- **PDF Template**: `src-tauri/resources/templates/template.typ` - Typst markup template for medical reports
- **Fonts**: `src-tauri/resources/fonts/Roboto/` - Complete Roboto font family for PDF generation
- **Icons**: `src-tauri/icons/` - Application icons for different platforms (ICO, ICNS, PNG)

### Build & Deployment
- **GitHub Workflow**: `.github/workflows/release.yml` - Multi-platform release builds (Windows/macOS/Linux)
- **VS Code**: `.vscode/` - Debug configurations and tasks for development
- **Dependencies**: `package-lock.json` (frontend), `src-tauri/Cargo.lock` (backend)

## Common Development Tasks

### Adding New Calculations
1. Modify `calculate_scores()` function in `src-tauri/src/main.rs`
2. Update frontend to handle new parameters in `src/routes/+page.svelte`
3. Test with sample data using validation workflow above
4. Run `cargo fmt` and `cargo clippy` before committing

### Modifying PDF Output
1. Edit Typst template in `src-tauri/resources/templates/template.typ`
2. Update `ContentData` struct and `make_pdf()` function if adding new fields
3. Test PDF generation through UI workflow
4. Verify PDF renders correctly with new content

### UI Changes
1. Modify Svelte components in `src/components/` or main page `src/routes/+page.svelte`
2. Use TailwindCSS classes with DaisyUI components for styling
3. Run `npm run check` to verify TypeScript types
4. Test in development mode with `npx tauri dev`

### Performance Optimization
- **Development builds**: Use `npx tauri dev` for faster iteration
- **Release builds**: Only run `npx tauri build` when necessary due to long compile times
- **Incremental compilation**: Rust builds are faster after initial compilation

## Known Issues & Workarounds

### Build Issues
- **AppImage packaging may fail** in headless CI environments - this is expected, .deb and .rpm packages build successfully
- **First Rust compilation** takes 8-10 minutes - this is normal for the dependency tree size
- **Vite warnings** about untrack exports are cosmetic and don't affect functionality

### Development Environment
- **VS Code tasks** reference `yarn` but project uses `npm` - use `npm` commands instead
- **No ESLint/Prettier** configured - rely on TypeScript checking via `npm run check`
- **Desktop app testing** requires full build - UI can be previewed with `npm run dev` but Tauri APIs won't work
- **Development mode fails in headless environments** (CI/Docker) due to missing GUI - use `npx tauri build` for validation instead

### Headless Environment Limitations
- **Cannot run `npx tauri dev`** - requires display server (X11/Wayland)
- **Cannot test desktop app functionality** - build validates code compilation only
- **Use frontend preview** with `npm run dev` for UI changes in headless environments

### Performance Notes
- **Cold builds are slow**: First `npx tauri build` takes 13-15 minutes due to dependency compilation
- **Incremental builds faster**: Subsequent builds take 2-5 minutes if only Rust code changed
- **Frontend builds are fast**: `npm run build` typically completes in under 10 seconds
- **Development compilation**: `npx tauri dev` compiles in 20-60 seconds for incremental changes

### Debugging Common Issues
- **Build fails with webkit errors**: Ensure Linux dependencies are installed (see Prerequisites)
- **Rust format check fails**: Run `cargo fmt` to auto-fix formatting issues
- **TypeScript errors**: Run `npm run check` to see detailed error messages
- **PDF generation fails**: Check that font files exist in `src-tauri/resources/fonts/Roboto/`
- **Chart rendering issues**: Verify Charming library chart data structure in `generate_chart()` function

## Timeout Recommendations for CI/Automation

- **npm install**: 5 minutes
- **npm run build**: 2 minutes  
- **npm run check**: 2 minutes
- **npx tauri build**: 20 minutes (NEVER CANCEL)
- **npx tauri dev**: 5 minutes for compilation (will fail in headless - expected)
- **cargo test --no-run**: 5 minutes (NEVER CANCEL)
- **cargo clippy**: 5 minutes (NEVER CANCEL)
- **cargo fmt --check**: 1 minute

**CRITICAL: Never cancel long-running build commands. Rust compilation legitimately takes 8+ minutes on first build.**