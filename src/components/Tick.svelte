<script>
    import * as d3 from "d3";
    export let x;
    export let y;
    export let value;
    export let direction;
    const xTranslation = direction === "horizontal" ? x - 10 : x;

    const valueLabel = value

</script>

<g transform={"translate(" + xTranslation + ", " + y + ")"}>
  <text
    y={direction === "horizontal" ? 0 : 20}
    font-size="13px"
    text-anchor={direction === "horizontal" ? "end" : "middle"}
    alignment-baseline="middle">
      {valueLabel}
  </text>
  {#if direction === "horizontal"}
    <line 
      x1={2} 
      x2={8} 
      y1={0} 
      y2={0} 
      stroke="black" 
      stroke-width="1" />
  {:else}
    <line 
      x1={0} 
      x2={0} 
      y1={2} 
      y2={8} 
      stroke="black" 
      stroke-width="1" />
  {/if}
  </g>