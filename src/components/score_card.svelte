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
