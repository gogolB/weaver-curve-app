import { describe, it, expect } from 'vitest';
import { calculateAgeInMonths, correctedAgeMonths } from './age';

describe('calculateAgeInMonths', () => {
    it('returns whole years as 12-month multiples on the birthday', () => {
        expect(calculateAgeInMonths('2024-06-15', new Date('2025-06-15T12:00:00'))).toBe(12);
        expect(calculateAgeInMonths('2023-06-15', new Date('2025-06-15T12:00:00'))).toBe(24);
    });

    it('does not count a month until the day-of-month is reached', () => {
        // One day before the 12-month mark
        expect(calculateAgeInMonths('2024-06-15', new Date('2025-06-14T12:00:00'))).toBe(11);
        // On the mark
        expect(calculateAgeInMonths('2024-06-15', new Date('2025-06-15T12:00:00'))).toBe(12);
        // One day after
        expect(calculateAgeInMonths('2024-06-15', new Date('2025-06-16T12:00:00'))).toBe(12);
    });

    it('handles year boundaries (December DOB checked in January)', () => {
        expect(calculateAgeInMonths('2024-12-20', new Date('2025-01-19T12:00:00'))).toBe(0);
        expect(calculateAgeInMonths('2024-12-20', new Date('2025-01-20T12:00:00'))).toBe(1);
    });

    it('handles a newborn (DOB is today)', () => {
        expect(calculateAgeInMonths('2025-06-15', new Date('2025-06-15T12:00:00'))).toBe(0);
    });

    it('handles a leap-day DOB', () => {
        // Feb 29 2024 → Mar 29 2024 is 1 month; Mar 28 is still 0
        expect(calculateAgeInMonths('2024-02-29', new Date('2024-03-28T12:00:00'))).toBe(0);
        expect(calculateAgeInMonths('2024-02-29', new Date('2024-03-29T12:00:00'))).toBe(1);
        // One year later, Feb 28 2025 (no Feb 29): day 28 < 29, so still 11 months
        expect(calculateAgeInMonths('2024-02-29', new Date('2025-02-28T12:00:00'))).toBe(11);
        expect(calculateAgeInMonths('2024-02-29', new Date('2025-03-01T12:00:00'))).toBe(12);
    });

    it('handles month-length differences (31st DOB checked in a 30-day month)', () => {
        // May 31 DOB: June has 30 days, so on June 30 day diff is -1 → 0 months
        expect(calculateAgeInMonths('2025-05-31', new Date('2025-06-30T12:00:00'))).toBe(0);
        expect(calculateAgeInMonths('2025-05-31', new Date('2025-07-01T12:00:00'))).toBe(1);
    });
});

describe('correctedAgeMonths', () => {
    it('returns chronological age unchanged when not premature', () => {
        expect(correctedAgeMonths(12, 0, 0)).toBe(12);
        expect(correctedAgeMonths(0, 0, 0)).toBe(0);
    });

    it('matches the Rust formula for a 32-week gestation', () => {
        // 12 - (40 - 32) / 4.345 = 12 - 1.84119... = 10.15880...
        expect(correctedAgeMonths(12, 32, 0)).toBeCloseTo(10.1588, 4);
    });

    it('includes days as fractional weeks', () => {
        // gest = 32 + 3/7 = 32.42857; 12 - (40 - 32.42857) / 4.345 = 10.25744...
        expect(correctedAgeMonths(12, 32, 3)).toBeCloseTo(10.2574, 4);
    });

    it('applies correction when only days are given (matches Rust weeks>0 || days>0 branch)', () => {
        expect(correctedAgeMonths(12, 0, 3)).toBeLessThan(12);
    });

    it('is identity at full term (40 weeks, 0 days)', () => {
        expect(correctedAgeMonths(12, 40, 0)).toBe(12);
    });
});
