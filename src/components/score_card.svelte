<script>
    export let show_score = false;
    export let show_corrected_score = false;
    export let child_score;
    export let correct_score;
    export let mother_score;
    export let father_score;
    export let child_age_in_months;
    export let gestiational_age_in_weeks = 0;

    const intercept = 0.138891; 
    const slope = 0.483034;
    $: parental_average = (mother_score + father_score) / 2;
    $: y_mean = intercept + slope * parental_average
    $: is_abnormal = child_score > y_mean + 2 || child_score < y_mean - 2
    $: is_abnormal_corrected = correct_score > y_mean + 2 || correct_score < y_mean - 2
    $: is_invalid  = (gestiational_age_in_weeks > 0 && child_age_in_months < (40 - gestiational_age_in_weeks) / 4.345) ? true : false;
</script>

<div class="card bg-neutral text-neutral-content place-self-center w-5/6 mt-3">
    <div class="card-body items-center text-center">
        <h2 class="card-title">Weaver Scores</h2>
        <div class="flex flex-col justify-center">
          {#if show_score}
            <div class="flex flex-row">
              <div class="stats shadow">
                  {#if show_corrected_score}
                  <div class="stat place-items-center">
                    <div class="stat-title">Child</div>
                    <div class="stat-value {is_abnormal_corrected ? 'text-error' : 'text-success'}">{correct_score.toFixed(2)}</div>
                    <div class="stat-desc {is_abnormal ? 'text-error' : 'text-success'}">Corrected score, original score: {child_score.toFixed(2)}</div>
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
                    <div class="stat-value">{((mother_score + father_score) / 2).toFixed(2)}</div>
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
            <div class="bg-orange-100 border-l-4 border-orange-500 text-orange-700 p-4" role="alert">
              <p class="font-bold">Warning</p>
              <p>Results may not be accurate for current premature age.</p>
            </div>
          </div>
          {/if}
        </div>
    </div>
</div>