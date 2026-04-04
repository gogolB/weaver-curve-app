import { jsPDF } from 'jspdf';
import { INTERCEPT, SLOPE, SD_THRESHOLD } from './constants';

export interface PdfExportOptions {
    svgData: string;
    svgWidth: number;
    svgHeight: number;
    appVersion: string;
    gender: string;
    childDob: string;
    childAgeMonths: number;
    childHeadCircumferenceCm: number;
    motherCircumferenceCm: number;
    fatherCircumferenceCm: number;
    prematureConceptionWeeks: number;
    prematureConceptionDays: number;
    childScore: number;
    correctedChildScore: number;
    motherScore: number;
    fatherScore: number;
    showCorrectedScore: boolean;
    isAbnormal: boolean;
    isAbnormalCorrected: boolean;
}

/**
 * Rasterize an SVG string to a PNG data URL via an offscreen canvas.
 * Uses 2x scale for crisp rendering in print.
 */
function svgToImageDataUrl(svgData: string, width: number, height: number): Promise<string> {
    return new Promise((resolve, reject) => {
        const scale = 2;
        const canvas = document.createElement('canvas');
        canvas.width = width * scale;
        canvas.height = height * scale;
        const ctx = canvas.getContext('2d');
        if (!ctx) {
            reject(new Error('Could not get canvas 2d context'));
            return;
        }

        const img = new Image();
        img.onload = () => {
            ctx.drawImage(img, 0, 0, canvas.width, canvas.height);
            resolve(canvas.toDataURL('image/png'));
        };
        img.onerror = (e) => {
            reject(new Error(`Failed to load SVG as image: ${e}`));
        };

        // Encode SVG as a data URL for the Image element
        const svgBlob = new Blob([svgData], { type: 'image/svg+xml;charset=utf-8' });
        img.src = URL.createObjectURL(svgBlob);
    });
}

