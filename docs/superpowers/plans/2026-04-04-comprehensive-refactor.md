# Comprehensive Refactor & PDF Overhaul Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix all 12 review issues, migrate to Svelte 5, replace the Rust PDF pipeline with frontend jsPDF, add comprehensive testing including E2E with Playwright.

**Architecture:** The Rust backend is slimmed down to only `calculate_scores` (medical math). PDF generation moves entirely to the frontend using jsPDF with the existing SVG chart. A Tauri API abstraction layer (`src/lib/api.ts`) enables E2E testing with mocked backends. All components migrate to Svelte 5 runes.

**Tech Stack:** Svelte 5, SvelteKit, Tauri 2, Rust, jsPDF, Playwright, Vitest, DaisyUI/Tailwind

**Spec:** `docs/superpowers/specs/2026-04-04-comprehensive-refactor-design.md`

---

## Task 1: Create Branch and Upgrade Frontend Dependencies

**Files:**
- Modify: `package.json`
- Modify: `package-lock.json`
- Modify: `svelte.config.js`
- Modify: `vite.config.js`
- Modify: `vitest.config.js`

- [ ] **Step 1: Create feature branch**

```bash
git checkout -b refactor/comprehensive-overhaul
```

- [ ] **Step 2: Check current outdated packages**

```bash
npm outdated
```

- [ ] **Step 3: Upgrade Svelte ecosystem to v5**

```bash
npm install svelte@^5 @sveltejs/kit@latest @sveltejs/vite-plugin-svelte@latest @sveltejs/adapter-static@latest svelte-check@^4
```

- [ ] **Step 4: Upgrade remaining dev dependencies**

```bash
npm install -D vite@latest vitest@latest @vitest/ui@latest @testing-library/svelte@latest @testing-library/jest-dom@latest @testing-library/user-event@latest typescript@latest postcss@latest autoprefixer@latest tailwindcss@latest jsdom@latest tslib@latest
```

- [ ] **Step 5: Remove unused chart.js, add jsPDF**

```bash
npm uninstall chart.js
npm install jspdf
```

- [ ] **Step 6: Add Playwright**

```bash
npm install -D @playwright/test
npx playwright install chromium
```

- [ ] **Step 7: Upgrade Tauri packages**

```bash
npm install @tauri-apps/api@latest @tauri-apps/plugin-dialog@latest @tauri-apps/plugin-fs@latest @tauri-apps/plugin-shell@latest @tauri-apps/cli@latest
```

- [ ] **Step 8: Update svelte.config.js for Svelte 5**

The `vitePreprocess` import path changes in Svelte 5:

```js
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter(),
  },
};

export default config;
```

Note: If the import already works (some versions re-export from the same path), leave it as-is.

- [ ] **Step 9: Verify frontend builds**

```bash
npm run build
```

Expected: Build succeeds. There may be Svelte 5 deprecation warnings for the old syntax — that's expected and will be fixed in Task 3.

- [ ] **Step 10: Commit**

```bash
git add package.json package-lock.json svelte.config.js vite.config.js vitest.config.js
git commit -m "chore: upgrade frontend deps — Svelte 5, Vite, Vitest, Playwright, jsPDF"
```

---

## Task 2: Upgrade Rust Dependencies and Remove PDF Pipeline Crates

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Modify: `src-tauri/Cargo.lock`

- [ ] **Step 1: Update Cargo.toml — remove PDF crates, upgrade remaining**

Replace the entire `[dependencies]` section in `src-tauri/Cargo.toml`:

```toml
[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-shell = "2"
tauri-plugin-dialog = "2"
tauri-plugin-fs = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
interp = "1.0.3"
thiserror = "2"
```

Removed: `typst`, `typst-pdf`, `typst-as-lib`, `derive_typst_intoval`, `charming`, `tempfile`.

- [ ] **Step 2: Verify Cargo.lock updates**

```bash
cd src-tauri && cargo check 2>&1 | head -50
```

Expected: Will fail because `main.rs` still references removed crates. That's expected — we'll fix `main.rs` in Task 4.

- [ ] **Step 3: Commit Cargo.toml change**

```bash
git add src-tauri/Cargo.toml
git commit -m "chore: remove typst/charming PDF crates, upgrade thiserror to v2"
```

---

## Task 3: Svelte 5 Migration — All Components

**Files:**
- Modify: `src/routes/+page.svelte`
- Modify: `src/components/score_card.svelte`
- Modify: `src/components/weaver_plot.svelte`
- Delete: `src/components/Tick.svelte` (unused — not imported anywhere)

- [ ] **Step 1: Delete unused Tick.svelte**

Verify it's unused:

```bash
grep -r "Tick" src/ --include="*.svelte" --include="*.ts"
```

Expected: No imports found. Delete the file.

```bash
rm src/components/Tick.svelte
```

- [ ] **Step 2: Migrate +page.svelte to Svelte 5 runes**

Replace the entire `<script lang="ts">` block in `src/routes/+page.svelte`:

```svelte
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import ScoreCard from "../components/score_card.svelte";
  import WeaverPlot from "../components/weaver_plot.svelte";

  let date: string = $state("");
  let mother_circumference_in_cm = $state(0);
  let father_circumference_in_cm = $state(0);
  let child_head_circumference_in_cm = $state(0);
  let child_age_in_months = $state(0);
  let premature_conception_in_days = $state(0);
  let premature_conception_in_weeks = $state(0);
  let selected_gender: string = $state("");

  let error_selected_gender = $state(false);
  let error_child_age_in_months = $state(false);
  let error_child_head_circumference_in_cm = $state(false);
  let error_mother_circumference_in_cm = $state(false);
  let error_father_circumference_in_cm = $state(false);
  let error_premature_conception_in_days = $state(false);
  let error_premature_conception_in_weeks = $state(false);

  let error = $state("");

  let show_scores = $state(false);
  let show_corrected_scores = $state(false);
  let child_score = $state(0);
  let corrected_child_score = $state(0);
  let mother_score = $state(0);
  let father_score = $state(0);

  let innerWidth = $state(0);
  let innerHeight = $state(0);

  let gender_id = $derived(selected_gender === "male" ? 0 : 1);

  function process_form() {
    console.log("Processing form");

    // Clear previous errors
    error_selected_gender = false;
    error_child_age_in_months = false;
    error_child_head_circumference_in_cm = false;
    error_mother_circumference_in_cm = false;
    error_father_circumference_in_cm = false;
    error_premature_conception_in_days = false;
    error_premature_conception_in_weeks = false;
    error = "";

    let has_error = false;
    if (selected_gender.length === 0) {
      error_selected_gender = true;
      has_error = true;
    }

    if (child_age_in_months <= 0 || child_age_in_months > 216) {
      error_child_age_in_months = true;
      has_error = true;
    }

    if (child_head_circumference_in_cm <= 0 || child_head_circumference_in_cm > 70) {
      error_child_head_circumference_in_cm = true;
      has_error = true;
    }

    if (mother_circumference_in_cm <= 0 || mother_circumference_in_cm > 70) {
      error_mother_circumference_in_cm = true;
      has_error = true;
    }

    if (father_circumference_in_cm <= 0 || father_circumference_in_cm > 70) {
      error_father_circumference_in_cm = true;
      has_error = true;
    }

    if (premature_conception_in_days > 7 || premature_conception_in_days < 0) {
      error_premature_conception_in_days = true;
      has_error = true;
    }

    if (premature_conception_in_weeks > 42 || premature_conception_in_weeks < 0) {
      error_premature_conception_in_weeks = true;
      has_error = true;
    }

    if (has_error) {
      return;
    }

    invoke("calculate_scores", {
      childAgeMonths: child_age_in_months,
      childHeadCircumferenceCm: child_head_circumference_in_cm,
      motherCircumferenceCm: mother_circumference_in_cm,
      fatherCircumferenceCm: father_circumference_in_cm,
      prematureConceptionWeeks: premature_conception_in_weeks,
      prematureConceptionDays: premature_conception_in_days,
      gender: selected_gender,
    }).then((res: any) => {
      show_scores = true;
      show_corrected_scores =
        premature_conception_in_days > 0 || premature_conception_in_weeks > 0;
      father_score = res[0];
      mother_score = res[1];
      child_score = res[2];
      corrected_child_score = res[3];
    }).catch((err: any) => {
      error = `Calculation failed: ${err}`;
      console.error("invoke error:", err);
    });
  }

  function update_child_age_in_months() {
    if (!date) return;
    const child_dob = new Date(date + "T00:00:00");
    const today = new Date();
    let months = (today.getFullYear() - child_dob.getFullYear()) * 12 + (today.getMonth() - child_dob.getMonth());
    if (today.getDate() - child_dob.getDate() < 0) {
      months -= 1;
    }
    child_age_in_months = months;
  }

  function reset_form() {
    show_scores = false;
    show_corrected_scores = false;
    selected_gender = "";
    error_selected_gender = false;
    error_child_age_in_months = false;
    error_child_head_circumference_in_cm = false;
    error_mother_circumference_in_cm = false;
    error_father_circumference_in_cm = false;
    error_premature_conception_in_days = false;
    error_premature_conception_in_weeks = false;
    error = "";
    child_age_in_months = 0;
    child_head_circumference_in_cm = 0;
    mother_circumference_in_cm = 0;
    father_circumference_in_cm = 0;
    premature_conception_in_weeks = 0;
    premature_conception_in_days = 0;
    date = "";
  }
</script>
```

