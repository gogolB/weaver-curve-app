import { correctedAgeMonths } from './age';

const isMocked = typeof import.meta !== 'undefined' && import.meta.env?.VITE_MOCK_TAURI === 'true';

// Mock implementation of calculate_scores (matches Rust logic approximately)
function mockCalculateScores(args: {
    childAgeMonths: number;
    childHeadCircumferenceCm: number;
    motherCircumferenceCm: number;
    fatherCircumferenceCm: number;
    prematureConceptionWeeks: number;
    prematureConceptionDays: number;
    gender: string;
}): [number, number, number, number] {
    const ADULT_MEAN_MALE = 55.95;
    const ADULT_STD_MALE = 1.34;
    const ADULT_MEAN_FEMALE = 54.94;
    const ADULT_STD_FEMALE = 1.40;

    // Simplified mock: use 12-month approximation for child lookup
    const childMean = args.gender === 'male' ? 47.0 : 45.81;
    const childStd = 1.31;

    const correctedAge = correctedAgeMonths(
        args.childAgeMonths,
        args.prematureConceptionWeeks,
        args.prematureConceptionDays,
    );

    const childScore = (args.childHeadCircumferenceCm - childMean) / childStd;
    const correctedChildScore = correctedAge !== args.childAgeMonths
        ? (args.childHeadCircumferenceCm - childMean) / childStd * 0.95
        : childScore;

    const dadScore = (args.fatherCircumferenceCm - ADULT_MEAN_MALE) / ADULT_STD_MALE;
    const momScore = (args.motherCircumferenceCm - ADULT_MEAN_FEMALE) / ADULT_STD_FEMALE;

    return [dadScore, momScore, childScore, correctedChildScore];
}

export async function invokeCalculateScores(args: {
    childAgeMonths: number;
    childHeadCircumferenceCm: number;
    motherCircumferenceCm: number;
    fatherCircumferenceCm: number;
    prematureConceptionWeeks: number;
    prematureConceptionDays: number;
    gender: string;
}): Promise<[number, number, number, number]> {
    if (isMocked) {
        return mockCalculateScores(args);
    }
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke('calculate_scores', args);
}
