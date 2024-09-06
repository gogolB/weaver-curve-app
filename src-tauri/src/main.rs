// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use plotly::color::NamedColor;
use plotly::common::{DashType, Line, Mode};
use plotly::common::{Marker, MarkerSymbol};
use plotly::layout::{Axis, Layout};
use plotly::{Plot, Scatter};
use std::fs::{self};
use std::vec;
use tauri::Manager;
use typst::eval::Tracer;
use typst::foundations::{Bytes, Dict, IntoValue, Smart};
use typst::text::Font;
use typst_as_lib::TypstTemplate;

use derive_typst_intoval::{IntoDict, IntoValue};
use tempfile::Builder;

use charming::{Chart, ImageRenderer, series::{Line, Scatter}};

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
    app_version: &str,
    child_dob: &str,
) -> Result<String, Error> {
    // Load the font file asset.
    let template_path = handle
        .path()
        .resolve(
            "resources/templates/template.typ",
            tauri::path::BaseDirectory::Resource,
        )
        .expect("Could not find template file");

    let template = fs::read_to_string(template_path).expect("Could not read template file.");

    let font_files = vec![
        "Roboto-Black.ttf",
        "Roboto-BlackItalic.ttf",
        "Roboto-Bold.ttf",
        "Roboto-BoldItalic.ttf",
        "Roboto-Italic.ttf",
        "Roboto-Regular.ttf",
    ];
    let mut fonts = Vec::new();
    for font_file in font_files {
        let font_path = handle
            .path()
            .resolve(
                format!("resources/fonts/Roboto/{}", font_file),
                tauri::path::BaseDirectory::Resource,
            )
            .expect("Could not find font file");
        let font_data = fs::read(font_path).expect("Could not read font file");
        let font_data = Bytes::from(font_data);
        let font = Font::new(font_data, 0).expect("Could not parse font file");
        fonts.push(font);
    }

    let corrected_age: f32 = get_corrected_age(
        child_age_months,
        premature_conception_weeks,
        premature_conception_days,
    );

    // Write the results
    let (dad_score, mom_score, child_score, corrected_child_score) = calculate_scores(
        child_age_months,
        child_head_circumference_cm,
        mother_circumference_cm,
        father_circumference_cm,
        premature_conception_weeks,
        premature_conception_days,
        gender,
    );

    let parental_average = (dad_score + mom_score) / 2.0;

    const INTERCEPT: f64 = 0.138891;
    const SLOPE: f64 = 0.483034;

    let average_y1: f64 = INTERCEPT + SLOPE * (-5.0 as f64);
    let average_y2: f64 = INTERCEPT + SLOPE * (5.0 as f64);

    let parent_avg_score = INTERCEPT + SLOPE * parental_average;
    let is_normal = child_score < parent_avg_score + (2.0 as f64)
        && child_score > (parent_avg_score - 2.0 as f64);
    let is_normal_corrected = corrected_child_score < parent_avg_score + (2.0 as f64)
        && corrected_child_score > parent_avg_score - (2.0 as f64);

    let color_baseline = if is_normal {
        NamedColor::Green
    } else {
        NamedColor::Red
    };
    let color_corrected = if is_normal_corrected {
        NamedColor::Green
    } else {
        NamedColor::Red
    };

    // Generate the Graph
    let mut plot = Plot::new();
    let layout = Layout::new()
        .title("Weaver Curve")
        .x_axis(
            Axis::new()
                .title("Parental Average")
                .range(vec![-5.0, 5.0])
                .position(0.0)
                .anchor("y"),
        )
        .y_axis(
            Axis::new()
                .title("Child Score")
                .range(vec![-5.0, 5.0])
                .position(0.0)
                .anchor("x"),
        );
    let normal_trace = Scatter::new(vec![parental_average], vec![child_score])
        .name("Child Score")
        .marker(Marker::new().color(color_baseline).size(10));
    plot.add_trace(normal_trace);

    if (premature_conception_weeks > 0) || (premature_conception_days > 0) {
        let corrected_trace = Scatter::new(vec![parental_average], vec![corrected_child_score])
            .name("Child Score (Corrected)")
            .marker(
                Marker::new()
                    .color(color_corrected)
                    .symbol(MarkerSymbol::Diamond)
                    .size(12),
            );
        plot.add_trace(corrected_trace);
    }

    let avg_trace = Scatter::new(vec![-5.0, 5.0], vec![average_y1, average_y2])
        .name("Parental Average")
        .mode(Mode::Lines)
        .line(Line::new().color(NamedColor::Blue));
    plot.add_trace(avg_trace);
    let sd_2m_trace = Scatter::new(vec![-5.0, 5.0], vec![average_y1 - 2.0, average_y2 - 2.0])
        .name("- 2 SD")
        .mode(Mode::Lines)
        .line(Line::new().color(NamedColor::Orange).dash(DashType::Dash));
    plot.add_trace(sd_2m_trace);
    let sd_2p_trace = Scatter::new(vec![-5.0, 5.0], vec![average_y1 + 2.0, average_y2 + 2.0])
        .name("+ 2 SD")
        .mode(Mode::Lines)
        .line(Line::new().color(NamedColor::Orange).dash(DashType::Dash));
    plot.add_trace(sd_2p_trace);
    plot.set_layout(layout);


    let named_temp_file = Builder::new().prefix("weaver_curve").suffix(".png").tempfile().expect("Could not create temporary file.");
    let path = named_temp_file.into_temp_path();
    let image_file_path = path.to_str().expect("Could not convert path to string.");
    plot.write_image(image_file_path, plotly::ImageFormat::PNG, 1024, 720, 1.0);
    
    let c = ContentData {
        gender: if gender == 0 {
            "Male".to_string()
        } else {
            "Female".to_string()
        },
        age: child_age_months,
        head_circumference: child_head_circumference_cm.to_string(),
        mother_head_circumference: mother_circumference_cm.to_string(),
        father_head_circumference: father_circumference_cm.to_string(),
        premature_weeks: Some(premature_conception_weeks),
        premature_days: Some(premature_conception_days),
        corrected_age: corrected_age.to_string(),
        child_score: format!("{:.2}", child_score),
        corrected_child_score: format!("{:.2}", corrected_child_score),
        father_score: format!("{:.2}", dad_score),
        mother_score: format!("{:.2}", mom_score),
        parental_average: format!("{:.2}", parental_average),
        version: app_version.to_string(),
        dob: child_dob.to_string(),
    };

    let image_bytes: Vec<u8> = fs::read(image_file_path).expect("Could not read image.");

    let content = Content { v: c };

    let template = TypstTemplate::new(fonts, template)
        .with_static_file_resolver([("./images/graph.png", image_bytes)]);
    let mut tracer = Tracer::new();

    let doc = template
        .compile_with_input(&mut tracer, content)
        .expect("typst::compile() returned an error!");

    // Create pdf
    let pdf = typst_pdf::pdf(&doc, Smart::Auto, None);
    fs::write(file_path, pdf).expect("Could not write pdf.");

    // Attempt to remove the temporary files
    path.close().expect("Could not remove temporary files.");

    Ok("Success".to_string())
}