Update the template section — replace `on:click` with `onclick`, `on:input` with `oninput`, remove unused `outerWidth`/`outerHeight` bindings, add error banner:

```svelte
<svelte:window bind:innerWidth bind:innerHeight />
<div class="flex flex-col justify-center">
  {#if error}
    <div class="alert alert-error place-self-center w-5/6 mt-3">
      <span>{error}</span>
    </div>
  {/if}
  <div class="card bg-neutral text-neutral-content place-self-center w-5/6 mt-3">
    <div class="card-body items-center text-center">
      <h2 class="card-title">Weaver Curve</h2>
```

Replace the two `on:input` and `on:click` occurrences in the template:

- `on:input={update_child_age_in_months}` → `oninput={update_child_age_in_months}`
- `on:click={process_form}` → `onclick={process_form}`
- `on:click={reset_form}` → `onclick={reset_form}`

Update the ScoreCard usage — fix the typo `gestiational_age_in_weeks` → `gestational_age_in_weeks`:

```svelte
  <ScoreCard show_score={show_scores} show_corrected_score={show_corrected_scores} child_score={child_score} correct_score={corrected_child_score} mother_score={mother_score} father_score={father_score} child_age_in_months={child_age_in_months} gestational_age_in_weeks={premature_conception_in_weeks}/>
```

Update the WeaverPlot usage — remove `gender={gender_id}` and pass `gender={selected_gender}`:

```svelte
  <WeaverPlot show_score={show_scores} show_corrected_score={show_corrected_scores} child_score={child_score} correct_score={corrected_child_score} mother_score={mother_score} father_score={father_score} chartWidth={innerWidth * 0.8} chartHeight={innerWidth * 0.8 * 0.78} child_age_in_months={child_age_in_months} premature_conception_in_weeks={premature_conception_in_weeks} premature_conception_in_days={premature_conception_in_days} gender={selected_gender} mother_circumference_in_cm={mother_circumference_in_cm} father_circumference_in_cm={father_circumference_in_cm} child_head_circumference_in_cm={child_head_circumference_in_cm} child_dob={date}/>
```

- [ ] **Step 3: Migrate score_card.svelte to Svelte 5 runes**

Replace the entire file content of `src/components/score_card.svelte`:

```svelte
<script lang="ts">
    import { INTERCEPT, SLOPE, SD_THRESHOLD } from '$lib/constants';

    let {
        show_score = false,
        show_corrected_score = false,
        child_score,
        correct_score,
        mother_score,
        father_score,
        child_age_in_months,
        gestational_age_in_weeks = 0,
    }: {
        show_score: boolean;
        show_corrected_score: boolean;
        child_score: number;
        correct_score: number;
        mother_score: number;
        father_score: number;
        child_age_in_months: number;
        gestational_age_in_weeks?: number;
    } = $props();

    let parental_average = $derived((mother_score + father_score) / 2);
    let y_mean = $derived(INTERCEPT + SLOPE * parental_average);
    let is_abnormal = $derived(child_score > y_mean + SD_THRESHOLD || child_score < y_mean - SD_THRESHOLD);
    let is_abnormal_corrected = $derived(correct_score > y_mean + SD_THRESHOLD || correct_score < y_mean - SD_THRESHOLD);
    let is_invalid = $derived(gestational_age_in_weeks > 0 && child_age_in_months < (40 - gestational_age_in_weeks) / 4.345);
</script>

<div class="card bg-neutral text-neutral-content place-self-center w-5/6 mt-3">
    <div class="card-body items-center text-center">
        <h2 class="card-title">Weaver Scores</h2>
        <div class="flex flex-col justify-center">
          {#if show_score}
            <div class="flex flex-row justify-center">
              <div class="stats shadow">
                  {#if show_corrected_score}
                  <div class="stat place-items-center">
                    <div class="stat-title">Child (corrected)</div>
                    <div class="stat-value {is_abnormal_corrected ? 'text-error' : 'text-success'}">{correct_score.toFixed(2)}</div>
                    <div class="stat-desc {is_abnormal ? 'text-error' : 'text-success'}">Original score: {child_score.toFixed(2)}</div>
                  </div>
                  {:else}
                  <div class="stat place-items-center">
                      <div class="stat-title">Child</div>
                      <div class="stat-value {is_abnormal ? 'text-error' : 'text-success'}">{child_score.toFixed(2)}</div>
                    </div>
                  {/if}
                  <div class="divider divider-vertical"></div>
                  <div class="stat place-items-center">
                    <div class="stat-title">Parental Average</div>
                    <div class="stat-value">{parental_average.toFixed(2)}</div>
                    <div class="stat-desc text-secondary">M:{mother_score.toFixed(2)} | F:{father_score.toFixed(2)}</div>
                  </div>
              </div>
            </div>
          {:else}
          <div class="flex flex-row"></div>
            <div class="stats shadow">
                <div class="stat place-items-center">
                    <div class="stat-title">Child</div>
                    <div class="skeleton h-8 w-28"></div>
                </div>

                <div class="stat place-items-center">
                  <div class="stat-title">Parental Average</div>
                  <div class="skeleton h-8 w-28"></div>
                </div>
            </div>
          {/if}
          {#if is_invalid}
          <div class="flex flex-row">
            <div class="bg-orange-100 border-l-4 border-orange-500 text-orange-700 p-4 rounded-box mt-3 border-t-4" role="alert">
              <p class="font-bold">Warning</p>
              <p>Results may not be accurate for current premature age.</p>
            </div>
          </div>
          {/if}
        </div>
    </div>
</div>
```

- [ ] **Step 4: Create src/lib/constants.ts**

```bash
mkdir -p src/lib
```

Create `src/lib/constants.ts`:

```ts
/** Regression intercept for the Weaver curve parental-average-to-child-score model */
export const INTERCEPT = 0.138891;

/** Regression slope for the Weaver curve parental-average-to-child-score model */
export const SLOPE = 0.483034;

/** Standard deviation threshold — scores outside ±SD_THRESHOLD from expected are abnormal */
export const SD_THRESHOLD = 2;

/** Maximum supported child age in months (18 years) */
export const MAX_AGE_MONTHS = 216;

/** Maximum reasonable head circumference in cm */
export const MAX_HEAD_CIRCUMFERENCE_CM = 70;

/** Maximum premature conception weeks (full term) */
export const MAX_PREMATURE_WEEKS = 42;

/** Maximum premature conception days */
export const MAX_PREMATURE_DAYS = 7;
```

- [ ] **Step 5: Migrate weaver_plot.svelte to Svelte 5 runes**

Replace the entire file content of `src/components/weaver_plot.svelte`:

