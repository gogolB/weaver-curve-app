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
