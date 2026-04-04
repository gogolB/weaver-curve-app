import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import ScoreCard from './score_card.svelte';

describe('ScoreCard', () => {
    it('shows skeleton when show_score is false', () => {
        render(ScoreCard, {
            props: {
                show_score: false,
                show_corrected_score: false,
                child_score: 0,
                correct_score: 0,
                mother_score: 0,
                father_score: 0,
                child_age_in_months: 12,
            }
        });
        const skeletons = document.querySelectorAll('.skeleton');
        expect(skeletons.length).toBeGreaterThan(0);
    });

    it('displays scores when show_score is true', () => {
        render(ScoreCard, {
            props: {
                show_score: true,
                show_corrected_score: false,
                child_score: 1.5,
                correct_score: 1.3,
                mother_score: 0.5,
                father_score: 0.8,
                child_age_in_months: 12,
            }
        });
        expect(screen.getByText('1.50')).toBeTruthy();
        expect(screen.getByText('0.65')).toBeTruthy(); // parental average (0.5+0.8)/2
    });

    it('shows corrected score when enabled', () => {
        render(ScoreCard, {
            props: {
                show_score: true,
                show_corrected_score: true,
                child_score: 1.5,
                correct_score: 1.2,
                mother_score: 0.5,
                father_score: 0.8,
                child_age_in_months: 12,
            }
        });
        expect(screen.getByText('Child (corrected)')).toBeTruthy();
        expect(screen.getByText('1.20')).toBeTruthy();
    });

    it('shows warning for invalid premature age', () => {
        render(ScoreCard, {
            props: {
                show_score: true,
                show_corrected_score: false,
                child_score: 1.5,
                correct_score: 1.3,
                mother_score: 0.5,
                father_score: 0.8,
                child_age_in_months: 2,
                gestational_age_in_weeks: 30,
            }
        });
        expect(screen.getByText('Warning')).toBeTruthy();
        expect(screen.getByText('Results may not be accurate for current premature age.')).toBeTruthy();
    });

    it('applies error styling for abnormal scores', () => {
        render(ScoreCard, {
            props: {
                show_score: true,
                show_corrected_score: false,
                child_score: 5.0,
                correct_score: 5.0,
                mother_score: 0.0,
                father_score: 0.0,
                child_age_in_months: 12,
            }
        });
        const scoreValue = screen.getByText('5.00');
        expect(scoreValue.classList.contains('text-error')).toBe(true);
    });
});