```svelte
<script lang="ts">
    import { select } from "d3-selection";
    import { axisLeft, axisBottom } from "d3-axis";
    import { scaleLinear } from "d3-scale";
    import { save } from '@tauri-apps/plugin-dialog';
    import { writeFile } from '@tauri-apps/plugin-fs';
    import { getVersion } from '@tauri-apps/api/app';
    import { INTERCEPT, SLOPE, SD_THRESHOLD } from '$lib/constants';
    import { generatePdf } from '$lib/pdf-export';

    let {
        show_score = true,
        show_corrected_score = false,
        child_score,
        correct_score,
        mother_score,
        father_score,
        child_dob,
        child_age_in_months = 0,
        mother_circumference_in_cm = 0,
        father_circumference_in_cm = 0,
        child_head_circumference_in_cm = 0,
        premature_conception_in_days = 0,
        premature_conception_in_weeks = 0,
        gender = "",
        chartWidth = 700,
        chartHeight = 700,
    }: {
        show_score?: boolean;
        show_corrected_score?: boolean;
        child_score: number;
        correct_score: number;
        mother_score: number;
        father_score: number;
        child_dob?: string | null;
        child_age_in_months?: number;
        mother_circumference_in_cm?: number;
        father_circumference_in_cm?: number;
        child_head_circumference_in_cm?: number;
        premature_conception_in_days?: number;
        premature_conception_in_weeks?: number;
        gender?: string;
        chartWidth?: number;
        chartHeight?: number;
    } = $props();

    const paddings = {
        top: 20,
        left: 60,
        right: 20,
        bottom: 50,
    };

    let gy: SVGGElement;
    let gx: SVGGElement;
    let svgElement: SVGSVGElement;

    let xScale = $derived(scaleLinear().domain([-5, 5]).range([paddings.left, chartWidth - paddings.right]));
    let yScale = $derived(scaleLinear().domain([5, -5]).range([paddings.top, chartHeight - paddings.bottom]));

    let parental_average = $derived((mother_score + father_score) / 2);
    let y_mean = $derived(INTERCEPT + SLOPE * parental_average);
    let is_abnormal = $derived(child_score > y_mean + SD_THRESHOLD || child_score < y_mean - SD_THRESHOLD);
    let is_abnormal_corrected = $derived(correct_score > y_mean + SD_THRESHOLD || correct_score < y_mean - SD_THRESHOLD);

    $effect(() => {
        if (gy) select(gy).call(axisLeft(yScale));
    });

    $effect(() => {
        if (gx) select(gx).call(axisBottom(xScale));
    });

    let toast_message = $state("");
    let toast_type: "success" | "error" = $state("success");
    let show_toast = $state(false);

    function show_notification(message: string, type: "success" | "error") {
        toast_message = message;
        toast_type = type;
        show_toast = true;
        setTimeout(() => { show_toast = false; }, 3000);
    }

    async function print_to_pdf() {
        try {
            const path = await save({
                filters: [{ name: 'PDF Document', extensions: ['pdf'] }],
            });
            if (!path) return;

            const appVersion = await getVersion();
            const svgData = new XMLSerializer().serializeToString(svgElement);

            const pdfBytes = generatePdf({
                svgData,
                svgWidth: chartWidth,
                svgHeight: chartHeight,
                appVersion,
                gender: gender || "Unknown",
                childDob: child_dob || "Not provided",
                childAgeMonths: child_age_in_months,
                childHeadCircumferenceCm: child_head_circumference_in_cm,
                motherCircumferenceCm: mother_circumference_in_cm,
                fatherCircumferenceCm: father_circumference_in_cm,
                prematureConceptionWeeks: premature_conception_in_weeks,
                prematureConceptionDays: premature_conception_in_days,
                childScore: child_score,
                correctedChildScore: correct_score,
                motherScore: mother_score,
                fatherScore: father_score,
                showCorrectedScore: show_corrected_score,
                isAbnormal: is_abnormal,
                isAbnormalCorrected: is_abnormal_corrected,
            });

            await writeFile(path, pdfBytes, {});
            show_notification(`PDF saved to ${path}`, "success");
        } catch (err) {
            console.error("PDF export error:", err);
            show_notification(`Failed to save PDF: ${err}`, "error");
        }
    }
</script>

<div class="card bg-neutral text-neutral-content place-self-center w-5/6 mt-3">
    <div class="card-body items-center text-center">
        <h2 class="card-title">Weaver Plot</h2>
        {#if show_score}
            <svg bind:this={svgElement} width={chartWidth} height={chartHeight}>
                <g>
                    {#each {length: 10} as _, x}
                        <line
                            x1={xScale(x-5)}
                            x2={xScale(x-4)}
                            y1={yScale(INTERCEPT + (x-5) * SLOPE)}
                            y2={yScale(INTERCEPT + (x-4) * SLOPE)}
                            stroke="blue"
                            stroke-width="2"
                        />
                        <line
                            x1={xScale(x-5)}
                            x2={xScale(x-4)}
                            y1={yScale(INTERCEPT + (x-5) * SLOPE + SD_THRESHOLD)}
                            y2={yScale(INTERCEPT + (x-4) * SLOPE + SD_THRESHOLD)}
                            stroke="orange"
                            style="stroke-dasharray: 3, 3;"
                            stroke-width="1"
                        />
                        <line
                            x1={xScale(x-5)}
                            x2={xScale(x-4)}
                            y1={yScale(INTERCEPT + (x-5) * SLOPE - SD_THRESHOLD)}
                            y2={yScale(INTERCEPT + (x-4) * SLOPE - SD_THRESHOLD)}
                            stroke="orange"
                            style="stroke-dasharray: 3, 3;"
                            stroke-width="1"
                        />
                    {/each}
                </g>

                <g bind:this={gx} transform="translate(0,{chartHeight - 25 < yScale(-5) ? chartHeight-25 : yScale(-5)})"></g>
                <g>
                    <text x={xScale(0)} y={chartHeight - 15 < yScale(-5) - 40 ? chartHeight-15 : yScale(-5) + 40} fill="white" text-anchor="middle">Standard Score (Parental Average)</text>
                </g>

                <g bind:this={gy} transform="translate({paddings.left},0)"></g>
                <g>
                    <text x={-yScale(0)} y={paddings.left / 2} transform="rotate(-90)" fill="white" text-anchor="middle"> Standard Score (Child)</text>
                </g>
                <g>
                    <circle
                      cx={xScale(parental_average)}
                      cy={yScale(child_score)}
                      r={5}
                      style="fill: {is_abnormal ? 'red' : 'green'}"
                    />
                </g>

                {#if show_corrected_score}
                    <g>
                      <rect
                        x={xScale(parental_average) - 10}
                        y={yScale(correct_score) - 10}
                        width=20
                        height=20
                        style="fill: {is_abnormal_corrected ? 'red' : 'green'}"
                      />
                    </g>
                {/if}

                <!-- Grid Lines -->
                {#each {length: 10} as _, x}
                    <g>
                      <line x1={xScale(x-5)} x2={xScale(x-5)} y1={yScale(5)} y2={yScale(-5)} stroke="gray" stroke-width="0.5" />
                    </g>
                {/each}
                {#each {length: 10} as _, y}
                    <g>
                      <line x1={xScale(-5)} x2={xScale(5)} y1={yScale(y-5)} y2={yScale(y-5)} stroke="gray" stroke-width="0.5" />
                    </g>
                {/each}

                <!-- Legend -->
                <g>
                  <rect x={xScale(-5) + 5} y={yScale(5) + 5} width={show_corrected_score ? 230 : 120} height={show_corrected_score ? 100 : 80} style="fill: grey" stroke="black" />
                </g>
                <g>
                    <line x1={xScale(-5) + 15} x2={xScale(-5) + 30} y1={yScale(5) + 25} y2={yScale(5) + 25} stroke="blue" stroke-width="2" />
                    <text x={xScale(-5) + 35} y={yScale(5) + 30} fill="white" text-anchor="start">Mean</text>
                </g>
                <g>
                    <line x1={xScale(-5) + 15} x2={xScale(-5) + 30} y1={yScale(5) + 45} y2={yScale(5) + 45} stroke="orange" stroke-width="1" style="stroke-dasharray: 3, 3;" />
                    <text x={xScale(-5) + 35} y={yScale(5) + 50} fill="white" text-anchor="start">± 2 SD</text>
                </g>
                <g>
                  <circle cx={xScale(-5) + 22} cy={yScale(5) + 65} r={3} style="fill: {is_abnormal ? 'red' : 'green'}" />
                  <text x={xScale(-5) + 35} y={yScale(5) + 70} fill="white" text-anchor="start">Child Score</text>
                </g>
                {#if show_corrected_score}
                  <g>
                    <rect x={xScale(-5) + 17} y={yScale(5) + 80} width=10 height=10 style="fill: {is_abnormal_corrected ? 'red' : 'green'}" />
                    <text x={xScale(-5) + 35} y={yScale(5) + 90} fill="white" text-anchor="start">Child Score (GA Corrected)</text>
                  </g>
                {/if}
            </svg>
          <button class="btn btn-primary mt-4" onclick={print_to_pdf}>Print</button>
        {:else}
            <div class="skeleton h-96 w-96"></div>
        {/if}
    </div>
</div>

{#if show_toast}
<div class="toast toast-end">
  <div class="alert {toast_type === 'success' ? 'alert-success' : 'alert-error'}">
    <span>{toast_message}</span>
  </div>
</div>
{/if}
```

- [ ] **Step 6: Verify the build compiles (frontend only)**

```bash
npm run build 2>&1 | tail -20
```

Expected: May fail because `$lib/constants` and `$lib/pdf-export` don't exist yet. If so, that's expected — we created `constants.ts` in step 4 but `pdf-export.ts` comes in Task 6. Create a stub for now:

