import { test, expect } from '@playwright/test';

test.describe('Weaver Curve App', () => {

    test('shows form with all required fields', async ({ page }) => {
        await page.goto('/');
        await expect(page.locator('h2:has-text("Weaver Curve")')).toBeVisible();
        await expect(page.locator('select[name="sex"]')).toBeVisible();
        await expect(page.getByText('Child DOB')).toBeVisible();
        await expect(page.getByText('Child Age')).toBeVisible();
        await expect(page.getByRole('button', { name: 'Submit' })).toBeVisible();
        await expect(page.getByRole('button', { name: 'Reset' })).toBeVisible();
    });

    test('shows validation errors on empty submit', async ({ page }) => {
        await page.goto('/');
        await page.getByRole('button', { name: 'Submit' }).click();
        await expect(page.getByText('Please select a gender')).toBeVisible();
    });

    test('happy path — calculate and display scores', async ({ page }) => {
        await page.goto('/');

        // Fill form
        await page.selectOption('select[name="sex"]', 'male');

        // Fill child age
        const childAgeInput = page.locator('input[type="number"]').nth(0);
        await childAgeInput.fill('12');

        // Fill head circumferences (child, mother, father)
        const numberInputs = page.locator('input[type="number"]');
        await numberInputs.nth(1).fill('47');  // child head circ
        await numberInputs.nth(2).fill('54');  // mother head circ
        await numberInputs.nth(3).fill('56');  // father head circ

        // Submit
        await page.getByRole('button', { name: 'Submit' }).click();

        // Verify scores appear
        await expect(page.locator('h2:has-text("Weaver Scores")')).toBeVisible();
        await expect(page.locator('.stat-value').first()).toBeVisible();

        // Verify SVG chart rendered
        await expect(page.locator('svg')).toBeVisible();
        await expect(page.locator('svg circle')).toBeVisible();
    });

    test('reset clears form and hides scores', async ({ page }) => {
        await page.goto('/');

        // Fill and submit
        await page.selectOption('select[name="sex"]', 'female');
        const numberInputs = page.locator('input[type="number"]');
        await numberInputs.nth(0).fill('24');
        await numberInputs.nth(1).fill('48');
        await numberInputs.nth(2).fill('55');
        await numberInputs.nth(3).fill('57');
        await page.getByRole('button', { name: 'Submit' }).click();
        await expect(page.locator('.stat-value').first()).toBeVisible();

        // Reset
        await page.getByRole('button', { name: 'Reset' }).click();

        // Verify skeleton placeholders are back
        await expect(page.locator('.skeleton').first()).toBeVisible();
    });

    test('gender selection works', async ({ page }) => {
        await page.goto('/');
        const select = page.locator('select[name="sex"]');

        await select.selectOption('male');
        await expect(select).toHaveValue('male');

        await select.selectOption('female');
        await expect(select).toHaveValue('female');
    });

    test('print button appears after score calculation', async ({ page }) => {
        await page.goto('/');

        // Before calculation — no print button (it's inside the {#if show_score} block)
        await expect(page.getByRole('button', { name: 'Print' })).not.toBeVisible();

        // Fill and submit
        await page.selectOption('select[name="sex"]', 'male');
        const numberInputs = page.locator('input[type="number"]');
        await numberInputs.nth(0).fill('12');
        await numberInputs.nth(1).fill('47');
        await numberInputs.nth(2).fill('54');
        await numberInputs.nth(3).fill('56');
        await page.getByRole('button', { name: 'Submit' }).click();

        // After calculation — print button visible
        await expect(page.getByRole('button', { name: 'Print' })).toBeVisible();
    });

    test('validation errors clear on resubmit', async ({ page }) => {
        await page.goto('/');

        // Submit empty
        await page.getByRole('button', { name: 'Submit' }).click();
        await expect(page.getByText('Please select a gender')).toBeVisible();

        // Fix and resubmit
        await page.selectOption('select[name="sex"]', 'male');
        const numberInputs = page.locator('input[type="number"]');
        await numberInputs.nth(0).fill('12');
        await numberInputs.nth(1).fill('47');
        await numberInputs.nth(2).fill('54');
        await numberInputs.nth(3).fill('56');
        await page.getByRole('button', { name: 'Submit' }).click();

        // Errors should be gone
        await expect(page.getByText('Please select a gender')).not.toBeVisible();
    });
});
