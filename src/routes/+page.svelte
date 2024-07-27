<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import ScoreCard from "../components/score_card.svelte";
  import WeaverPlot from "../components/weaver_plot.svelte";

  let date: string;
  let mother_circumference_in_cm = 0;
  let father_circumference_in_cm = 0;
  let child_head_circumference_in_cm = 0;
  let child_age_in_months = 0;
  let premature_conception_in_days = 0;
  let premature_conception_in_weeks = 0;
  let selected_gender = "";

  let error_selected_gender = false;
  let error_child_age_in_months = false;
  let error_child_head_circumference_in_cm = false;
  let error_mother_circumference_in_cm = false;
  let error_father_circumference_in_cm = false;
  let error_premature_conception_in_days = false;
  let error_premature_conception_in_weeks = false;

  let error = "";

  let show_scores = false;
  let show_corrected_scores = false;
  let child_score = 0;
  let corrected_child_score = 0;
  let mother_score = 0;
  let father_score = 0;

	$: outerWidth = 0
	$: innerWidth = 0
	$: outerHeight = 0
	$: innerHeight = 0

  function process_form() {

    let error = false;
    if (selected_gender.length == 0) {
      error_selected_gender = true;
      error = true;
    }

    if (child_age_in_months <= 0) {
      error_child_age_in_months = true;
      error = true;
    }
      
    if (child_head_circumference_in_cm <= 0) {
      error_child_head_circumference_in_cm = true;
      error = true;
    }

    if (mother_circumference_in_cm <= 0) {
      error_mother_circumference_in_cm = true;
      error = true;
    }

    if (father_circumference_in_cm <= 0) {
      error_father_circumference_in_cm = true;
      error = true;
    }

    if (premature_conception_in_days > 7 || premature_conception_in_days < 0) {
      error_premature_conception_in_days = true;
      error = true;
    }

    if (premature_conception_in_weeks > 52 || premature_conception_in_weeks < 0) {
      error_premature_conception_in_weeks = true;
      error = true;
    }

    if (error) {
      return;
    }


    let gender_id = selected_gender == "male" ? 0 : 1;

    invoke("process_form", {
      childAgeMonths: child_age_in_months,
      childHeadCircumferenceCm: child_head_circumference_in_cm,
      motherCircumferenceCm: mother_circumference_in_cm,
      fatherCircumferenceCm: father_circumference_in_cm,
      prematureConceptionWeeks: premature_conception_in_weeks,
      prematureConceptionDays: premature_conception_in_days,
      gender: gender_id}
    ).then((res: any) => {
      show_scores = true;
      if (premature_conception_in_days > 0 || premature_conception_in_weeks > 0) {
        show_corrected_scores = true;
      }
      father_score = res[0];
      mother_score = res[1];
      child_score = res[2];
      corrected_child_score = res[3];
    });

  }

  const dateOnlyRegex = /^([0-9]([0-9]([0-9][1-9]|[1-9]0)|[1-9]00)|[1-9]000)(-(0[1-9]|1[0-2])(-(0[1-9]|[1-2][0-9]|3[0-1])))$/

  function parseDateString(dateString: string) {
    if (dateOnlyRegex.test(dateString)) {
      const utcDate = new Date(dateString)
      const localDate = new Date(utcDate.getTime() + utcDate.getTimezoneOffset() * 60000)
      return localDate  
    }
    return new Date(dateString)
  }

  function update_child_age_in_months() {
    const child_dob = parseDateString(date);
    const today = new Date(new Date().toISOString());
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
    error = "";
    child_age_in_months = 0;
    child_head_circumference_in_cm = 0;
    mother_circumference_in_cm = 0;
    father_circumference_in_cm = 0;
    premature_conception_in_weeks = 0;
    premature_conception_in_days = 0;
  }

</script>
<svelte:window bind:innerWidth bind:outerWidth bind:innerHeight bind:outerHeight />
<div class="flex flex-col justify-center">
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
          <input type="date" class="input input-bordered {error_child_age_in_months ? 'input-error' : ''}" bind:value={date} on:input={update_child_age_in_months}/>
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
              {#if error_premature_conception_in_weeks}
                <div class="label">
                  <span class="label-text-alt text-error">Please enter a valid age in days</span>
                </div>
              {/if}
            </label>
          </div>
        </div>
      </div>

      <button class="btn btn-primary mt-4" on:click={process_form}>Submit</button> <button class="btn btn-secondary mt-4" on:click={reset_form}>Reset</button>
        
    </div>
  </div>


  <ScoreCard show_score={show_scores} show_corrected_score={show_corrected_scores} child_score={child_score} correct_score={corrected_child_score} mother_score={mother_score} father_score={father_score} child_age_in_months={child_age_in_months} gestiational_age_in_weeks={premature_conception_in_weeks}/>
  <WeaverPlot show_score={show_scores} show_corrected_score={show_corrected_scores} child_score={child_score} correct_score={corrected_child_score} mother_score={mother_score} father_score={father_score} chartWidth={innerWidth * 0.8 } chartHeight={innerWidth * 0.8 * 0.78}/>

</div>