```bash
cat > src/lib/pdf-export.ts << 'EOF'
// Stub — implemented in Task 6
export function generatePdf(_opts: any): Uint8Array {
  throw new Error("PDF export not yet implemented");
}
EOF
```

```bash
npm run build
```

Expected: Build succeeds.

- [ ] **Step 7: Commit**

```bash
git add -A
git commit -m "feat: migrate all components to Svelte 5 runes, centralize constants, fix typos"
```

---

## Task 4: Rust Backend Refactor — Constants, Gender Enum, Remove PDF Code

**Files:**
- Create: `src-tauri/src/constants.rs`
- Modify: `src-tauri/src/main.rs`
- Delete: `src-tauri/resources/templates/template.typ`
- Delete: `src-tauri/resources/fonts/Roboto/` (all files except LICENSE.txt)
- Modify: `src-tauri/tauri.conf.json`

- [ ] **Step 1: Create src-tauri/src/constants.rs**

```rust
/// Regression intercept for the Weaver curve parental-average-to-child-score model
pub const INTERCEPT: f64 = 0.138891;

/// Regression slope for the Weaver curve parental-average-to-child-score model
pub const SLOPE: f64 = 0.483034;

/// Standard deviation threshold — scores outside ±SD_THRESHOLD from expected are abnormal
pub const SD_THRESHOLD: f64 = 2.0;

/// Maximum supported child age in months (18 years)
pub const MAX_AGE_MONTHS: u32 = 216;

/// Maximum reasonable head circumference in cm
pub const MAX_HEAD_CIRCUMFERENCE_CM: f32 = 70.0;

// Adult reference values for parental score calculation
pub const ADULT_MEAN_MALE: f64 = 55.95;
pub const ADULT_STD_MALE: f64 = 1.34;
pub const ADULT_MEAN_FEMALE: f64 = 54.94;
pub const ADULT_STD_FEMALE: f64 = 1.40;

// Clinical lookup tables: age in months
pub const AGE_MONTHS: &[f64] = &[
    0.0, 1.0, 3.0, 6.0, 9.0, 12.0, 18.0, 24.0, 36.0, 48.0, 60.0, 72.0, 84.0, 96.0, 108.0,
    120.0, 132.0, 144.0, 156.0, 168.0, 180.0, 192.0, 204.0, 216.0,
];

// Male head circumference means by age
pub const MALE_HEAD_CIRCUMFERENCE: &[f64] = &[
    34.74, 37.30, 40.62, 43.76, 45.75, 47.00, 48.31, 49.19, 50.63, 50.91, 51.41, 51.40, 52.24,
    52.35, 52.58, 53.16, 53.25, 53.71, 54.14, 54.59, 54.95, 55.37, 55.77, 55.95,
];

// Male head circumference standard deviations by age
pub const MALE_HEAD_STD: &[f64] = &[
    1.33, 1.30, 1.23, 1.29, 1.28, 1.31, 1.36, 1.39, 1.38, 1.39, 1.37, 1.41, 1.52, 1.40, 1.44,
    1.41, 1.53, 1.52, 1.57, 1.30, 1.51, 1.11, 1.32, 1.34,
];

// Female head circumference means by age
pub const FEMALE_HEAD_CIRCUMFERENCE: &[f64] = &[
    34.02, 36.43, 39.71, 42.68, 44.69, 45.81, 47.27, 48.02, 49.25, 50.10, 50.55, 50.52, 51.46,
    51.64, 51.87, 52.15, 52.64, 53.01, 53.70, 54.04, 54.39, 54.64, 54.78, 54.94,
];

// Female head circumference standard deviations by age
pub const FEMALE_HEAD_STD: &[f64] = &[
    1.22, 1.22, 1.20, 1.38, 1.30, 1.29, 1.36, 1.29, 1.36, 1.37, 1.32, 1.31, 1.35, 1.44, 1.33,
    1.50, 1.39, 1.50, 1.37, 1.39, 1.34, 1.16, 1.35, 1.40,
];
```

- [ ] **Step 2: Rewrite src-tauri/src/main.rs**

Replace the entire file:

```rust
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod constants;

use constants::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Gender {
    Male,
    Female,
}

#[tauri::command]
fn calculate_scores(
    child_age_months: u32,
    child_head_circumference_cm: f32,
    mother_circumference_cm: f32,
    father_circumference_cm: f32,
    premature_conception_weeks: u32,
    premature_conception_days: u32,
    gender: Gender,
) -> Result<(f64, f64, f64, f64), Error> {
    // Validate inputs
    if child_age_months > MAX_AGE_MONTHS {
        return Err(Error::Validation(format!(
            "Child age {} months exceeds maximum of {} months",
            child_age_months, MAX_AGE_MONTHS
        )));
    }
    if child_head_circumference_cm <= 0.0 || child_head_circumference_cm > MAX_HEAD_CIRCUMFERENCE_CM {
        return Err(Error::Validation(format!(
            "Child head circumference {} cm is out of valid range (0-{} cm)",
            child_head_circumference_cm, MAX_HEAD_CIRCUMFERENCE_CM
        )));
    }
    if mother_circumference_cm <= 0.0 || mother_circumference_cm > MAX_HEAD_CIRCUMFERENCE_CM {
        return Err(Error::Validation("Mother head circumference is out of valid range".into()));
    }
    if father_circumference_cm <= 0.0 || father_circumference_cm > MAX_HEAD_CIRCUMFERENCE_CM {
        return Err(Error::Validation("Father head circumference is out of valid range".into()));
    }

    let corrected_age = get_corrected_age(
        child_age_months,
        premature_conception_weeks,
        premature_conception_days,
    );

    let (head_circumference, head_std) =
        get_head_circumference_data(child_age_months as f32, &gender);
    let (head_circumference_corrected, head_std_corrected) =
        get_head_circumference_data(corrected_age, &gender);

    let child_score = (child_head_circumference_cm as f64 - head_circumference) / head_std;
    let corrected_child_score =
        (child_head_circumference_cm as f64 - head_circumference_corrected) / head_std_corrected;

    let dad_score = (father_circumference_cm as f64 - ADULT_MEAN_MALE) / ADULT_STD_MALE;
    let mom_score = (mother_circumference_cm as f64 - ADULT_MEAN_FEMALE) / ADULT_STD_FEMALE;

    Ok((dad_score, mom_score, child_score, corrected_child_score))
}

fn get_corrected_age(
    child_age_months: u32,
    premature_conception_weeks: u32,
    premature_conception_days: u32,
) -> f32 {
    if premature_conception_weeks > 0 || premature_conception_days > 0 {
        let gest_age = (premature_conception_weeks as f32) + (premature_conception_days as f32) / 7.0;
        (child_age_months as f32) - (40.0 - gest_age) / 4.345
    } else {
        child_age_months as f32
    }
}

fn get_head_circumference_data(child_age_months: f32, gender: &Gender) -> (f64, f64) {
    let (circumference_table, std_table) = match gender {
        Gender::Male => (MALE_HEAD_CIRCUMFERENCE, MALE_HEAD_STD),
        Gender::Female => (FEMALE_HEAD_CIRCUMFERENCE, FEMALE_HEAD_STD),
    };

    let head_circumference = interp::interp(AGE_MONTHS, circumference_table, child_age_months as f64);
    let head_std = interp::interp(AGE_MONTHS, std_table, child_age_months as f64);

    (head_circumference, head_std)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![calculate_scores])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Validation error: {0}")]
    Validation(String),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_scores_basic() {
        let result = calculate_scores(12, 47.0, 54.0, 56.0, 0, 0, Gender::Male);
        assert!(result.is_ok());
        let (dad_score, mom_score, child_score, corrected_child_score) = result.unwrap();

        assert!(dad_score > -10.0 && dad_score < 10.0);
        assert!(mom_score > -10.0 && mom_score < 10.0);
        assert!(child_score > -10.0 && child_score < 10.0);
        assert_eq!(child_score, corrected_child_score);
    }

    #[test]
    fn test_calculate_scores_with_premature_birth() {
        let result = calculate_scores(12, 47.0, 54.0, 56.0, 4, 3, Gender::Male);
        assert!(result.is_ok());
        let (_, _, child_score, corrected_child_score) = result.unwrap();

        assert_ne!(child_score, corrected_child_score);
    }

    #[test]
    fn test_calculate_scores_female() {
        let result = calculate_scores(12, 45.0, 54.0, 56.0, 0, 0, Gender::Female);
        assert!(result.is_ok());
    }

    #[test]
    fn test_calculate_scores_validation_age_too_high() {
        let result = calculate_scores(300, 47.0, 54.0, 56.0, 0, 0, Gender::Male);
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_scores_validation_negative_circumference() {
        let result = calculate_scores(12, -5.0, 54.0, 56.0, 0, 0, Gender::Male);
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_scores_validation_circumference_too_high() {
        let result = calculate_scores(12, 100.0, 54.0, 56.0, 0, 0, Gender::Male);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_corrected_age_no_premature() {
        let corrected_age = get_corrected_age(12, 0, 0);
        assert_eq!(corrected_age, 12.0);
    }

    #[test]
    fn test_get_corrected_age_with_premature() {
        let corrected_age = get_corrected_age(12, 4, 3);
        assert!(corrected_age < 12.0);
        assert!(corrected_age > 0.0);
    }

    #[test]
    fn test_get_head_circumference_data_male() {
        let (hc, std) = get_head_circumference_data(12.0, &Gender::Male);
        assert!(hc > 40.0 && hc < 50.0);
        assert!(std > 1.0 && std < 2.0);
    }

    #[test]
    fn test_get_head_circumference_data_female() {
        let (hc, std) = get_head_circumference_data(12.0, &Gender::Female);
        assert!(hc > 40.0 && hc < 50.0);
        assert!(std > 1.0 && std < 2.0);
    }

    #[test]
    fn test_get_head_circumference_data_edge_age_zero() {
        let (hc, _) = get_head_circumference_data(0.0, &Gender::Male);
        assert!((hc - 34.74).abs() < 0.01);
    }

    #[test]
    fn test_get_head_circumference_data_edge_age_max() {
        let (hc, _) = get_head_circumference_data(216.0, &Gender::Male);
        assert!((hc - 55.95).abs() < 0.01);
    }

    #[test]
    fn test_parent_score_at_mean() {
        let result = calculate_scores(12, 47.0, 54.94, 55.95, 0, 0, Gender::Male);
        assert!(result.is_ok());
        let (dad_score, mom_score, _, _) = result.unwrap();
        assert!(dad_score.abs() < 0.1);
        assert!(mom_score.abs() < 0.1);
    }

    #[test]
    fn test_gender_deserialization() {
        let male: Gender = serde_json::from_str("\"male\"").unwrap();
        assert!(matches!(male, Gender::Male));
        let female: Gender = serde_json::from_str("\"female\"").unwrap();
        assert!(matches!(female, Gender::Female));
    }
}
```

