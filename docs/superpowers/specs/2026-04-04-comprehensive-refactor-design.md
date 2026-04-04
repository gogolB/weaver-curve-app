# Comprehensive Refactor & PDF Overhaul — Design Spec

**Date:** 2026-04-04
**Scope:** Fix all 12 review issues, overhaul PDF output, Svelte 5 migration, replace typst with frontend PDF generation, comprehensive testing including E2E, upgrade dependencies

---

## 1. Centralize Medical Constants (Critical)

**Problem:** Regression constants `intercept = 0.138891`, `slope = 0.483034`, and the ±2 SD abnormality threshold are duplicated in `score_card.svelte`, `weaver_plot.svelte`, and `main.rs`.

**Design:**
- Create `src/lib/constants.ts` as the single source of truth for the frontend. Export `INTERCEPT`, `SLOPE`, `SD_THRESHOLD`.
- In Rust, define these as module-level `const` values in a new `src-tauri/src/constants.rs` module.
- Both `score_card.svelte` and `weaver_plot.svelte` import from `constants.ts`.
- `generate_chart()` and any future Rust code imports from `constants.rs`.
- Add a Tauri command `get_constants` that returns the Rust values, so a future version could validate frontend/backend agreement at startup.

---

## 2. Error Handling on Tauri Invoke (High)

**Problem:** `invoke("calculate_scores", ...)` in `+page.svelte` has no `.catch()`. Silent failures in a medical app.

**Design:**
- Add `.catch()` to both `invoke("calculate_scores")` and the existing `invoke("make_pdf")`.
- Display errors using a DaisyUI alert banner at the top of the form (set `error` string, show conditionally).
- On the Rust side, change `calculate_scores` to return `Result<(f64, f64, f64, f64), Error>` so it can return validation errors rather than relying solely on frontend validation.

---

## 3. Replace Magic Gender Encoding (Medium)

**Problem:** Gender is `0`/`1` integer throughout the stack.

**Design:**
- Rust: Create `enum Gender { Male, Female }` with `serde::Deserialize` (as string). Update `calculate_scores`, `get_head_circumference_data` signatures.
- Frontend: Pass `"male"` / `"female"` strings directly to Tauri invoke (Serde handles deserialization). Remove the `gender_id` reactive variable.

---

## 4. Static Clinical Lookup Tables (Medium)

**Problem:** `get_head_circumference_data()` heap-allocates Vecs on every call.

**Design:**
- Move all lookup arrays to module-level `const` arrays in `constants.rs`.
- `get_head_circumference_data()` references these statics with slices (`&[f64]`).

---

## 5. Remove Unused Code (Low-Medium)

- `Tick.svelte`: Remove unused `d3` import.
- `weaver_plot.svelte`: Use selective d3 imports only (`d3-selection`, `d3-axis`).
- `+page.svelte`: Remove unused `outerWidth`/`outerHeight` bindings. Remove `dateOnlyRegex` if unused path confirmed.
- Evaluate whether `Tick.svelte` is used at all — if not, remove the file.

---

## 6. Validation Improvements (Medium)

**Design:**
- Add upper bounds: child age max 216 months, head circumference max 70 cm (reasonable clinical upper bound), premature weeks max 42 (full term).
- Clear error flags at the start of `process_form()` before re-validating.
- Add Rust-side validation in `calculate_scores` as defense-in-depth (return `Err` for out-of-range values).

---

## 7. Frontend Tests (Low-Medium)

**Design:**
- Rewrite `utils.test.ts` to import from `constants.ts` and test the actual exported values/functions.
- Add at least one component smoke test for `score_card.svelte` using `@testing-library/svelte`.
- Remove tests that just assert literal math (`expect(2.1).toBe(2.1)`).

---

## 8. Fix Typo: `gestiational` → `gestational`

- `score_card.svelte:9`: Rename prop `gestiational_age_in_weeks` → `gestational_age_in_weeks`.
- Update the caller in `+page.svelte`.

