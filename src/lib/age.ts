/** Weeks per month used by the Weaver gestational-age correction (matches src-tauri/src/main.rs). */
export const WEEKS_PER_MONTH = 4.345;

/** Full-term gestation in weeks. */
export const FULL_TERM_WEEKS = 40;

/**
 * Calculate a child's age in whole months from an ISO date string (YYYY-MM-DD).
 * The month is only counted once the day-of-month has been reached.
 *
 * `today` is injectable for testing.
 */
export function calculateAgeInMonths(dobIsoDate: string, today: Date = new Date()): number {
    const dob = new Date(dobIsoDate + "T00:00:00");
    let months =
        (today.getFullYear() - dob.getFullYear()) * 12 +
        (today.getMonth() - dob.getMonth());
    if (today.getDate() - dob.getDate() < 0) {
        months -= 1;
    }
    return months;
}

/**
 * Gestational-age-corrected age in months.
 * Mirrors `get_corrected_age` in src-tauri/src/main.rs — keep the two in sync.
 * Returns the chronological age unchanged when no prematurity is given.
 */
export function correctedAgeMonths(
    ageMonths: number,
    prematureWeeks: number,
    prematureDays: number,
): number {
    if (prematureWeeks <= 0 && prematureDays <= 0) {
        return ageMonths;
    }
    const gestationalAgeWeeks = prematureWeeks + prematureDays / 7;
    return ageMonths - (FULL_TERM_WEEKS - gestationalAgeWeeks) / WEEKS_PER_MONTH;
}
