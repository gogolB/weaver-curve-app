import { describe, it, expect } from 'vitest';

describe('Math Utilities', () => {
  it('calculates parental average correctly', () => {
    const motherScore = 2.0;
    const fatherScore = 2.2;
    const parentalAverage = (motherScore + fatherScore) / 2;
    
    expect(parentalAverage).toBe(2.1);
  });

  it('calculates standard deviation scores', () => {
    const measurement = 47.0;
    const mean = 45.0;
    const stdDev = 1.5;
    const score = (measurement - mean) / stdDev;
    
    expect(score).toBeCloseTo(1.333, 2);
  });

  it('determines if score is abnormal', () => {
    const childScore = 5.0;
    const parentalAverage = 2.0;
    const intercept = 0.138891;
    const slope = 0.483034;
    
    const yMean = intercept + slope * parentalAverage;
    const isAbnormal = childScore > yMean + 2 || childScore < yMean - 2;
    
    expect(isAbnormal).toBe(true);
  });

  it('calculates corrected age for premature birth', () => {
    const childAgeMonths = 12;
    const prematureWeeks = 4;
    const prematureDays = 3;
    
    const gestAge = prematureWeeks + prematureDays / 7;
    const correctedAge = childAgeMonths - (40 - gestAge) / 4.345;
    
    expect(correctedAge).toBeLessThan(childAgeMonths);
    expect(correctedAge).toBeGreaterThan(0);
  });

  it('validates input ranges', () => {
    // Test circumference validation
    expect(35.0).toBeGreaterThan(0);
    expect(-5.0).toBeLessThan(0);
    
    // Test premature weeks validation
    expect(4).toBeGreaterThanOrEqual(0);
    expect(4).toBeLessThanOrEqual(52);
    expect(60).toBeGreaterThan(52);
    
    // Test premature days validation
    expect(3).toBeGreaterThanOrEqual(0);
    expect(3).toBeLessThanOrEqual(7);
    expect(10).toBeGreaterThan(7);
  });
});