- [ ] **Step 3: Remove resource files and update tauri.conf.json**

```bash
rm -rf src-tauri/resources/fonts/Roboto/
rm src-tauri/resources/templates/template.typ
rmdir src-tauri/resources/templates 2>/dev/null || true
rmdir src-tauri/resources/fonts 2>/dev/null || true
rmdir src-tauri/resources 2>/dev/null || true
```

Update `src-tauri/tauri.conf.json` — remove the `resources` key from `bundle`:

```json
{
  "productName": "weaver-curve-app",
  "version": "1.4.0",
  "identifier": "com.enmed.dev",
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../build"
  },
  "app": {
    "windows": [
      {
        "title": "weaver-curve-app",
        "width": 1024,
        "height": 720
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

Also add `fs:allow-write-file` to `src-tauri/capabilities/default.json` (needed for frontend PDF save via `@tauri-apps/plugin-fs`):

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    "core:path:default",
    "core:event:default",
    "core:window:default",
    "core:app:default",
    "core:image:default",
    "core:resources:default",
    "core:menu:default",
    "core:tray:default",
    "shell:allow-open",
    "dialog:allow-save",
    "dialog:allow-message",
    "fs:allow-write-text-file",
    "fs:default"
  ]
}
```

- [ ] **Step 4: Run Rust tests**

```bash
cd src-tauri && cargo test
```

Expected: All tests pass, including the new validation and gender deserialization tests.

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "feat: refactor Rust backend — Gender enum, constants module, static tables, remove PDF pipeline, add validation"
```

---

## Task 5: Update Frontend Test Setup for Svelte 5

**Files:**
- Modify: `src/test-setup.ts`
- Modify: `vitest.config.js`

- [ ] **Step 1: Update test-setup.ts for Svelte 5 compatibility**

Replace `src/test-setup.ts`:

```ts
import '@testing-library/jest-dom/vitest';
import { vi } from 'vitest';

// Mock Tauri API since tests run in Node.js environment
if (typeof globalThis.window === 'undefined') {
  (globalThis as any).window = {};
}
Object.assign(globalThis.window, {
  __TAURI_INTERNALS__: {},
});

// Mock Tauri functions
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

vi.mock('@tauri-apps/api/app', () => ({
  getVersion: vi.fn().mockResolvedValue('1.4.0'),
}));

vi.mock('@tauri-apps/plugin-dialog', () => ({
  save: vi.fn(),
}));

vi.mock('@tauri-apps/plugin-fs', () => ({
  writeFile: vi.fn(),
}));
```

- [ ] **Step 2: Run existing tests to verify setup**

```bash
npm run test:run
```

Expected: Tests pass (existing utils tests should still work).

- [ ] **Step 3: Commit**

```bash
git add src/test-setup.ts vitest.config.js
git commit -m "chore: update test setup for Svelte 5 and new Tauri plugin mocks"
```

---

## Task 6: Implement Frontend PDF Generation

**Files:**
- Create: `src/lib/pdf-export.ts` (replace stub from Task 3)

- [ ] **Step 1: Write the PDF export module**

Replace the stub in `src/lib/pdf-export.ts`:

```ts
import { jsPDF } from 'jspdf';
import { INTERCEPT, SLOPE, SD_THRESHOLD } from './constants';

export interface PdfExportOptions {
    svgData: string;
    svgWidth: number;
    svgHeight: number;
    appVersion: string;
    gender: string;
    childDob: string;
    childAgeMonths: number;
    childHeadCircumferenceCm: number;
    motherCircumferenceCm: number;
    fatherCircumferenceCm: number;
    prematureConceptionWeeks: number;
    prematureConceptionDays: number;
    childScore: number;
    correctedChildScore: number;
    motherScore: number;
    fatherScore: number;
    showCorrectedScore: boolean;
    isAbnormal: boolean;
    isAbnormalCorrected: boolean;
}

