import { describe, it, expect } from 'vitest';
import {
    expectedChildScore,
    isWithinExpectedRange,
    buildClinicalInterpretation,
} from './weaver';
import { INTERCEPT, SLOPE, SD_THRESHOLD } from './constants';

describe('expectedChildScore', () => {
    it('equals the intercept when the parental average is zero', () => {
        expect(expectedChildScore(0)).toBe(INTERCEPT);
    });

    it('follows the regression line', () => {
        expect(expectedChildScore(2)).toBeCloseTo(INTERCEPT + 2 * SLOPE, 10);
        expect(expectedChildScore(-2)).toBeCloseTo(INTERCEPT - 2 * SLOPE, 10);
    });
});

describe('isWithinExpectedRange', () => {
    it('accepts a score exactly on the expected line', () => {
        expect(isWithinExpectedRange(expectedChildScore(1), 1)).toBe(true);
    });

    it('accepts scores exactly at the ±SD boundary (inclusive)', () => {
        const expected = expectedChildScore(0);
        expect(isWithinExpectedRange(expected + SD_THRESHOLD, 0)).toBe(true);
        expect(isWithinExpectedRange(expected - SD_THRESHOLD, 0)).toBe(true);
    });

    it('rejects scores just outside the ±SD boundary', () => {
        const expected = expectedChildScore(0);
        expect(isWithinExpectedRange(expected + SD_THRESHOLD + 0.01, 0)).toBe(false);
        expect(isWithinExpectedRange(expected - SD_THRESHOLD - 0.01, 0)).toBe(false);
    });

    it('shifts the acceptable band with the parental average', () => {
        // A score of 2.0 is abnormal for average parents...
        expect(isWithinExpectedRange(2.5, 0)).toBe(false);
        // ...but normal when the parents are large-headed (expected ≈ 0.14 + 0.48*4 ≈ 2.07)
        expect(isWithinExpectedRange(2.5, 4)).toBe(true);
    });
});

describe('buildClinicalInterpretation', () => {
    const base = {
        childScore: 0.5,
        correctedChildScore: 1.0,
        motherScore: 0.4,
        fatherScore: 0.6,
        showCorrectedScore: false,
    };

    it('computes the parental average and expected score', () => {
        const r = buildClinicalInterpretation(base);
        expect(r.parentalAverage).toBeCloseTo(0.5, 10);
        expect(r.expectedScore).toBeCloseTo(INTERCEPT + SLOPE * 0.5, 10);
    });

    it('reports within-range text for a normal score', () => {
        const r = buildClinicalInterpretation(base);
        expect(r.withinRange).toBe(true);
        expect(r.text).toContain('falls within the expected range');
        expect(r.text).not.toContain('Further clinical evaluation');
        expect(r.text).toContain('(0.50)'); // the score that was checked
    });

    it('reports out-of-range text and recommends evaluation for an abnormal score', () => {
        const r = buildClinicalInterpretation({ ...base, childScore: 4.0 });
        expect(r.withinRange).toBe(false);
        expect(r.text).toContain('falls outside the expected range');
        expect(r.text).toContain('Further clinical evaluation may be warranted');
    });

    it('uses the corrected score when showCorrectedScore is true', () => {
        const r = buildClinicalInterpretation({
            ...base,
            childScore: 4.0, // abnormal, but should be ignored
            correctedChildScore: 0.5, // normal
            showCorrectedScore: true,
        });
        expect(r.withinRange).toBe(true);
        expect(r.text).toContain("corrected head circumference z-score");
    });

    it('uses the raw score when showCorrectedScore is false', () => {
        const r = buildClinicalInterpretation({
            ...base,
            childScore: 0.5, // normal
            correctedChildScore: 4.0, // abnormal, but should be ignored
            showCorrectedScore: false,
        });
        expect(r.withinRange).toBe(true);
        expect(r.text).not.toContain('corrected');
    });
});
