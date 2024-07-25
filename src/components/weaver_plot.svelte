<script>

    import * as d3 from "d3";
    import { scaleLinear } from "d3-scale";
    import Tick from "./Tick.svelte";


    export let show_score = true;
    export let show_corrected_score = false;
    export let child_score;
    export let correct_score;
    export let mother_score;
    export let father_score;

    export let chartWidth = 700;
    export let chartHeight = 700;
    const paddings = {
        top: 20,
        left: 60,
        right: 20,
        bottom: 50,
    };

    const xScale = scaleLinear().domain([-5, 5]).range([paddings.left, chartWidth - paddings.right]);
    const yScale = scaleLinear().domain([-5, 5]).range([chartHeight - paddings.bottom, paddings.top]);

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

</script>

<div class="card bg-neutral text-neutral-content place-self-center w-5/6 mt-3">
    <div class="card-body items-center text-center">
        <h2 class="card-title">Weaver Plot</h2>
        {#if show_score}
            <svg width={chartWidth} height={chartHeight}>
                <g>
                    <line
                    x1={paddings.left}
                    x2={chartWidth - paddings.right}
                    y1={chartHeight - paddings.bottom}
                    y2={chartHeight - paddings.bottom}
                    stroke="black"
                    stroke-width="2"
                    />
                    <line
                    x1={paddings.left}
                    x2={paddings.left}
                    y1={paddings.top}
                    y2={chartHeight - paddings.bottom}
                    stroke="black"
                    stroke-width="2"
                    />
                </g>

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

                  <g bind:this={gx} transform="translate(0,{chartHeight - paddings.bottom})"></g>
                  <g>
                    <text x={chartWidth / 2} y={chartHeight - 15} fill="white" text-anchor="middle">Standard Score (Parental Average)</text>
                  </g>
                  
                  <g bind:this={gy} transform="translate({paddings.left},0)"></g>
                  <g>
                    <text x={-chartHeight / 2} y={paddings.left / 2} transform="rotate(-90)" fill="white" text-anchor="middle"> Standard Score (Child)</text>
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
                <g>
                  <line>

                  </line>
                </g>




            </svg>
        {:else}
            <div class="skeleton h-96 w-96"></div>
        {/if}
    </div>
</div>