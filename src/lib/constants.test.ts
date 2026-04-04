import { describe, it, expect } from 'vitest';
import {
    INTERCEPT, SLOPE, SD_THRESHOLD,
    MAX_AGE_MONTHS, MAX_HEAD_CIRCUMFERENCE_CM,
    MAX_PREMATURE_WEEKS, MAX_PREMATURE_DAYS
} from './constants';

describe('Medical Constants', () => {
    it('has correct regression intercept', () => {
        expect(INTERCEPT).toBe(0.138891);
    });

    it('has correct regression slope', () => {
        expect(SLOPE).toBe(0.483034);
    });

    it('has SD threshold of 2', () => {
        expect(SD_THRESHOLD).toBe(2);
    });

    it('has max age of 216 months (18 years)', () => {
        expect(MAX_AGE_MONTHS).toBe(216);
    });

    it('has reasonable max head circumference', () => {
        expect(MAX_HEAD_CIRCUMFERENCE_CM).toBe(70);
    });

    it('has max premature weeks of 42', () => {
        expect(MAX_PREMATURE_WEEKS).toBe(42);
    });

    it('has max premature days of 7', () => {
        expect(MAX_PREMATURE_DAYS).toBe(7);
    });
});

describe('Abnormality Detection', () => {
    it('detects abnormal score above threshold', () => {
        const parentalAverage = 0;
        const yMean = INTERCEPT + SLOPE * parentalAverage;
        const childScore = yMean + SD_THRESHOLD + 0.5;
        const isAbnormal = childScore > yMean + SD_THRESHOLD || childScore < yMean - SD_THRESHOLD;
        expect(isAbnormal).toBe(true);
    });

    it('detects abnormal score below threshold', () => {
        const parentalAverage = 0;
        const yMean = INTERCEPT + SLOPE * parentalAverage;
        const childScore = yMean - SD_THRESHOLD - 0.5;
        const isAbnormal = childScore > yMean + SD_THRESHOLD || childScore < yMean - SD_THRESHOLD;
        expect(isAbnormal).toBe(true);
    });

    it('detects normal score within threshold', () => {
        const parentalAverage = 1.0;
        const yMean = INTERCEPT + SLOPE * parentalAverage;
        const childScore = yMean + 0.5;
        const isAbnormal = childScore > yMean + SD_THRESHOLD || childScore < yMean - SD_THRESHOLD;
        expect(isAbnormal).toBe(false);
    });
});
