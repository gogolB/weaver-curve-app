// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use printpdf::*;
use std::{fs::File, io::Cursor};
use std::io::BufWriter;
use std::path::Path;
use tauri::Manager;
use charming::{
    component::{Axis, Legend}, element::{AxisType, ItemStyle, NameLocation}, series::{Line, Scatter}, Chart, ImageFormat, ImageRenderer
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn calculate_scores(
    child_age_months: u32,
    child_head_circumference_cm: f32,
    mother_circumference_cm: f32,
    father_circumference_cm: f32,
    premature_conception_weeks: u32,
    premature_conception_days: u32,
    gender: u32,
) -> (f64, f64, f64, f64) {
    let corrected_age: f32 = get_corrected_age(
        child_age_months,
        premature_conception_weeks,
        premature_conception_days,
    );
    let (head_circumference, head_std) =
        get_head_circumference_data(child_age_months as f32, gender);
    let (head_circumference_corrected, head_std_corrected) =
        get_head_circumference_data(corrected_age, gender);

    let child_score: f64 = (child_head_circumference_cm as f64 - head_circumference) / head_std;
    let corrected_child_score: f64 =
        (child_head_circumference_cm as f64 - head_circumference_corrected) / head_std_corrected;

    //Use adult mean and SD for parents (18 years and older)
    const ADULT_MEAN_MALE: f64 = 55.95;
    const ADULT_STD_MALE: f64 = 1.34;
    const ADULT_MEAN_FEMALE: f64 = 54.94;
    const ADULT_STD_FEMALE: f64 = 1.40;

    // Calculate parent scores
    let dad_score: f64 = (father_circumference_cm as f64 - ADULT_MEAN_MALE) / ADULT_STD_MALE;
    let mom_score: f64 = (mother_circumference_cm as f64 - ADULT_MEAN_FEMALE) / ADULT_STD_FEMALE;

    (dad_score, mom_score, child_score, corrected_child_score)
}

// Calculate corrected age
fn get_corrected_age(
    child_age_months: u32,
    premature_conception_weeks: u32,
    premature_conception_days: u32,
) -> f32 {
    let corrected_age: f32 = if premature_conception_weeks > 0 || premature_conception_days > 0 {
        let gest_age: f32 =
            (premature_conception_weeks as f32) + (premature_conception_days as f32) / (7 as f32);
        let corrected_age: f32 = (child_age_months as f32) - ((40 as f32) - gest_age) / 4.345;
        corrected_age
    } else {
        // There is no correction factor here.
        child_age_months as f32
    };
    corrected_age
}

fn get_head_circumference_data(child_age_months: f32, gender: u32) -> (f64, f64) {
    let age: Vec<f64> = vec![
        0.0, 1.0, 3.0, 6.0, 9.0, 12.0, 18.0, 24.0, 36.0, 48.0, 60.0, 72.0, 84.0, 96.0, 108.0,
        120.0, 132.0, 144.0, 156.0, 168.0, 180.0, 192.0, 204.0, 216.0,
    ];
    let m_head_circumference: Vec<f64> = vec![
        34.74, 37.30, 40.62, 43.76, 45.75, 47.00, 48.31, 49.19, 50.63, 50.91, 51.41, 51.40, 52.24,
        52.35, 52.58, 53.16, 53.25, 53.71, 54.14, 54.59, 54.95, 55.37, 55.77, 55.95,
    ];
    let m_head_std: Vec<f64> = vec![
        1.33, 1.30, 1.23, 1.29, 1.28, 1.31, 1.36, 1.39, 1.38, 1.39, 1.37, 1.41, 1.52, 1.40, 1.44,
        1.41, 1.53, 1.52, 1.57, 1.30, 1.51, 1.11, 1.32, 1.34,
    ];

    let f_head_circumference: Vec<f64> = vec![
        34.02, 36.43, 39.71, 42.68, 44.69, 45.81, 47.27, 48.02, 49.25, 50.10, 50.55, 50.52, 51.46,
        51.64, 51.87, 52.15, 52.64, 53.01, 53.70, 54.04, 54.39, 54.64, 54.78, 54.94,
    ];
    let f_head_std: Vec<f64> = vec![
        1.22, 1.22, 1.20, 1.38, 1.30, 1.29, 1.36, 1.29, 1.36, 1.37, 1.32, 1.31, 1.35, 1.44, 1.33,
        1.50, 1.39, 1.50, 1.37, 1.39, 1.34, 1.16, 1.35, 1.40,
    ];

    // Calculate head circumference
    let head_circumference: f64 = if gender == 0 {
        let head_circumference: f64 =
            interp::interp(&age, &m_head_circumference, child_age_months as f64);
        head_circumference
    } else {
        let head_circumference: f64 =
            interp::interp(&age, &f_head_circumference, child_age_months as f64);
        head_circumference
    };
    let head_std: f64 = if gender == 0 {
        let head_std: f64 = interp::interp(&age, &m_head_std, child_age_months as f64);
        head_std
    } else {
        let head_std: f64 = interp::interp(&age, &f_head_std, child_age_months as f64);
        head_std
    };
    (head_circumference, head_std)
}

#[tauri::command]
fn make_pdf(
    handle: tauri::AppHandle,
    file_path: &str,
    child_age_months: u32,
    child_head_circumference_cm: f32,
    mother_circumference_cm: f32,
    father_circumference_cm: f32,
    premature_conception_weeks: u32,
    premature_conception_days: u32,
    gender: u32,
) -> f64{
    // Load the font file asset.
    let font_path = handle
        .path()
        .resolve(
            "resources/Roboto/Roboto-Regular.ttf",
            tauri::path::BaseDirectory::Resource,
        )
        .unwrap();
    let font_file = File::open(&font_path).unwrap();

    // Get the file name from the path.
    let path = Path::new(&file_path);
    let file_name = path.file_name().unwrap().to_str().unwrap();

    const PAGE_WIDTH:f32 = 215.9;
    const PAGE_HEIGHT:f32 = 279.4;

    // Create the pdf file.
    let (doc, page1, layer1) = PdfDocument::new(file_name, Mm(PAGE_WIDTH), Mm(PAGE_HEIGHT), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_external_font(font_file).unwrap();

    // Write the title
    let title = "Weaver Curve";
    current_layer.use_text(title, 48.0, Mm(10.0), Mm(250.0), &font);

    // Write the demographic data
    let demographics_text = "Demographics";
    current_layer.use_text(demographics_text, 24.0, Mm(10.0), Mm(240.0), &font);

    let corrected_age: f32 = get_corrected_age(
        child_age_months,
        premature_conception_weeks,
        premature_conception_days,
    );

    let age_text = format!("Age: {} months", child_age_months);
    current_layer.use_text(age_text, 14.0, Mm(10.0), Mm(230.0), &font);

    if (premature_conception_weeks > 0) || (premature_conception_days > 0) {
        let pc_text = format!("Premature Conception: {} weeks {} days", premature_conception_weeks, premature_conception_days);
        current_layer.use_text(pc_text, 14.0, Mm(100.0), Mm(230.0), &font);
        let pc_text = format!("Premature Conception Corrected Age: {} months", corrected_age);
        current_layer.use_text(pc_text, 14.0, Mm(100.0), Mm(220.0), &font);
    }

    // Write the gender data
    let gender_text = format!("Gender: {}", if gender == 0 {"Male"} else {"Female"});
    current_layer.use_text(gender_text, 14.0, Mm(10.0), Mm(220.0), &font);

    // Write the head circumference data
    let head_circumference_text = "Head Circumference";
    current_layer.use_text(head_circumference_text, 24.0, Mm(10.0), Mm(210.0), &font);
    let head_circumference_text = format!("Head Circumference: {} cm", child_head_circumference_cm);
    current_layer.use_text(head_circumference_text, 14.0, Mm(10.0), Mm(200.0), &font);
    let mother_circumference_text = format!("Mother's Circumference: {} cm", mother_circumference_cm);
    current_layer.use_text(mother_circumference_text, 14.0, Mm(10.0), Mm(190.0), &font);
    let father_circumference_text = format!("Father's Circumference: {} cm", father_circumference_cm);
    current_layer.use_text(father_circumference_text, 14.0, Mm(10.0), Mm(180.0), &font);

    // Write the results
    let (dad_score, mom_score, child_score, corrected_child_score) = calculate_scores(child_age_months, child_head_circumference_cm, mother_circumference_cm, father_circumference_cm, premature_conception_weeks, premature_conception_days, gender);
    
    let parental_average = (dad_score + mom_score) / 2.0;

    const INTERCEPT:f64 = 0.138891; 
    const SLOPE:f64 = 0.483034;

    let average_y1: f64 = INTERCEPT + SLOPE * (-5.0 as f64);
    let average_y2: f64 = INTERCEPT + SLOPE * (5.0 as f64);

    let parent_avg_score = INTERCEPT + SLOPE * parental_average;
    let is_normal = child_score < parent_avg_score + (2.0 as f64) && child_score > (parent_avg_score - 2.0 as f64);
    let is_normal_corrected = corrected_child_score <  parent_avg_score + (2.0 as f64) && corrected_child_score > parent_avg_score - (2.0 as f64);
    
    let score_test = "Scores";
    current_layer.use_text(score_test, 24.0, Mm(10.0), Mm(170.0), &font);
    let child_score_text = format!("Child Score: {:.2}", child_score);
    current_layer.use_text(child_score_text, 14.0, Mm(10.0), Mm(160.0), &font);
    if (premature_conception_weeks > 0) || (premature_conception_days > 0) {
        let corrected_child_score_text = format!("Corrected Child Score: {:.2}", corrected_child_score);
        current_layer.use_text(corrected_child_score_text, 14.0, Mm(100.0), Mm(160.0), &font);
    }
    let dad_score_text = format!("Dad Score: {:.2}", dad_score);
    current_layer.use_text(dad_score_text, 14.0, Mm(10.0), Mm(150.0), &font);
    let mom_score_text = format!("Mom Score: {:.2}", mom_score);
    current_layer.use_text(mom_score_text, 14.0, Mm(70.0), Mm(150.0), &font);
    let parental_average = (dad_score + mom_score) / 2.0;
    let parental_average_text = format!("Parental Average: {:.2}", parental_average);
    current_layer.use_text(parental_average_text, 14.0, Mm(130.0), Mm(150.0), &font);

    let color_baseline = if is_normal { "green" } else { "red" };
    let color_corrected = if is_normal_corrected { "green" } else { "red" };
    
    // Generate the Graph
    let mut chart: Chart = Chart::new()
        .legend(Legend::new().top("top"))
        .x_axis(
            Axis::new().type_(AxisType::Value).name("Standard Score (Parental Average)").position("bottom").name_location(NameLocation::Middle),
        )
        .y_axis(
            Axis::new().type_(AxisType::Value).name("Standard Score (Child)").position("left").name_location(NameLocation::Middle).name_rotation(-90.0),
        )
        .series(
            Line::new()
                .name("Average")
                .item_style(ItemStyle::new().color("blue"))
                .data(vec![
                    vec![-5.0 as f64, average_y1], 
                    vec![5.0 as f64, average_y2]
                ])
        )
        .series(
            Line::new()
                .name("+2 SD")
                .item_style(ItemStyle::new().color("orange"))
                .data(vec![
                    vec![-5.0, average_y1 + 2.0],
                    vec![5.0, average_y2 + 2.0],
                ])
        )
        .series(
            Line::new()
                .name("-2 SD")
                .item_style(ItemStyle::new().color("orange"))
                .data(vec![
                    vec![-5.0, average_y1 - 2.0],
                    vec![5.0, average_y2 - 2.0],
                ])
        )
        .series(
            Scatter::new()
                .name("Child Score")
                .item_style(ItemStyle::new().color(color_baseline))
                .data(vec![
                    vec![parental_average, child_score],
                ])
        );
    if premature_conception_weeks > 0 || premature_conception_days > 0 {
        chart = chart.series(
            Scatter::new()
                .name("Corrected Child Score")
                .item_style(ItemStyle::new().color(color_corrected))
                .data(vec![
                    vec![parental_average, corrected_child_score],
                ])  
        );
    }

    let mut renderer: ImageRenderer = ImageRenderer::new(1000, 800);
    let data = renderer.render_format(ImageFormat::Png, &chart).unwrap();
    let mut cursor = Cursor::new(data);

    let decoder = image_crate::codecs::png::PngDecoder::new(&mut cursor).unwrap();
    let image = Image::try_from(decoder).unwrap();
    image.add_to_layer(
        current_layer.clone(),
        ImageTransform {
            translate_x: Some(Mm(10.0)),
            translate_y: Some(Mm(100.0)),
            ..Default::default()
        },
    );


    // Save the PDF
    doc.save(&mut BufWriter::new(
        File::create(path).unwrap(),
    ))
    .unwrap();

    0.0
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![calculate_scores, make_pdf])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