---

## 9. Remove Rust PDF Pipeline — Replace with Frontend PDF Generation (Critical)

**Problem:** The current Rust PDF pipeline (typst + typst-as-lib + typst-pdf + derive_typst_intoval + charming) is:
- The source of silent failures reported by users (`.expect()` panics with no error feedback)
- Extremely fragile to upgrade (typst 0.11→0.14 breaks `Tracer`, `TypstTemplate`, `pdf()` signature, font API)
- Redundant — it re-renders the chart in Rust using `charming` when the chart already exists as SVG in the browser
- Heavy — 5 crates adding significant compile time and binary size

**Design: Move PDF generation entirely to the frontend.**

### What gets removed from Rust:
- `make_pdf` Tauri command
- `generate_chart` function
- `Content` / `ContentData` structs
- All typst/charming imports and dependencies
- `resources/fonts/Roboto/` directory (13 font files)
- `resources/templates/template.typ`
- Cargo dependencies: `typst`, `typst-pdf`, `typst-as-lib`, `derive_typst_intoval`, `charming`, `tempfile`
- Bundle resources config in `tauri.conf.json`

### What gets added to the frontend:
- **npm packages:** `jspdf` + `svg2pdf.js` (for embedding the existing SVG chart)
- **New component:** `src/lib/pdf-export.ts` — a module that:
  1. Takes the current scores, demographics, and chart SVG element as input
  2. Creates a jsPDF document with professional layout
  3. Embeds the actual rendered SVG chart (pixel-perfect match to screen)
  4. Returns PDF as a `Uint8Array`
- **Save flow:** Use `@tauri-apps/plugin-dialog` for save path + `@tauri-apps/plugin-fs` to write the bytes
- **Error handling:** All in JS with try/catch — errors displayed via DaisyUI toast/alert

### PDF Layout (generated by jsPDF):
- **Header:** "Weaver Curve Report" + app version + generation date
- **Demographics section:** Gender, DOB, age, premature birth info if applicable
- **Clinical section:** Head circumference measurements (child, mother, father)
- **Scores table:** Child score (with corrected if premature), mother score, father score, parental average. Color-coded: green for normal, red for abnormal.
- **Chart:** The actual SVG from the browser, embedded via svg2pdf.js
- **Clinical interpretation:** Plain-text statement: "The child's head circumference z-score falls [within/outside] the expected range (±2 SD) relative to parental average."
- **Disclaimer footer:** "This report is generated by Weaver Curve App v{version}. It is intended as a clinical decision support tool and does not constitute a diagnosis."
- **Optional patient ID:** Blank line labeled "Patient ID: _______________" for clinician to fill in

### Benefits:
- Chart on screen === chart in PDF (no duplicate rendering)
- Standard JS error handling (no cross-runtime silent failures)
- Dramatically simpler dependency tree
- Faster Rust compile times
- Smaller binary size

---

## 11. Release Workflow: Add macOS x86 Target

- Add `macos-13` with `--target x86_64-apple-darwin` to the release matrix.
- Or: remove the x86 claim from README if Intel Mac support is intentionally dropped.
- Decision: **Add the target** (preserves existing user base).

---

## 12. Commit Cargo.lock

- `Cargo.lock` exists but may not be tracked. Ensure it's committed for reproducible builds.
- Verify `.gitignore` doesn't exclude it.

---

## 13. PDF UX Improvements

All PDF content and layout details are covered in section 9. This section covers the remaining UX changes:

- **Save dialog filter:** "PDF Document" instead of "My Filter".
- **Success feedback:** Replace blocking `alert()` with a DaisyUI toast notification that auto-dismisses after 3 seconds.
- **Error feedback:** Display errors in the same toast system with error styling. No more silent failures.
- **Print button:** Only enabled when scores are calculated. Disabled state with tooltip: "Calculate scores first".

---

## 14. Library Upgrades

