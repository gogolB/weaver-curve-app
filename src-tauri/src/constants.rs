/// Maximum supported child age in months (18 years)
pub const MAX_AGE_MONTHS: u32 = 216;

/// Maximum reasonable head circumference in cm
pub const MAX_HEAD_CIRCUMFERENCE_CM: f32 = 70.0;

// Adult reference values for parental score calculation
pub const ADULT_MEAN_MALE: f64 = 55.95;
pub const ADULT_STD_MALE: f64 = 1.34;
pub const ADULT_MEAN_FEMALE: f64 = 54.94;
pub const ADULT_STD_FEMALE: f64 = 1.40;

// Clinical lookup tables: age in months
pub const AGE_MONTHS: &[f64] = &[
    0.0, 1.0, 3.0, 6.0, 9.0, 12.0, 18.0, 24.0, 36.0, 48.0, 60.0, 72.0, 84.0, 96.0, 108.0,
    120.0, 132.0, 144.0, 156.0, 168.0, 180.0, 192.0, 204.0, 216.0,
];

// Male head circumference means by age
pub const MALE_HEAD_CIRCUMFERENCE: &[f64] = &[
    34.74, 37.30, 40.62, 43.76, 45.75, 47.00, 48.31, 49.19, 50.63, 50.91, 51.41, 51.40, 52.24,
    52.35, 52.58, 53.16, 53.25, 53.71, 54.14, 54.59, 54.95, 55.37, 55.77, 55.95,
];

// Male head circumference standard deviations by age
pub const MALE_HEAD_STD: &[f64] = &[
    1.33, 1.30, 1.23, 1.29, 1.28, 1.31, 1.36, 1.39, 1.38, 1.39, 1.37, 1.41, 1.52, 1.40, 1.44,
    1.41, 1.53, 1.52, 1.57, 1.30, 1.51, 1.11, 1.32, 1.34,
];

// Female head circumference means by age
pub const FEMALE_HEAD_CIRCUMFERENCE: &[f64] = &[
    34.02, 36.43, 39.71, 42.68, 44.69, 45.81, 47.27, 48.02, 49.25, 50.10, 50.55, 50.52, 51.46,
    51.64, 51.87, 52.15, 52.64, 53.01, 53.70, 54.04, 54.39, 54.64, 54.78, 54.94,
];

// Female head circumference standard deviations by age
pub const FEMALE_HEAD_STD: &[f64] = &[
    1.22, 1.22, 1.20, 1.38, 1.30, 1.29, 1.36, 1.29, 1.36, 1.37, 1.32, 1.31, 1.35, 1.44, 1.33,
    1.50, 1.39, 1.50, 1.37, 1.39, 1.34, 1.16, 1.35, 1.40,
];
