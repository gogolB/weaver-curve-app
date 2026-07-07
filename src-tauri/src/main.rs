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

    // --- Golden-value tests: exact expected z-scores, hand-computed from the
    // --- lookup tables in constants.rs. These pin the clinical math down so a
    // --- table edit, formula change, or interpolation regression fails loudly.

    #[test]
    fn test_golden_male_12_months_at_table_point() {
        // Age 12 is a table knot: mean 47.00, std 1.31 — a 47.0 cm child is exactly the mean.
        let (dad_score, mom_score, child_score, corrected) =
            calculate_scores(12, 47.0, 54.0, 56.0, 0, 0, Gender::Male).unwrap();

        assert!(child_score.abs() < 1e-9, "child at table mean must be 0, got {child_score}");
        assert_eq!(child_score, corrected, "no prematurity → corrected equals raw");
        // dad: (56.0 - 55.95) / 1.34
        assert!((dad_score - 0.037_313_4).abs() < 1e-6, "dad_score {dad_score}");
        // mom: (54.0 - 54.94) / 1.40
        assert!((mom_score - (-0.671_428_6)).abs() < 1e-6, "mom_score {mom_score}");
    }

    #[test]
    fn test_golden_female_9_months_at_table_point() {
        // Age 9 knot (female): mean 44.69, std 1.30 → (46.0 - 44.69) / 1.30
        let (_, _, child_score, _) =
            calculate_scores(9, 46.0, 54.94, 55.95, 0, 0, Gender::Female).unwrap();
        assert!((child_score - 1.007_692_3).abs() < 1e-6, "child_score {child_score}");
    }

    #[test]
    fn test_golden_male_15_months_interpolated() {
        // Age 15 is midway between knots 12 (47.00, 1.31) and 18 (48.31, 1.36):
        // mean 47.655, std 1.335 → (48.0 - 47.655) / 1.335
        let (_, _, child_score, _) =
            calculate_scores(15, 48.0, 54.0, 56.0, 0, 0, Gender::Male).unwrap();
        assert!((child_score - 0.258_427_0).abs() < 1e-6, "child_score {child_score}");
    }

    #[test]
    fn test_golden_corrected_age_32_weeks() {
        // 12 - (40 - 32) / 4.345 = 10.158804 (must match correctedAgeMonths in src/lib/age.ts)
        let corrected_age = get_corrected_age(12, 32, 0);
        assert!((corrected_age - 10.158_804).abs() < 1e-4, "corrected_age {corrected_age}");
    }

    #[test]
    fn test_golden_corrected_age_32_weeks_3_days() {
        // gest = 32 + 3/7; 12 - (40 - gest) / 4.345 = 10.257444
        let corrected_age = get_corrected_age(12, 32, 3);
        assert!((corrected_age - 10.257_444).abs() < 1e-4, "corrected_age {corrected_age}");
    }

    #[test]
    fn test_golden_corrected_score_premature_32_weeks() {
        // Corrected age 10.158804 interpolates between knots 9 (45.75, 1.28) and
        // 12 (47.00, 1.31): mean 46.232835, std 1.291588 → (47.0 - mean) / std
        let (_, _, child_score, corrected) =
            calculate_scores(12, 47.0, 54.0, 56.0, 32, 0, Gender::Male).unwrap();
        assert!(child_score.abs() < 1e-9, "raw score still judged at chronological age");
        assert!((corrected - 0.593_97).abs() < 1e-3, "corrected {corrected}");
    }

    // --- Validation branches not covered above ---

    #[test]
    fn test_validation_mother_circumference_invalid() {
        for bad in [0.0, -1.0, 75.0] {
            let err = calculate_scores(12, 47.0, bad, 56.0, 0, 0, Gender::Male).unwrap_err();
            assert!(err.to_string().contains("Mother"), "unexpected error: {err}");
        }
    }

    #[test]
    fn test_validation_father_circumference_invalid() {
        for bad in [0.0, -1.0, 75.0] {
            let err = calculate_scores(12, 47.0, 54.0, bad, 0, 0, Gender::Male).unwrap_err();
            assert!(err.to_string().contains("Father"), "unexpected error: {err}");
        }
    }

    #[test]
    fn test_validation_age_boundary_is_inclusive() {
        // 216 months (18 years) is the last table knot and must be accepted...
        assert!(calculate_scores(216, 55.0, 54.0, 56.0, 0, 0, Gender::Male).is_ok());
        // ...while 217 is rejected.
        assert!(calculate_scores(217, 55.0, 54.0, 56.0, 0, 0, Gender::Male).is_err());
    }
}