export function generatePdf(opts: PdfExportOptions): Uint8Array {
    const doc = new jsPDF({ orientation: 'portrait', unit: 'mm', format: 'a4' });
    const pageWidth = doc.internal.pageSize.getWidth();
    const margin = 15;
    let y = margin;

    // --- Header ---
    doc.setFontSize(8);
    doc.setTextColor(128);
    const today = new Date().toLocaleDateString('en-US', {
        weekday: 'long', year: 'numeric', month: 'short', day: 'numeric'
    });
    doc.text(`Weaver Curve App v${opts.appVersion}`, margin, y);
    doc.text(`Date Generated: ${today}`, pageWidth - margin, y, { align: 'right' });

    // --- Title ---
    y += 12;
    doc.setFontSize(18);
    doc.setTextColor(0);
    doc.text('Weaver Curve Report', pageWidth / 2, y, { align: 'center' });

    // --- Patient ID line ---
    y += 10;
    doc.setFontSize(10);
    doc.setTextColor(80);
    doc.text('Patient ID: _______________________________________________', margin, y);

    // --- Demographics ---
    y += 12;
    doc.setFontSize(13);
    doc.setTextColor(0);
    doc.text('Demographics', margin, y);
    y += 2;
    doc.setDrawColor(0);
    doc.line(margin, y, pageWidth - margin, y);
    y += 7;

    doc.setFontSize(10);
    doc.setTextColor(40);

    const demoLines: string[] = [
        `Gender: ${opts.gender.charAt(0).toUpperCase() + opts.gender.slice(1)}`,
        `DOB: ${opts.childDob}`,
        `Age: ${opts.childAgeMonths} months`,
    ];
    if (opts.prematureConceptionWeeks > 0 || opts.prematureConceptionDays > 0) {
        demoLines.push(`Premature Conception: ${opts.prematureConceptionWeeks} weeks, ${opts.prematureConceptionDays} days`);
        const gestAge = opts.prematureConceptionWeeks + opts.prematureConceptionDays / 7;
        const correctedAge = opts.childAgeMonths - (40 - gestAge) / 4.345;
        demoLines.push(`Corrected Age: ${correctedAge.toFixed(1)} months`);
    }
    for (const line of demoLines) {
        doc.text(`• ${line}`, margin + 2, y);
        y += 5.5;
    }

    // --- Clinical Measurements ---
    y += 5;
    doc.setFontSize(13);
    doc.setTextColor(0);
    doc.text('Clinical Measurements', margin, y);
    y += 2;
    doc.line(margin, y, pageWidth - margin, y);
    y += 7;

    doc.setFontSize(10);
    doc.setTextColor(40);
    doc.text(`• Child Head Circumference: ${opts.childHeadCircumferenceCm} cm`, margin + 2, y); y += 5.5;
    doc.text(`• Mother Head Circumference: ${opts.motherCircumferenceCm} cm`, margin + 2, y); y += 5.5;
    doc.text(`• Father Head Circumference: ${opts.fatherCircumferenceCm} cm`, margin + 2, y); y += 5.5;

    // --- Scores Table ---
    y += 5;
    doc.setFontSize(13);
    doc.setTextColor(0);
    doc.text('Scores', margin, y);
    y += 2;
    doc.line(margin, y, pageWidth - margin, y);
    y += 3;

    const parentalAverage = (opts.motherScore + opts.fatherScore) / 2;
    const tableData: [string, string][] = [
        ['Child Score', opts.childScore.toFixed(2)],
    ];
    if (opts.showCorrectedScore) {
        tableData.push(['Child Score (Corrected)', opts.correctedChildScore.toFixed(2)]);
    }
    tableData.push(
        ['Mother Score', opts.motherScore.toFixed(2)],
        ['Father Score', opts.fatherScore.toFixed(2)],
        ['Parental Average', parentalAverage.toFixed(2)],
    );

    // Draw table
    const colWidth = (pageWidth - 2 * margin) / 2;
    const rowHeight = 8;
    doc.setFontSize(9);

    // Header row
    y += 2;
    doc.setFillColor(60, 60, 60);
    doc.rect(margin, y, pageWidth - 2 * margin, rowHeight, 'F');
    doc.setTextColor(255);
    doc.text('Metric', margin + 3, y + 5.5);
    doc.text('Value', margin + colWidth + 3, y + 5.5);
    y += rowHeight;

    // Data rows
    for (const [label, value] of tableData) {
        const isAbnormalRow =
            (label === 'Child Score' && opts.isAbnormal) ||
            (label === 'Child Score (Corrected)' && opts.isAbnormalCorrected);

        if (isAbnormalRow) {
            doc.setFillColor(255, 220, 220);
        } else {
            doc.setFillColor(220, 255, 220);
        }

        if (label === 'Child Score' || label === 'Child Score (Corrected)') {
            doc.rect(margin, y, pageWidth - 2 * margin, rowHeight, 'F');
        }

        doc.setTextColor(40);
        doc.text(label, margin + 3, y + 5.5);
        doc.text(value, margin + colWidth + 3, y + 5.5);

        // Row border
        doc.setDrawColor(200);
        doc.line(margin, y + rowHeight, pageWidth - margin, y + rowHeight);
        y += rowHeight;
    }

    // --- Chart ---
    y += 8;
    doc.setFontSize(13);
    doc.setTextColor(0);
    doc.text('Weaver Diagram', margin, y);
    y += 2;
    doc.line(margin, y, pageWidth - margin, y);
    y += 5;

    // Embed SVG as image
    const chartPdfWidth = pageWidth - 2 * margin;
    const chartPdfHeight = chartPdfWidth * (opts.svgHeight / opts.svgWidth);

    // Check if chart fits on current page; if not, add a new page
    if (y + chartPdfHeight > doc.internal.pageSize.getHeight() - 30) {
        doc.addPage();
        y = margin;
    }

    doc.addSvgAsImage(opts.svgData, margin, y, chartPdfWidth, chartPdfHeight);
    y += chartPdfHeight + 8;

    // --- Clinical Interpretation ---
    if (y + 30 > doc.internal.pageSize.getHeight() - 30) {
        doc.addPage();
        y = margin;
    }

    doc.setFontSize(13);
    doc.setTextColor(0);
    doc.text('Clinical Interpretation', margin, y);
    y += 2;
    doc.line(margin, y, pageWidth - margin, y);
    y += 7;

    doc.setFontSize(10);
    doc.setTextColor(40);

    const expectedScore = INTERCEPT + SLOPE * parentalAverage;
    const scoreToCheck = opts.showCorrectedScore ? opts.correctedChildScore : opts.childScore;
    const scoreLabel = opts.showCorrectedScore ? "corrected " : "";
    const withinRange = scoreToCheck <= expectedScore + SD_THRESHOLD && scoreToCheck >= expectedScore - SD_THRESHOLD;

    const interpretation = withinRange
        ? `The child's ${scoreLabel}head circumference z-score (${scoreToCheck.toFixed(2)}) falls within the expected range (±${SD_THRESHOLD} SD) relative to the parental average (${parentalAverage.toFixed(2)}).`
        : `The child's ${scoreLabel}head circumference z-score (${scoreToCheck.toFixed(2)}) falls outside the expected range (±${SD_THRESHOLD} SD) relative to the parental average (${parentalAverage.toFixed(2)}). Further clinical evaluation may be warranted.`;

    const splitInterpretation = doc.splitTextToSize(interpretation, pageWidth - 2 * margin);
    doc.text(splitInterpretation, margin, y);
    y += splitInterpretation.length * 5 + 5;

    // --- Disclaimer ---
    if (y + 20 > doc.internal.pageSize.getHeight() - 10) {
        doc.addPage();
        y = margin;
    }

    doc.setFontSize(7);
    doc.setTextColor(128);
    const disclaimer = `This report is generated by Weaver Curve App v${opts.appVersion}. It is intended as a clinical decision support tool and does not constitute a diagnosis.`;
    const splitDisclaimer = doc.splitTextToSize(disclaimer, pageWidth - 2 * margin);
    doc.text(splitDisclaimer, margin, doc.internal.pageSize.getHeight() - 10);

    // Return as Uint8Array
    const arrayBuffer = doc.output('arraybuffer');
    return new Uint8Array(arrayBuffer);
}
```

- [ ] **Step 2: Verify the build**

```bash
npm run build
```

Expected: Build succeeds.

- [ ] **Step 3: Commit**

```bash
git add src/lib/pdf-export.ts
git commit -m "feat: implement frontend PDF generation with jsPDF, replacing Rust typst pipeline"
```

---

## Task 7: Frontend Unit Tests

**Files:**
- Modify: `src/utils.test.ts` → rename to `src/lib/constants.test.ts`
- Create: `src/components/score_card.test.ts`

- [ ] **Step 1: Replace utils.test.ts with constants.test.ts**

Delete the old file and create the new one:

```bash
rm src/utils.test.ts
```

Create `src/lib/constants.test.ts`:

```ts
import { describe, it, expect } from 'vitest';
import {
    INTERCEPT, SLOPE, SD_THRESHOLD,
    MAX_AGE_MONTHS, MAX_HEAD_CIRCUMFERENCE_CM,
    MAX_PREMATURE_WEEKS, MAX_PREMATURE_DAYS
} from './constants';

describe('Medical Constants', () => {
    it('has correct regression intercept', () => {
        expect(INTERCEPT).toBe(0.138891);
    });

    it('has correct regression slope', () => {
        expect(SLOPE).toBe(0.483034);
    });

    it('has SD threshold of 2', () => {
        expect(SD_THRESHOLD).toBe(2);
    });

    it('has max age of 216 months (18 years)', () => {
        expect(MAX_AGE_MONTHS).toBe(216);
    });

    it('has reasonable max head circumference', () => {
        expect(MAX_HEAD_CIRCUMFERENCE_CM).toBe(70);
    });

    it('has max premature weeks of 42', () => {
        expect(MAX_PREMATURE_WEEKS).toBe(42);
    });

    it('has max premature days of 7', () => {
        expect(MAX_PREMATURE_DAYS).toBe(7);
    });
});

describe('Abnormality Detection', () => {
    it('detects abnormal score above threshold', () => {
        const parentalAverage = 0;
        const yMean = INTERCEPT + SLOPE * parentalAverage;
        const childScore = yMean + SD_THRESHOLD + 0.5;
        const isAbnormal = childScore > yMean + SD_THRESHOLD || childScore < yMean - SD_THRESHOLD;
        expect(isAbnormal).toBe(true);
    });

    it('detects abnormal score below threshold', () => {
        const parentalAverage = 0;
        const yMean = INTERCEPT + SLOPE * parentalAverage;
        const childScore = yMean - SD_THRESHOLD - 0.5;
        const isAbnormal = childScore > yMean + SD_THRESHOLD || childScore < yMean - SD_THRESHOLD;
        expect(isAbnormal).toBe(true);
    });

    it('detects normal score within threshold', () => {
        const parentalAverage = 1.0;
        const yMean = INTERCEPT + SLOPE * parentalAverage;
        const childScore = yMean + 0.5;
        const isAbnormal = childScore > yMean + SD_THRESHOLD || childScore < yMean - SD_THRESHOLD;
        expect(isAbnormal).toBe(false);
    });
});
```

- [ ] **Step 2: Create score_card component test**

Create `src/components/score_card.test.ts`:

```ts
import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import ScoreCard from './score_card.svelte';

