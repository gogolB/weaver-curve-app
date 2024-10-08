<script>

    import * as d3 from "d3";
    import { scaleLinear } from "d3-scale";
    import { save } from '@tauri-apps/plugin-dialog';
    import { invoke } from "@tauri-apps/api/core";
    import { getVersion } from '@tauri-apps/api/app';

    export let show_score = true;
    export let show_corrected_score = false;
    export let child_score;
    export let correct_score;
    export let mother_score;
    export let father_score;
    // @ts-ignore
    export let child_dob;

    export let child_age_in_months = 0;
    export let mother_circumference_in_cm = 0;
    export let father_circumference_in_cm = 0;
    export let child_head_circumference_in_cm = 0;
    export let premature_conception_in_days = 0;
    export let premature_conception_in_weeks = 0;
    export let gender = 0;


    export let chartWidth = 700;
    export let chartHeight = 700;
    const paddings = {
        top: 20,
        left: 60,
        right: 20,
        bottom: 50,
    };

    let xScale = scaleLinear().domain([-5, 5]).range([paddings.left, chartWidth - paddings.right]);
    let yScale = scaleLinear().domain([5, -5]).range([paddings.top, chartHeight - paddings.bottom]);

    const yGrid = yScale.ticks(10)
    const xGrid = xScale.ticks(10)

    /**
     * @type {SVGGElement}
     */
    let gy;
    /**
     * @type {SVGGElement}
     */
    let gx;

    const intercept = 0.138891; 
    const slope = 0.483034;
    $: parental_average = (mother_score + father_score) / 2;
    $: y_mean = intercept + slope * parental_average
    $: is_abnormal = child_score > y_mean + 2 || child_score < y_mean - 2
    $: is_abnormal_corrected = correct_score > y_mean + 2 || correct_score < y_mean - 2

    $: d3.select(gy).call(d3.axisLeft(yScale));
    $: d3.select(gx).call(d3.axisBottom(xScale));
    $: xScale = scaleLinear().domain([-5, 5]).range([paddings.left, chartWidth - paddings.right]);
    $: yScale = scaleLinear().domain([5, -5]).range([paddings.top, chartHeight - paddings.bottom]);

    async function print_to_pdf() {
      const path = await save({
        filters: [
          {
            name: 'My Filter',
            extensions: ['pdf'],
          },
        ],
      });
      const appVersion = await getVersion();
      let result = await invoke("make_pdf", {
        filePath: path,
        childAgeMonths: child_age_in_months,
        childHeadCircumferenceCm: child_head_circumference_in_cm,
        motherCircumferenceCm: mother_circumference_in_cm,
        fatherCircumferenceCm: father_circumference_in_cm,
        prematureConceptionWeeks: premature_conception_in_weeks,
        prematureConceptionDays: premature_conception_in_days,
        gender: gender,
        childDob: child_dob,
        appVersion: appVersion
        
      })
      console.log(result)

      if (result) {
        alert("PDF saved to " + path);
      } else {
        alert("Failed to save PDF");
      }
    }

</script>

<div class="card bg-neutral text-neutral-content place-self-center w-5/6 mt-3">
    <div class="card-body items-center text-center">
        <h2 class="card-title">Weaver Plot</h2>
        {#if show_score}
            <svg width={chartWidth} height={chartHeight}>
                <g>
                    {#each {length: 10} as _, x}
                        <line
                            x1={xScale(x-5)}
                            x2={xScale(x-4)}
                            y1={yScale(0.138891 + (x-5) * 0.483034)}
                            y2={yScale(0.138891 + (x-4) * 0.483034)}
                            stroke="blue"
                            stroke-width="2"
                        />

                        <line
                            x1={xScale(x-5)}
                            x2={xScale(x-4)}
                            y1={yScale(0.138891 + (x-5) * 0.483034 + 2)}
                            y2={yScale(0.138891 + (x-4) * 0.483034 + 2)}
                            stroke="orange"
                            style="stroke-dasharray: 3, 3;"
                            stroke-width="1"
                        />

                        <line
                            x1={xScale(x-5)}
                            x2={xScale(x-4)}
                            y1={yScale(0.138891 + (x-5) * 0.483034 - 2)}
                            y2={yScale(0.138891 + (x-4) * 0.483034 - 2)}
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
                        style="fill: {is_abnormal_corrected ? "red" : "green"}"
                      />
                    </g>
                  {/if}


                  <!-- Grid Line -->
                  {#each {length: 10} as _, x}
                    <g>
                      <line
                        x1={xScale(x-5)}
                        x2={xScale(x-5)}
                        y1={yScale(5)}
                        y2={yScale(-5)}
                        stroke="gray"
                        stroke-width="0.5"
                      />
                    </g>
                  {/each}

                  {#each {length: 10} as _, y}
                  <g>
                    <line
                      x1={xScale(-5)}
                      x2={xScale(5)}
                      y1={yScale(y-5)}
                      y2={yScale(y-5)}
                      stroke="gray"
                      stroke-width="0.5"
                    />
                  </g>
                {/each}

                <!-- Legend -->
                <g>
                  <rect
                    x={xScale(-5) + 5}
                    y={yScale(5) + 5}
                    width = {show_corrected_score ? 230 : 120}
                    height = {show_corrected_score ? 100 : 80}
                    style="fill: grey"
                    stroke="black"
                  />
                </g>
                <g>
                    <line
                        x1={xScale(-5) + 15}
                        x2={xScale(-5) + 30}
                        y1={yScale(5) + 25}
                        y2={yScale(5) + 25}
                        stroke="blue"
                        stroke-width="2"
                    />
                    <text x={xScale(-5) + 35} y={yScale(5) + 30} fill="white" text-anchor="start">Mean</text>
                </g>
                <g>
                    <line
                        x1={xScale(-5) + 15}
                        x2={xScale(-5) + 30}
                        y1={yScale(5) + 45}
                        y2={yScale(5) + 45}
                        stroke="orange"
                        stroke-width="1"
                        style="stroke-dasharray: 3, 3;"
                    />
                    <text x={xScale(-5) + 35} y={yScale(5) + 50} fill="white" text-anchor="start">± 2 SD</text>
                </g>
                <g>
                  <circle
                    cx={xScale(-5) + 22}
                    cy={yScale(5) + 65}
                    r={3}
                    style="fill: {is_abnormal ? "red" : "green"}"
                  />
                  <text x={xScale(-5) + 35} y={yScale(5) + 70} fill="white" text-anchor="start">Child Score</text>
                </g>
                {#if show_corrected_score}
                  <g>
                    <rect
                      x={xScale(-5) + 17}
                      y={yScale(5) + 80}
                      width=10
                      height=10
                      style="fill: {is_abnormal_corrected ? "red" : "green"}"
                    />
                    <text x={xScale(-5) + 35} y={yScale(5) + 90} fill="white" text-anchor="start">Child Score (GA Corrected)</text>
                  </g>
                {/if}
            </svg>
          <button class="btn btn-primary mt-4" on:click={print_to_pdf}>Print</button>
        {:else}
            <div class="skeleton h-96 w-96"></div>
        {/if}
    </div>
</div>