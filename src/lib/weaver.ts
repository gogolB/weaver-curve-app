import { INTERCEPT, SLOPE, SD_THRESHOLD } from './constants';

/** Expected child z-score for a given parental-average z-score (Weaver regression line). */
export function expectedChildScore(parentalAverage: number): number {
    return INTERCEPT + SLOPE * parentalAverage;
}

/** Whether a child z-score falls within ±SD_THRESHOLD of the expected score. */
export function isWithinExpectedRange(score: number, parentalAverage: number): boolean {
    const expected = expectedChildScore(parentalAverage);
    return score <= expected + SD_THRESHOLD && score >= expected - SD_THRESHOLD;
}

export interface InterpretationInput {
    childScore: number;
    correctedChildScore: number;
    motherScore: number;
    fatherScore: number;
    showCorrectedScore: boolean;
}

export interface Interpretation {
    parentalAverage: number;
    expectedScore: number;
    withinRange: boolean;
    text: string;
}

/**
 * Build the clinical interpretation shown on the PDF report.
 * Uses the corrected score when one is being shown, otherwise the raw child score.
 */
export function buildClinicalInterpretation(input: InterpretationInput): Interpretation {
    const parentalAverage = (input.motherScore + input.fatherScore) / 2;
    const expectedScore = expectedChildScore(parentalAverage);
    const scoreToCheck = input.showCorrectedScore ? input.correctedChildScore : input.childScore;
    const scoreLabel = input.showCorrectedScore ? 'corrected ' : '';
    const withinRange = isWithinExpectedRange(scoreToCheck, parentalAverage);

    const text = withinRange
        ? `The child's ${scoreLabel}head circumference z-score (${scoreToCheck.toFixed(2)}) falls within the expected range (±${SD_THRESHOLD} SD) relative to the parental average (${parentalAverage.toFixed(2)}).`
        : `The child's ${scoreLabel}head circumference z-score (${scoreToCheck.toFixed(2)}) falls outside the expected range (±${SD_THRESHOLD} SD) relative to the parental average (${parentalAverage.toFixed(2)}). Further clinical evaluation may be warranted.`;

    return { parentalAverage, expectedScore, withinRange, text };
}