### Frontend (npm)
- Audit with `npm outdated` and upgrade to latest compatible versions.
- Key targets: SvelteKit, Vite, Vitest, Tauri API packages, d3.
- **Remove:** `chart.js` (unused — the app uses d3 for plotting, chart.js was never imported).
- **Add:** `jspdf`, `svg2pdf.js` (for frontend PDF generation).
- Run `npm audit fix` for security patches.

### Backend (Cargo)
- **Remove:** `typst`, `typst-pdf`, `typst-as-lib`, `derive_typst_intoval`, `charming`, `tempfile` (all related to the removed PDF pipeline).
- **Keep and upgrade:** `tauri` 2.x (latest patch), `serde`/`serde_json`, `interp`, `thiserror` (1 → 2), `tauri-plugin-*` packages.
- The Cargo dependency list drops from 12 entries to 6, significantly simplifying the build.

---

## 15. Svelte 5 Migration

**Problem:** The app uses Svelte 4 patterns (`$:` reactive statements, `export let` props, `on:click` event handlers). Svelte 5 introduces runes (`$state`, `$derived`, `$effect`, `$props`) and new event syntax that are the future of the framework.

### Migration Surface (small — 4 component files)

| Svelte 4 Pattern | Count | Svelte 5 Replacement |
|---|---|---|
| `$:` reactive statements | 17 | `$derived()` for computed values, `$effect()` for side effects |
| `export let` props | ~22 | `let { prop1, prop2 } = $props()` |
| `on:click`, `on:input` | ~5 | `onclick`, `oninput` (lowercase, no colon) |
| `bind:value` | ~8 | `bind:value` (unchanged) |
| `bind:this` | 2 | `bind:this` (unchanged) |
| `<svelte:window bind:...>` | 1 | Same syntax, works in Svelte 5 |

### Design

**Package upgrades:**
- `svelte` 4.x → 5.x
- `@sveltejs/kit` 2.x → latest 2.x (Svelte 5 compatible)
- `@sveltejs/vite-plugin-svelte` 3.x → 4.x or 5.x (Svelte 5 compatible)
- `svelte-check` 3.x → 4.x (Svelte 5 compatible)
- `@testing-library/svelte` 5.x → latest (Svelte 5 compatible)

**Per-component migration:**