describe('ScoreCard', () => {
    it('shows skeleton when show_score is false', () => {
        render(ScoreCard, {
            props: {
                show_score: false,
                show_corrected_score: false,
                child_score: 0,
                correct_score: 0,
                mother_score: 0,
                father_score: 0,
                child_age_in_months: 12,
            }
        });
        const skeletons = document.querySelectorAll('.skeleton');
        expect(skeletons.length).toBeGreaterThan(0);
    });

    it('displays scores when show_score is true', () => {
        render(ScoreCard, {
            props: {
                show_score: true,
                show_corrected_score: false,
                child_score: 1.5,
                correct_score: 1.3,
                mother_score: 0.5,
                father_score: 0.8,
                child_age_in_months: 12,
            }
        });
        expect(screen.getByText('1.50')).toBeTruthy();
        expect(screen.getByText('0.65')).toBeTruthy(); // parental average (0.5+0.8)/2
    });

    it('shows corrected score when enabled', () => {
        render(ScoreCard, {
            props: {
                show_score: true,
                show_corrected_score: true,
                child_score: 1.5,
                correct_score: 1.2,
                mother_score: 0.5,
                father_score: 0.8,
                child_age_in_months: 12,
            }
        });
        expect(screen.getByText('Child (corrected)')).toBeTruthy();
        expect(screen.getByText('1.20')).toBeTruthy();
    });

    it('shows warning for invalid premature age', () => {
        render(ScoreCard, {
            props: {
                show_score: true,
                show_corrected_score: false,
                child_score: 1.5,
                correct_score: 1.3,
                mother_score: 0.5,
                father_score: 0.8,
                child_age_in_months: 2,
                gestational_age_in_weeks: 30,
            }
        });
        expect(screen.getByText('Warning')).toBeTruthy();
        expect(screen.getByText('Results may not be accurate for current premature age.')).toBeTruthy();
    });

    it('applies error styling for abnormal scores', () => {
        render(ScoreCard, {
            props: {
                show_score: true,
                show_corrected_score: false,
                child_score: 5.0,
                correct_score: 5.0,
                mother_score: 0.0,
                father_score: 0.0,
                child_age_in_months: 12,
            }
        });
        const scoreValue = screen.getByText('5.00');
        expect(scoreValue.classList.contains('text-error')).toBe(true);
    });
});
```

- [ ] **Step 3: Run all frontend tests**

```bash
npm run test:run
```

Expected: All tests pass.

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "test: rewrite frontend unit tests with constants and component tests"
```

---

## Task 8: E2E Test Infrastructure — Tauri Mock and Playwright Setup

**Files:**
- Create: `src/lib/api.ts`
- Create: `src/lib/tauri-mock.ts`
- Create: `playwright.config.ts`
- Create: `e2e/weaver-curve.spec.ts`
- Modify: `package.json` (scripts)

- [ ] **Step 1: Create the API abstraction layer**

Create `src/lib/api.ts`:

```ts
import { INTERCEPT, SLOPE, SD_THRESHOLD, MAX_AGE_MONTHS, MAX_HEAD_CIRCUMFERENCE_CM } from './constants';

const isMocked = typeof import.meta !== 'undefined' && import.meta.env?.VITE_MOCK_TAURI === 'true';

// Mock implementation of calculate_scores (matches Rust logic)
function mockCalculateScores(args: {
    childAgeMonths: number;
    childHeadCircumferenceCm: number;
    motherCircumferenceCm: number;
    fatherCircumferenceCm: number;
    prematureConceptionWeeks: number;
    prematureConceptionDays: number;
    gender: string;
}): [number, number, number, number] {
    const ADULT_MEAN_MALE = 55.95;
    const ADULT_STD_MALE = 1.34;
    const ADULT_MEAN_FEMALE = 54.94;
    const ADULT_STD_FEMALE = 1.40;

    // Simple mock: use adult values as approximation for child lookup
    // (Real interpolation happens in Rust; this is close enough for E2E UI testing)
    const childMean = args.gender === 'male' ? 47.0 : 45.81; // 12-month approximation
    const childStd = 1.31;

    let correctedAge = args.childAgeMonths;
    if (args.prematureConceptionWeeks > 0 || args.prematureConceptionDays > 0) {
        const gestAge = args.prematureConceptionWeeks + args.prematureConceptionDays / 7;
        correctedAge = args.childAgeMonths - (40 - gestAge) / 4.345;
    }

    const childScore = (args.childHeadCircumferenceCm - childMean) / childStd;
    const correctedChildScore = correctedAge !== args.childAgeMonths
        ? (args.childHeadCircumferenceCm - childMean) / childStd * 0.95  // Slight offset for corrected
        : childScore;

    const dadScore = (args.fatherCircumferenceCm - ADULT_MEAN_MALE) / ADULT_STD_MALE;
    const momScore = (args.motherCircumferenceCm - ADULT_MEAN_FEMALE) / ADULT_STD_FEMALE;

    return [dadScore, momScore, childScore, correctedChildScore];
}

export async function invokeCalculateScores(args: {
    childAgeMonths: number;
    childHeadCircumferenceCm: number;
    motherCircumferenceCm: number;
    fatherCircumferenceCm: number;
    prematureConceptionWeeks: number;
    prematureConceptionDays: number;
    gender: string;
}): Promise<[number, number, number, number]> {
    if (isMocked) {
        return mockCalculateScores(args);
    }
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke('calculate_scores', args);
}
```

- [ ] **Step 2: Update +page.svelte to use the API abstraction**

In `src/routes/+page.svelte`, replace the import and invoke call:

Change:
```ts
import { invoke } from "@tauri-apps/api/core";
```
To:
```ts
import { invokeCalculateScores } from "$lib/api";
```

And replace the `invoke("calculate_scores", {...})` call with:
```ts
    invokeCalculateScores({
      childAgeMonths: child_age_in_months,
      childHeadCircumferenceCm: child_head_circumference_in_cm,
      motherCircumferenceCm: mother_circumference_in_cm,
      fatherCircumferenceCm: father_circumference_in_cm,
      prematureConceptionWeeks: premature_conception_in_weeks,
      prematureConceptionDays: premature_conception_in_days,
      gender: selected_gender,
    }).then((res) => {
      show_scores = true;
      show_corrected_scores =
        premature_conception_in_days > 0 || premature_conception_in_weeks > 0;
      father_score = res[0];
      mother_score = res[1];
      child_score = res[2];
      corrected_child_score = res[3];
    }).catch((err: any) => {
      error = `Calculation failed: ${err}`;
      console.error("invoke error:", err);
    });
```

- [ ] **Step 3: Create playwright.config.ts**

```ts
import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
    testDir: './e2e',
    fullyParallel: true,
    forbidOnly: !!process.env.CI,
    retries: process.env.CI ? 2 : 0,
    workers: process.env.CI ? 1 : undefined,
    reporter: 'html',
    use: {
        baseURL: 'http://localhost:1420',
        trace: 'on-first-retry',
    },
    projects: [
        {
            name: 'chromium',
            use: { ...devices['Desktop Chrome'] },
        },
    ],
    webServer: {
        command: 'VITE_MOCK_TAURI=true npm run dev -- --port 1420',
        url: 'http://localhost:1420',
        reuseExistingServer: !process.env.CI,
        timeout: 30000,
    },
});
```

- [ ] **Step 4: Add npm scripts**

Add to `package.json` scripts:

```json
"test:e2e": "playwright test",
"test:e2e:ui": "playwright test --ui"
```

- [ ] **Step 5: Create E2E test file**

Create `e2e/weaver-curve.spec.ts`:

