// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn process_form(child_age_months: u32, child_head_circumference_cm: f32, mother_circumference_cm: f32, father_circumference_cm: f32, premature_conception_weeks: u32, premature_conception_days: u32, gender: u32) -> (f64, f64, f64, f64){
    let corrected_age: f32 = get_corrected_age(child_age_months, premature_conception_weeks, premature_conception_days); 
    let (head_circumference, head_std) = get_head_circumference_data(child_age_months as f32, gender);
    let (head_circumference_corrected, head_std_corrected) = get_head_circumference_data(corrected_age, gender);
    
    let child_score: f64 = (child_head_circumference_cm as f64 - head_circumference) / head_std;
    let corrected_child_score: f64 = (child_head_circumference_cm as f64 - head_circumference_corrected) / head_std_corrected;

    //Use adult mean and SD for parents (18 years and older)
    const ADULT_MEAN_MALE: f64 = 55.95;
    const ADULT_STD_MALE: f64 = 1.34;
    const ADULT_MEAN_FEMALE: f64 = 54.94;
    const ADULT_STD_FEMALE: f64 = 1.40;

    // Calculate parent scores
    let dad_score: f64 = (father_circumference_cm as f64 - ADULT_MEAN_MALE) / ADULT_STD_MALE;
    let mom_score: f64 = (mother_circumference_cm as f64 - ADULT_MEAN_FEMALE) / ADULT_STD_FEMALE;
    
    (dad_score, mom_score, child_score, corrected_child_score)
}

// Calculate corrected age
fn get_corrected_age(child_age_months: u32, premature_conception_weeks: u32, premature_conception_days: u32) -> f32 {
    let corrected_age: f32 = if premature_conception_weeks > 0 || premature_conception_days > 0 {
        let gest_age: f32 = (premature_conception_weeks as f32) + (premature_conception_days as f32) / (7 as f32);
        let corrected_age: f32  = (child_age_months as f32)- ((40 as f32)- gest_age) / 4.345;
        corrected_age
    } else {
        // There is no correction factor here.
        child_age_months as f32
    };
    corrected_age
}

fn get_head_circumference_data(child_age_months: f32, gender: u32) -> (f64, f64) {
    let age: Vec<f64> = vec![0.0, 1.0, 3.0, 6.0, 9.0, 12.0, 18.0, 24.0, 36.0, 48.0, 60.0, 72.0, 84.0, 96.0, 108.0, 120.0, 132.0, 144.0, 156.0, 168.0, 180.0, 192.0, 204.0, 216.0];
    let m_head_circumference: Vec<f64> = vec![34.74, 37.30, 40.62, 43.76, 45.75, 47.00, 48.31, 49.19, 50.63, 50.91, 51.41, 51.40, 52.24, 52.35, 52.58, 53.16, 53.25, 53.71, 54.14, 54.59, 54.95, 55.37, 55.77, 55.95];    
    let m_head_std: Vec<f64> = vec![1.33, 1.30, 1.23, 1.29, 1.28, 1.31, 1.36, 1.39, 1.38, 1.39, 1.37, 1.41, 1.52, 1.40, 1.44, 1.41, 1.53, 1.52, 1.57, 1.30, 1.51, 1.11, 1.32, 1.34];

    let f_head_circumference: Vec<f64> = vec![34.02, 36.43, 39.71, 42.68, 44.69, 45.81, 47.27, 48.02, 49.25, 50.10, 50.55, 50.52, 51.46, 51.64, 51.87, 52.15, 52.64, 53.01, 53.70, 54.04, 54.39, 54.64, 54.78, 54.94];
    let f_head_std: Vec<f64> = vec![1.22, 1.22, 1.20, 1.38, 1.30, 1.29, 1.36, 1.29, 1.36, 1.37, 1.32, 1.31, 1.35, 1.44, 1.33, 1.50, 1.39, 1.50, 1.37, 1.39, 1.34, 1.16, 1.35, 1.40];

    // Calculate head circumference
    let head_circumference: f64 = if gender == 0 {
        let head_circumference: f64 = interp::interp(&age, &m_head_circumference, child_age_months as f64);
        head_circumference
    } else {
        let head_circumference: f64 = interp::interp(&age, &f_head_circumference, child_age_months as f64);
        head_circumference
    };
    let head_std: f64 = if gender == 0 {    
        let head_std: f64 = interp::interp(&age, &m_head_std, child_age_months as f64);
        head_std
    } else {
        let head_std: f64 = interp::interp(&age, &f_head_std, child_age_months as f64);
        head_std
    };
    (head_circumference, head_std)

}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![process_form])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