1. **`+page.svelte`** (main form):
   - `let` variables → `let x = $state(0)` for mutable state
   - `$: gender_id = ...` → `let gender_id = $derived(...)` (then removed per #3)
   - `$: outerWidth = 0` etc → removed (unused per #5)
   - `on:click={process_form}` → `onclick={process_form}`
   - `on:input={update_child_age_in_months}` → `oninput={update_child_age_in_months}`

2. **`score_card.svelte`**:
   - `export let show_score` etc → `let { show_score, ... } = $props()`
   - Add `<script lang="ts">` (currently missing lang attribute)
   - `$: parental_average = ...` → `let parental_average = $derived(...)`

3. **`weaver_plot.svelte`**:
   - Same prop/reactive migration as score_card
   - `$: d3.select(gy).call(...)` → `$effect(() => { d3.select(gy).call(...) })`
   - The d3 axis bindings are side effects (DOM manipulation), so `$effect` is correct here

4. **`Tick.svelte`**:
   - Simple prop migration. If unused, delete entirely.

### Ordering Consideration
Svelte 5 migration should happen **after** library upgrades but **before** the other frontend refactors, so all subsequent component work uses Svelte 5 patterns from the start.

---

## 16. Testing Strategy

### Unit Tests (Frontend)
- Rewrite `utils.test.ts` to test actual exported functions from `constants.ts` and any utility modules.
- Add component tests for `score_card.svelte` and `weaver_plot.svelte` using `@testing-library/svelte`.
  - Score card: renders correct scores, shows/hides corrected score, displays warning for premature age.
  - Weaver plot: renders SVG with correct data points, legend visibility.
- Remove tests that assert literal math.

### Unit Tests (Backend)
- Existing Rust tests are decent. Add:
  - Edge case tests: age = 0, age = 216 (max), age beyond table range.
  - Validation error tests: negative values, out-of-range inputs.
  - Gender enum deserialization tests.

### E2E Tests
**Framework:** Playwright

**The Backend Problem & Solution:**

The frontend calls `invoke("calculate_scores", ...)` which requires the Tauri Rust backend. Without it, form submission does nothing. Two strategies:

**Strategy A — Mock the Tauri API (for CI/fast tests):**
- Create `src/lib/tauri-mock.ts` that intercepts `@tauri-apps/api/core` `invoke` calls during test mode.
- The mock implements `calculate_scores` using the same z-score math (imported from `constants.ts`) — this is simple arithmetic, not complex business logic.
- Playwright tests run against `vite dev` with the mock active (via `VITE_MOCK_TAURI=true` env var).
- Covers: form interaction, validation, score display, chart rendering, DOB calculation, reset behavior.

**Strategy B — Full Tauri E2E (for release validation, future):**
- Use `cargo tauri dev` to launch the full app, Playwright connects to the webview.
- Requires Rust toolchain + system deps. Runs locally or in a dedicated CI job with full Tauri build environment.
- Covers: actual Rust calculations, PDF export, file save dialog interaction.

**We implement Strategy A first** (runs in CI, fast, no Rust needed). Strategy B is documented as a follow-up for release validation.

**Setup:**
- Add `@playwright/test` as a devDependency.
- Create `e2e/` directory with test files.
- Create `src/lib/tauri-mock.ts` — mock implementation of `invoke` for `calculate_scores`.
- Create `src/lib/api.ts` — a thin wrapper around `invoke` that switches between real Tauri and mock based on `import.meta.env.VITE_MOCK_TAURI`.
- Add `playwright.config.ts` configured for Vite dev server with `VITE_MOCK_TAURI=true`.
- Add npm scripts: `test:e2e` and `test:e2e:ui`.

**Test scenarios (Strategy A):**
1. **Happy path**: Fill form with valid data → submit → verify score card shows numeric values → verify plot renders SVG with data point.
2. **Validation errors**: Submit with empty/zero fields → verify all error messages appear. Fix one field → resubmit → verify that field's error clears.
3. **Premature birth**: Fill premature fields → submit → verify corrected score appears, warning displays for young premature infants.
4. **Gender selection**: Verify male/female works, default placeholder state.
5. **Reset**: Fill form → calculate → reset → verify all fields cleared, scores hidden.
6. **DOB calculation**: Enter a DOB → verify age in months auto-calculates.
7. **PDF button state**: Verify print button appears only after score calculation.
8. **Abnormal score display**: Enter values that produce abnormal score → verify red highlighting in score card and plot.

### CI Integration
- Add E2E test job to `ci.yml` using Strategy A (Vite dev server + mocked Tauri, no Rust needed).
- Keep existing frontend unit test + backend Rust test jobs.
- Document Strategy B as a manual pre-release checklist.

---

## Implementation Order

The work is ordered to minimize merge conflicts and maximize safety:

1. **Branch creation + library upgrades** (npm + Cargo) — foundation first
2. **Svelte 5 migration** — before other frontend work so all new code uses runes
3. **Rust refactor** — constants module, static tables, Gender enum, remove PDF pipeline, error handling on `calculate_scores`
4. **Frontend constants + remove dead code + fix typo**
5. **Frontend PDF generation** — jspdf + svg2pdf.js, replace Rust PDF pipeline
6. **Frontend validation + error handling on invoke + toast notifications**
7. **Testing** — unit tests (frontend + backend), E2E setup with Tauri mock + Playwright tests
8. **CI/CD fixes** — release workflow (macOS x86), Cargo.lock, E2E in CI
9. **Final verification** — build all, run all tests, review

---

## Out of Scope

- Adding new clinical features (e.g., growth tracking over time)
- Internationalization
- Custom clinic branding (future feature)