fn generate_chart(dad_score:f64, mom_score:f64, child_score:f64, corrected_child_score:f64) -> Result<Vec<u8>, Error>
{
    let parental_average = (dad_score + mom_score) / 2.0;

    const INTERCEPT: f64 = 0.138891;
    const SLOPE: f64 = 0.483034;

    let average_y1: f64 = INTERCEPT + SLOPE * (-5.0 as f64);
    let average_y2: f64 = INTERCEPT + SLOPE * (5.0 as f64);

    let parent_avg_score = INTERCEPT + SLOPE * parental_average;
    let is_normal = child_score < parent_avg_score + (2.0 as f64)
        && child_score > (parent_avg_score - 2.0 as f64);
    let is_normal_corrected = corrected_child_score < parent_avg_score + (2.0 as f64)
        && corrected_child_score > parent_avg_score - (2.0 as f64);


}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![calculate_scores, make_pdf])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
impl From<Content> for Dict {
    fn from(value: Content) -> Self {
        value.into_dict()
    }
}

#[derive(Debug, Clone, IntoValue, IntoDict)]
struct Content {
    v: ContentData,
}

#[derive(Debug, Clone, IntoValue, IntoDict)]
struct ContentData {
    gender: String,
    age: u32,
    head_circumference: String,
    mother_head_circumference: String,
    father_head_circumference: String,
    premature_weeks: Option<u32>,
    premature_days: Option<u32>,
    corrected_age: String,
    child_score: String,
    corrected_child_score: String,
    father_score: String,
    mother_score: String,
    parental_average: String,
    version: String,
    dob: String,
}

// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error)
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}