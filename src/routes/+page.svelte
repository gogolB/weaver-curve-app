<script lang="ts">
  import { invokeCalculateScores } from "$lib/api";
  import { calculateAgeInMonths } from "$lib/age";
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

  function process_form() {
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
  }

  function update_child_age_in_months() {
    if (!date) return;
    child_age_in_months = calculateAgeInMonths(date);
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

      <div class="divider">Demographics</div>
      <label class="form-control w-full max-w-xs">
        <div class="label">
          <span class="label-text">Sex Assigned at Birth</span>
        </div>
        <select class="select select-bordered {error_selected_gender ? 'select-error' : ''}" name="sex" bind:value={selected_gender}>
          <option value="" disabled selected>Pick one</option>
          <option value="male">Male</option>
          <option value="female">Female</option>
        </select>
        {#if error_selected_gender}
          <div class="label">
            <span class="label-text-alt text-error">Please select a gender</span>
          </div>
        {/if}
      </label>

      <div class="flex flex-row">

        <label class="form-control w-1/2 max-w-xs">
          <div class="label">
            <span class="label-text">Child DOB</span>
          </div>
          <input type="date" class="input input-bordered {error_child_age_in_months ? 'input-error' : ''}" bind:value={date} oninput={update_child_age_in_months}/>
          {#if error_child_age_in_months}
            <div class="label">
              <span class="label-text-alt text-error">Please enter a valid date</span>
            </div>
          {/if}
        </label>

        <div class="divider divider-horizontal ">OR</div>

        <label class="form-control w-1/2 max-w-xs">
          <div class="label">
            <span class="label-text">Child Age</span>
            <span class="label-text-alt">in months</span>
          </div>
          <input type="number" class="input input-bordered {error_child_age_in_months ? 'input-error' : ''}" bind:value={child_age_in_months}/>
          {#if error_child_age_in_months}
            <div class="label">
              <span class="label-text-alt text-error">Please enter a valid age</span>
            </div>
           {/if}
        </label>
      </div>

      <div class="divider">Head Circumference</div>

      <div class="flex flex-row w-full justify-center">

        <label class="form-control w-1/4 max-w-xs">
          <div class="label">
            <span class="label-text">Child</span>
            <span class="label-text-alt">in cm</span>
          </div>
          <input type="number" class="input input-bordered {error_child_head_circumference_in_cm ? 'input-error' : ''}" bind:value={child_head_circumference_in_cm}/>
          {#if error_child_head_circumference_in_cm}
            <div class="label">
              <span class="label-text-alt text-error">Please enter a valid circumference</span>
            </div>
          {/if}
        </label>

        <label class="form-control w-1/4 max-w-xs mx-2">
          <div class="label">
            <span class="label-text">Mother</span>
            <span class="label-text-alt">in cm</span>
          </div>
          <input type="number" class="input input-bordered {error_mother_circumference_in_cm ? 'input-error' : ''}" bind:value={mother_circumference_in_cm}/>
          {#if error_mother_circumference_in_cm}
            <div class="label">
              <span class="label-text-alt text-error">Please enter a valid circumference</span>
            </div>
          {/if}
        </label>

        <label class="form-control w-1/4 max-w-xs">
          <div class="label">
            <span class="label-text">Father</span>
            <span class="label-text-alt">in cm</span>
          </div>
          <input type="number" class="input input-bordered {error_father_circumference_in_cm ? 'input-error' : ''}" bind:value={father_circumference_in_cm}/>
          {#if error_father_circumference_in_cm}
            <div class="label">
              <span class="label-text-alt text-error">Please enter a valid circumference</span>
            </div>
          {/if}
        </label>
      </div>

      <div class="collapse collapse-arrow bg-base-200">
        <input type="checkbox"/>
        <div class="collapse-title text-xl font-medium">Premature Birth</div>
        <div class="collapse-content">
          <div class="flex flex-row w-full justify-center">
            <label class="form-control w-1/4 max-w-xs">
              <div class="label">
                <span class="label-text-alt">in weeks</span>
              </div>
              <input type="number" class="input input-bordered {error_premature_conception_in_weeks ? 'input-error' : ''}" bind:value={premature_conception_in_weeks}/>
              {#if error_premature_conception_in_weeks}
                <div class="label">
                  <span class="label-text-alt text-error">Please enter a valid age in weeks</span>
                </div>
              {/if}
            </label>

            <label class="form-control w-1/4 max-w-xs mx-2">
              <div class="label">
                <span class="label-text-alt">in days</span>
              </div>
              <input type="number" class="input input-bordered {error_premature_conception_in_days ? 'input-error' : ''}" bind:value={premature_conception_in_days}/>
              {#if error_premature_conception_in_days}
                <div class="label">
                  <span class="label-text-alt text-error">Please enter a valid age in days</span>
                </div>
              {/if}
            </label>
          </div>
        </div>
      </div>

      <button class="btn btn-primary mt-4" onclick={process_form}>Submit</button> <button class="btn btn-secondary mt-4" onclick={reset_form}>Reset</button>

    </div>
  </div>


  <ScoreCard  show_score={show_scores} show_corrected_score={show_corrected_scores} child_score={child_score} correct_score={corrected_child_score} mother_score={mother_score} father_score={father_score} child_age_in_months={child_age_in_months} gestational_age_in_weeks={premature_conception_in_weeks}/>
  <WeaverPlot show_score={show_scores} show_corrected_score={show_corrected_scores} child_score={child_score} correct_score={corrected_child_score} mother_score={mother_score} father_score={father_score} chartWidth={innerWidth * 0.8 } chartHeight={innerWidth * 0.8 * 0.78} child_age_in_months={child_age_in_months} premature_conception_in_weeks={premature_conception_in_weeks} premature_conception_in_days={premature_conception_in_days} gender={selected_gender} mother_circumference_in_cm={mother_circumference_in_cm} father_circumference_in_cm={father_circumference_in_cm} child_head_circumference_in_cm={child_head_circumference_in_cm} child_dob={date}/>
</div>
