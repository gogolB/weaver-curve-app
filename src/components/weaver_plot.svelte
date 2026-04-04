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

    let gy = $state<SVGGElement>(undefined!);
    let gx = $state<SVGGElement>(undefined!);
    let svgElement = $state<SVGSVGElement>(undefined!);

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
                        width="20"
                        height="20"
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
                    <rect x={xScale(-5) + 17} y={yScale(5) + 80} width="10" height="10" style="fill: {is_abnormal_corrected ? 'red' : 'green'}" />
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