```ts
import { test, expect } from '@playwright/test';

test.describe('Weaver Curve App', () => {

    test('shows form with all required fields', async ({ page }) => {
        await page.goto('/');
        await expect(page.locator('h2:has-text("Weaver Curve")')).toBeVisible();
        await expect(page.locator('select[name="sex"]')).toBeVisible();
        await expect(page.locator('text=Child DOB')).toBeVisible();
        await expect(page.locator('text=Child Age')).toBeVisible();
        await expect(page.locator('text=Submit')).toBeVisible();
        await expect(page.locator('text=Reset')).toBeVisible();
    });

    test('shows validation errors on empty submit', async ({ page }) => {
        await page.goto('/');
        await page.click('button:has-text("Submit")');
        await expect(page.locator('text=Please select a gender')).toBeVisible();
        await expect(page.locator('text=Please enter a valid age')).toBeVisible();
        await expect(page.locator('text=Please enter a valid circumference')).toBeVisible();
    });

    test('happy path — calculate and display scores', async ({ page }) => {
        await page.goto('/');

        // Fill form
        await page.selectOption('select[name="sex"]', 'male');
        await page.fill('input[type="number"]:near(:text("Child Age"))', '12');
        await page.fill('input[type="number"]:near(:text("Child"), :text("in cm"))', '47');
        await page.fill('input[type="number"]:near(:text("Mother"))', '54');
        await page.fill('input[type="number"]:near(:text("Father"))', '56');

        // Submit
        await page.click('button:has-text("Submit")');

        // Verify scores appear
        await expect(page.locator('.stat-value').first()).toBeVisible();
        await expect(page.locator('h2:has-text("Weaver Scores")')).toBeVisible();
        await expect(page.locator('h2:has-text("Weaver Plot")')).toBeVisible();

        // Verify SVG chart rendered
        await expect(page.locator('svg')).toBeVisible();
        await expect(page.locator('svg circle')).toBeVisible();
    });

    test('reset clears form and hides scores', async ({ page }) => {
        await page.goto('/');

        // Fill and submit
        await page.selectOption('select[name="sex"]', 'female');
        await page.fill('input[type="number"]:near(:text("Child Age"))', '24');
        await page.fill('input[type="number"]:near(:text("Child"), :text("in cm"))', '48');
        await page.fill('input[type="number"]:near(:text("Mother"))', '55');
        await page.fill('input[type="number"]:near(:text("Father"))', '57');
        await page.click('button:has-text("Submit")');
        await expect(page.locator('.stat-value').first()).toBeVisible();

        // Reset
        await page.click('button:has-text("Reset")');

        // Verify skeleton placeholders are back
        await expect(page.locator('.skeleton').first()).toBeVisible();
    });

    test('premature birth shows corrected score and warning', async ({ page }) => {
        await page.goto('/');

        await page.selectOption('select[name="sex"]', 'male');
        await page.fill('input[type="number"]:near(:text("Child Age"))', '3');
        await page.fill('input[type="number"]:near(:text("Child"), :text("in cm"))', '40');
        await page.fill('input[type="number"]:near(:text("Mother"))', '54');
        await page.fill('input[type="number"]:near(:text("Father"))', '56');

        // Open premature section and fill
        await page.click('text=Premature Birth');
        await page.fill('input[type="number"]:near(:text("in weeks"))', '30');
        await page.fill('input[type="number"]:near(:text("in days"))', '3');

        await page.click('button:has-text("Submit")');

        // Verify corrected score visible
        await expect(page.locator('text=Child (corrected)')).toBeVisible();
        // Verify warning for young premature infant
        await expect(page.locator('text=Warning')).toBeVisible();
    });

    test('gender selection works', async ({ page }) => {
        await page.goto('/');
        const select = page.locator('select[name="sex"]');

        await select.selectOption('male');
        await expect(select).toHaveValue('male');

        await select.selectOption('female');
        await expect(select).toHaveValue('female');
    });

    test('print button appears after score calculation', async ({ page }) => {
        await page.goto('/');

        // Before calculation — no print button
        await expect(page.locator('button:has-text("Print")')).not.toBeVisible();

        // Fill and submit
        await page.selectOption('select[name="sex"]', 'male');
        await page.fill('input[type="number"]:near(:text("Child Age"))', '12');
        await page.fill('input[type="number"]:near(:text("Child"), :text("in cm"))', '47');
        await page.fill('input[type="number"]:near(:text("Mother"))', '54');
        await page.fill('input[type="number"]:near(:text("Father"))', '56');
        await page.click('button:has-text("Submit")');

        // After calculation — print button visible
        await expect(page.locator('button:has-text("Print")')).toBeVisible();
    });

    test('validation errors clear on resubmit', async ({ page }) => {
        await page.goto('/');

        // Submit empty to trigger errors
        await page.click('button:has-text("Submit")');
        await expect(page.locator('text=Please select a gender')).toBeVisible();

        // Fix gender and fill form fully
        await page.selectOption('select[name="sex"]', 'male');
        await page.fill('input[type="number"]:near(:text("Child Age"))', '12');
        await page.fill('input[type="number"]:near(:text("Child"), :text("in cm"))', '47');
        await page.fill('input[type="number"]:near(:text("Mother"))', '54');
        await page.fill('input[type="number"]:near(:text("Father"))', '56');

        // Resubmit
        await page.click('button:has-text("Submit")');

        // Errors should be gone
        await expect(page.locator('text=Please select a gender')).not.toBeVisible();
    });
});
```

- [ ] **Step 6: Verify build still works**

```bash
npm run build
```

- [ ] **Step 7: Commit**

```bash
git add -A
git commit -m "test: add E2E infrastructure with Playwright, Tauri mock, and API abstraction layer"
```

---

## Task 9: CI/CD Fixes — Release Workflow, E2E in CI

**Files:**
- Modify: `.github/workflows/ci.yml`
- Modify: `.github/workflows/release.yml`

- [ ] **Step 1: Add E2E test job and update CI workflow**

Replace `.github/workflows/ci.yml`:

```yaml
name: CI
permissions:
  contents: read

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test-frontend:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4

    - name: Setup Node.js
      uses: actions/setup-node@v4
      with:
        node-version: 'lts/*'
        cache: 'npm'

    - name: Install frontend dependencies
      run: npm ci

    - name: Run frontend linting
      run: npm run check

    - name: Run frontend tests
      run: npm run test:run

    - name: Build frontend
      run: npm run build

  test-e2e:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4

    - name: Setup Node.js
      uses: actions/setup-node@v4
      with:
        node-version: 'lts/*'
        cache: 'npm'

    - name: Install dependencies
      run: npm ci

    - name: Install Playwright browsers
      run: npx playwright install --with-deps chromium

    - name: Run E2E tests
      run: npm run test:e2e
      env:
        VITE_MOCK_TAURI: 'true'

    - name: Upload Playwright report
      uses: actions/upload-artifact@v4
      if: always()
      with:
        name: playwright-report
        path: playwright-report/
        retention-days: 30

  test-backend:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4

    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable

    - name: Install system dependencies
      run: |
        sudo apt-get update
        sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf libgtk-3-dev

    - name: Cache Rust dependencies
      uses: actions/cache@v4
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          src-tauri/target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

    - name: Run Rust tests
      run: cd src-tauri && cargo test

    - name: Check Rust compilation
      run: cd src-tauri && cargo check
```

- [ ] **Step 2: Add macOS x86 to release workflow**

In `.github/workflows/release.yml`, add the Intel Mac target to the matrix:

```yaml
    strategy:
      fail-fast: false
      matrix:
        include:
          - platform: 'macos-latest'
            args: '--target aarch64-apple-darwin'
          - platform: 'macos-13'
            args: '--target x86_64-apple-darwin'
          - platform: 'ubuntu-22.04'
            args: ''
          - platform: 'windows-latest'
            args: ''
```

Also add the `x86_64-apple-darwin` target in the Rust toolchain step:

```yaml
      - name: install Rust stable
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.platform == 'macos-latest' && 'aarch64-apple-darwin' || matrix.platform == 'macos-13' && 'x86_64-apple-darwin' || '' }}
```

- [ ] **Step 3: Commit**

```bash
git add .github/workflows/ci.yml .github/workflows/release.yml
git commit -m "ci: add E2E test job with Playwright, add macOS x86 release target"
```

---

## Task 10: Final Verification

- [ ] **Step 1: Run all frontend tests**

```bash
npm run test:run
```

Expected: All unit tests pass.

- [ ] **Step 2: Run Rust tests**

```bash
cd src-tauri && cargo test
```

Expected: All Rust tests pass, including new validation and gender tests.

- [ ] **Step 3: Run frontend build**

```bash
npm run build
```

Expected: Build succeeds with no errors.

- [ ] **Step 4: Run Rust check**

```bash
cd src-tauri && cargo check
```

Expected: No compilation errors.

- [ ] **Step 5: Run E2E tests (if Playwright is installed)**

```bash
VITE_MOCK_TAURI=true npm run test:e2e
```

Expected: All 7 E2E test scenarios pass.

- [ ] **Step 6: Verify git status is clean**

```bash
git status
git log --oneline -10
```

Expected: All changes committed across ~9 commits on the `refactor/comprehensive-overhaul` branch.