export async function generatePdf(opts: PdfExportOptions): Promise<Uint8Array> {
    const doc = new jsPDF({ orientation: 'portrait', unit: 'mm', format: 'a4' });
    const pageWidth = doc.internal.pageSize.getWidth();
    const margin = 15;
    let y = margin;

    // --- Header ---
    doc.setFontSize(8);
    doc.setTextColor(128);
    const today = new Date().toLocaleDateString('en-US', {
        weekday: 'long', year: 'numeric', month: 'short', day: 'numeric'
    });
    doc.text(`Weaver Curve App v${opts.appVersion}`, margin, y);
    doc.text(`Date Generated: ${today}`, pageWidth - margin, y, { align: 'right' });

    // --- Title ---
    y += 12;
    doc.setFontSize(18);
    doc.setTextColor(0);
    doc.text('Weaver Curve Report', pageWidth / 2, y, { align: 'center' });

    // --- Patient ID line ---
    y += 10;
    doc.setFontSize(10);
    doc.setTextColor(80);
    doc.text('Patient ID: _______________________________________________', margin, y);

    // --- Demographics ---
    y += 12;
    doc.setFontSize(13);
    doc.setTextColor(0);
    doc.text('Demographics', margin, y);
    y += 2;
    doc.setDrawColor(0);
    doc.line(margin, y, pageWidth - margin, y);
    y += 7;

    doc.setFontSize(10);
    doc.setTextColor(40);

    const demoLines: string[] = [
        `Gender: ${opts.gender.charAt(0).toUpperCase() + opts.gender.slice(1)}`,
        `DOB: ${opts.childDob}`,
        `Age: ${opts.childAgeMonths} months`,
    ];
    if (opts.prematureConceptionWeeks > 0 || opts.prematureConceptionDays > 0) {
        demoLines.push(`Premature Conception: ${opts.prematureConceptionWeeks} weeks, ${opts.prematureConceptionDays} days`);
        const gestAge = opts.prematureConceptionWeeks + opts.prematureConceptionDays / 7;
        const correctedAge = opts.childAgeMonths - (40 - gestAge) / 4.345;
        demoLines.push(`Corrected Age: ${correctedAge.toFixed(1)} months`);
    }
    for (const line of demoLines) {
        doc.text(`\u2022 ${line}`, margin + 2, y);
        y += 5.5;
    }

    // --- Clinical Measurements ---
    y += 5;
    doc.setFontSize(13);
    doc.setTextColor(0);
    doc.text('Clinical Measurements', margin, y);
    y += 2;
    doc.line(margin, y, pageWidth - margin, y);
    y += 7;

    doc.setFontSize(10);
    doc.setTextColor(40);
    doc.text(`\u2022 Child Head Circumference: ${opts.childHeadCircumferenceCm} cm`, margin + 2, y); y += 5.5;
    doc.text(`\u2022 Mother Head Circumference: ${opts.motherCircumferenceCm} cm`, margin + 2, y); y += 5.5;
    doc.text(`\u2022 Father Head Circumference: ${opts.fatherCircumferenceCm} cm`, margin + 2, y); y += 5.5;

    // --- Scores Table ---
    y += 5;
    doc.setFontSize(13);
    doc.setTextColor(0);
    doc.text('Scores', margin, y);
    y += 2;
    doc.line(margin, y, pageWidth - margin, y);
    y += 3;

    const parentalAverage = (opts.motherScore + opts.fatherScore) / 2;
    const tableData: [string, string][] = [
        ['Child Score', opts.childScore.toFixed(2)],
    ];
    if (opts.showCorrectedScore) {
        tableData.push(['Child Score (Corrected)', opts.correctedChildScore.toFixed(2)]);
    }
    tableData.push(
        ['Mother Score', opts.motherScore.toFixed(2)],
        ['Father Score', opts.fatherScore.toFixed(2)],
        ['Parental Average', parentalAverage.toFixed(2)],
    );

    // Draw table
    const colWidth = (pageWidth - 2 * margin) / 2;
    const rowHeight = 8;
    doc.setFontSize(9);

    // Header row
    y += 2;
    doc.setFillColor(60, 60, 60);
    doc.rect(margin, y, pageWidth - 2 * margin, rowHeight, 'F');
    doc.setTextColor(255);
    doc.text('Metric', margin + 3, y + 5.5);
    doc.text('Value', margin + colWidth + 3, y + 5.5);
    y += rowHeight;

    // Data rows
    for (const [label, value] of tableData) {
        const isAbnormalRow =
            (label === 'Child Score' && opts.isAbnormal) ||
            (label === 'Child Score (Corrected)' && opts.isAbnormalCorrected);

        if (isAbnormalRow) {
            doc.setFillColor(255, 220, 220);
        } else {
            doc.setFillColor(220, 255, 220);
        }

        if (label === 'Child Score' || label === 'Child Score (Corrected)') {
            doc.rect(margin, y, pageWidth - 2 * margin, rowHeight, 'F');
        }

        doc.setTextColor(40);
        doc.text(label, margin + 3, y + 5.5);
        doc.text(value, margin + colWidth + 3, y + 5.5);

        // Row border
        doc.setDrawColor(200);
        doc.line(margin, y + rowHeight, pageWidth - margin, y + rowHeight);
        y += rowHeight;
    }

    // --- Chart ---
    y += 8;
    doc.setFontSize(13);
    doc.setTextColor(0);
    doc.text('Weaver Diagram', margin, y);
    y += 2;
    doc.line(margin, y, pageWidth - margin, y);
    y += 5;

    // Rasterize SVG to PNG via canvas, then embed as image
    const chartPdfWidth = pageWidth - 2 * margin;
    const chartPdfHeight = chartPdfWidth * (opts.svgHeight / opts.svgWidth);

    // Check if chart fits on current page; if not, add a new page
    if (y + chartPdfHeight > doc.internal.pageSize.getHeight() - 30) {
        doc.addPage();
        y = margin;
    }

    try {
        const imageDataUrl = await svgToImageDataUrl(opts.svgData, opts.svgWidth, opts.svgHeight);
        doc.addImage(imageDataUrl, 'PNG', margin, y, chartPdfWidth, chartPdfHeight);
    } catch (err) {
        console.error('SVG rasterization failed:', err);
        doc.setFontSize(10);
        doc.setTextColor(128);
        doc.text('[Chart could not be embedded — see application for visual]', margin, y + 10);
    }
    y += chartPdfHeight + 8;

    // --- Clinical Interpretation ---
    if (y + 30 > doc.internal.pageSize.getHeight() - 30) {
        doc.addPage();
        y = margin;
    }

    doc.setFontSize(13);
    doc.setTextColor(0);
    doc.text('Clinical Interpretation', margin, y);
    y += 2;
    doc.line(margin, y, pageWidth - margin, y);
    y += 7;

    doc.setFontSize(10);
    doc.setTextColor(40);

    const expectedScore = INTERCEPT + SLOPE * parentalAverage;
    const scoreToCheck = opts.showCorrectedScore ? opts.correctedChildScore : opts.childScore;
    const scoreLabel = opts.showCorrectedScore ? "corrected " : "";
    const withinRange = scoreToCheck <= expectedScore + SD_THRESHOLD && scoreToCheck >= expectedScore - SD_THRESHOLD;

    const interpretation = withinRange
        ? `The child's ${scoreLabel}head circumference z-score (${scoreToCheck.toFixed(2)}) falls within the expected range (\u00B1${SD_THRESHOLD} SD) relative to the parental average (${parentalAverage.toFixed(2)}).`
        : `The child's ${scoreLabel}head circumference z-score (${scoreToCheck.toFixed(2)}) falls outside the expected range (\u00B1${SD_THRESHOLD} SD) relative to the parental average (${parentalAverage.toFixed(2)}). Further clinical evaluation may be warranted.`;

    const splitInterpretation = doc.splitTextToSize(interpretation, pageWidth - 2 * margin);
    doc.text(splitInterpretation, margin, y);
    y += splitInterpretation.length * 5 + 5;

    // --- Disclaimer ---
    doc.setFontSize(7);
    doc.setTextColor(128);
    const disclaimer = `This report is generated by Weaver Curve App v${opts.appVersion}. It is intended as a clinical decision support tool and does not constitute a diagnosis.`;
    const splitDisclaimer = doc.splitTextToSize(disclaimer, pageWidth - 2 * margin);
    doc.text(splitDisclaimer, margin, doc.internal.pageSize.getHeight() - 10);

    // Return as Uint8Array
    const arrayBuffer = doc.output('arraybuffer');
    return new Uint8Array(arrayBuffer);
}
