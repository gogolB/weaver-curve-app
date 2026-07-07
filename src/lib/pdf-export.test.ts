import { describe, it, expect } from 'vitest';
import { generatePdf, type PdfExportOptions } from './pdf-export';

// In jsdom, canvas.getContext('2d') returns null, so SVG rasterization fails and
// generatePdf falls back to the "[Chart could not be embedded]" placeholder.
// That exercises the full document flow (header, demographics, table, interpretation)
// without needing a real canvas implementation.

const baseOpts: PdfExportOptions = {
    svgData: '<svg xmlns="http://www.w3.org/2000/svg" width="700" height="546"></svg>',
    svgWidth: 700,
    svgHeight: 546,
    appVersion: '1.5.1',
    gender: 'male',
    childDob: '2024-06-15',
    childAgeMonths: 12,
    childHeadCircumferenceCm: 47,
    motherCircumferenceCm: 54,
    fatherCircumferenceCm: 56,
    prematureConceptionWeeks: 0,
    prematureConceptionDays: 0,
    childScore: 0.0,
    correctedChildScore: 0.0,
    motherScore: -0.67,
    fatherScore: 0.04,
    showCorrectedScore: false,
    isAbnormal: false,
    isAbnormalCorrected: false,
};

/** Decode PDF bytes for substring assertions (jsPDF streams are uncompressed by default). */
function pdfText(bytes: Uint8Array): string {
    return new TextDecoder('latin1').decode(bytes);
}

describe('generatePdf', () => {
    it('produces a valid PDF byte stream', async () => {
        const bytes = await generatePdf(baseOpts);
        expect(bytes).toBeInstanceOf(Uint8Array);
        expect(bytes.length).toBeGreaterThan(1000);
        // PDF magic header
        expect(pdfText(bytes.slice(0, 5))).toBe('%PDF-');
    });

    it('includes the report title, version, and clinical sections', async () => {
        const text = pdfText(await generatePdf(baseOpts));
        expect(text).toContain('Weaver Curve Report');
        expect(text).toContain('Weaver Curve App v1.5.1');
        expect(text).toContain('Demographics');
        expect(text).toContain('Clinical Measurements');
        expect(text).toContain('Clinical Interpretation');
    });

    it('reports a normal child as within the expected range', async () => {
        const text = pdfText(await generatePdf(baseOpts));
        expect(text).toContain('falls within the expected range');
        expect(text).not.toContain('Further clinical evaluation');
    });

    it('recommends further evaluation for an abnormal score', async () => {
        const text = pdfText(await generatePdf({
            ...baseOpts,
            childScore: 4.5,
            isAbnormal: true,
        }));
        expect(text).toContain('falls outside the expected range');
        expect(text).toContain('Further clinical evaluation may be warranted');
    });

    it('omits corrected-age lines for full-term children', async () => {
        const text = pdfText(await generatePdf(baseOpts));
        expect(text).not.toContain('Corrected Age');
        expect(text).not.toContain('Child Score (Corrected)');
    });

    it('includes corrected age and corrected score row for premature children', async () => {
        const text = pdfText(await generatePdf({
            ...baseOpts,
            prematureConceptionWeeks: 32,
            prematureConceptionDays: 0,
            correctedChildScore: 0.59,
            showCorrectedScore: true,
        }));
        expect(text).toContain('Premature Conception: 32 weeks, 0 days');
        // 12 - (40 - 32) / 4.345 = 10.2 months (rounded to 1 dp)
        expect(text).toContain('Corrected Age: 10.2 months');
        expect(text).toContain('Child Score \\(Corrected\\)'); // parens are escaped in PDF strings
        expect(text).toContain('corrected head circumference z-score');
    });

    it('judges interpretation by the corrected score when shown', async () => {
        const text = pdfText(await generatePdf({
            ...baseOpts,
            prematureConceptionWeeks: 32,
            childScore: 4.5, // abnormal raw score should be ignored...
            correctedChildScore: 0.5, // ...in favor of the normal corrected score
            showCorrectedScore: true,
        }));
        expect(text).toContain('falls within the expected range');
    });

    it('falls back gracefully when the chart cannot be rasterized', async () => {
        // jsdom has no canvas 2d context, so this is the path always taken here —
        // assert the fallback note is present rather than a crash.
        const text = pdfText(await generatePdf(baseOpts));
        expect(text).toContain('Chart could not be embedded');
    });
});
