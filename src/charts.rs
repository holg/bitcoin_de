#![cfg(feature = "cmdline")]
//TODO: right now this is cmdline-only, so this module is only compiled when the "cmdline" feature is enabled.
//! Module for generating charts from exchange rate data.
use chrono::{Duration, NaiveDateTime, NaiveTime, Timelike};
use csv::ReaderBuilder;
use plotters::backend::SVGBackend;
use plotters::coord::ranged1d::{KeyPointHint, NoDefaultFormatting, Ranged, ValueFormatter};
use plotters::prelude::*;
use plotters::series::{LineSeries, PointSeries};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::ops::Range;
use tracing::{debug, error, info, warn};

/// Represents a time range with start and end datetimes.
///
/// This struct is used to define the range of time for which data is displayed
/// on a chart. It is also used to implement the `Ranged` trait for `NaiveDateTime`,
/// allowing it to be used as a coordinate system for plotting time series data.
#[derive(Clone, Debug)]
struct TimeRangedDateTime(NaiveDateTime, NaiveDateTime);

impl TimeRangedDateTime {
    // You can add methods here if needed, e.g., to get the start or end time.
}

/// Formats a `NaiveDateTime` value into a string representation for chart labels.
///
///
/// This implementation converts datetime values to a human-readable format
/// for display on chart axes and labels.
impl ValueFormatter<NaiveDateTime> for TimeRangedDateTime {
    /// Formats a datetime value into a string in "YYYY-MM-DD HH:MM" format.
    ///
    /// # Arguments
    ///
    /// * `value` - A reference to the `NaiveDateTime` value to format.
    ///
    /// # Returns
    ///
    /// A formatted string representation of the datetime.
    fn format(value: &NaiveDateTime) -> String {
        value.format("%Y-%m-%d %H:%M").to_string()
    }
}

/// Implements the `Ranged` trait for `TimeRangedDateTime`.
///
/// This implementation enables `TimeRangedDateTime` to be used as a coordinate
/// system for plotting time series data, handling the mapping between datetime
/// values and pixel coordinates on a chart.
impl Ranged for TimeRangedDateTime {
    type ValueType = NaiveDateTime;
    type FormatOption = NoDefaultFormatting;

    /// Maps a datetime value to a pixel coordinate within the specified limits.
    ///
    /// # Arguments
    ///
    /// * `value` - A reference to the `NaiveDateTime` value to map.
    /// * `limit` - A tuple of (min, max) pixel coordinates representing the available space.
    ///
    /// # Returns
    ///
    /// An integer representing the pixel coordinate for the given datetime value.
    fn map(&self, value: &Self::ValueType, limit: (i32, i32)) -> i32 {
        let start_ts = self.0.and_utc().timestamp();
        let end_ts = self.1.and_utc().timestamp();
        let range = end_ts - start_ts;

        if range <= 0 {
            return limit.0;
        }

        let value_ts = value.and_utc().timestamp();
        let clamped_value_ts = value_ts.clamp(start_ts, end_ts);
        let value_pos = (clamped_value_ts - start_ts) as f64 / range as f64;

        (limit.0 as f64 + value_pos * (limit.1 - limit.0) as f64).round() as i32
    }

    /// Generates key points (tick marks) for the time axis based on the time range.
    ///
    /// This function intelligently selects appropriate time intervals for tick marks
    /// based on the total duration of the time range, ensuring readable and evenly
    /// spaced labels on the chart.
    ///
    /// # Arguments
    ///
    /// * `hint` - A hint object that provides guidance on the maximum number of points to generate.
    ///
    /// # Returns
    ///
    /// A vector of `NaiveDateTime` values representing the key points for axis labels.
    fn key_points<Hint: KeyPointHint>(&self, hint: Hint) -> Vec<Self::ValueType> {
        let max_labels = hint.max_num_points().max(2);
        let start_time = self.0;
        let end_time = self.1;
        let duration = end_time - start_time;

        if duration <= Duration::zero() {
            return vec![start_time];
        }

        // Determine a "nice" interval based on the total duration
        let interval = if duration <= Duration::minutes(30) {
            Duration::minutes(5)
        } else if duration <= Duration::hours(2) {
            Duration::minutes(15)
        } else if duration <= Duration::hours(6) {
            Duration::minutes(30)
        } else if duration <= Duration::hours(12) {
            Duration::hours(1)
        } else if duration <= Duration::days(2) {
            Duration::hours(3)
        } else if duration <= Duration::days(7) {
            Duration::days(1)
        } else if duration <= Duration::days(30) {
            Duration::days(7)
        } else {
            Duration::days((duration.num_days() / (max_labels as i64)).max(1))
        };

        let mut key_points = Vec::new();
        let interval_seconds = interval.num_seconds();

        if interval_seconds <= 0 {
            return vec![start_time];
        }

        let start_ts = start_time.and_utc().timestamp();
        let first_marker_ts =
            (start_ts + interval_seconds - 1) / interval_seconds * interval_seconds;

        let mut current_ts = first_marker_ts;
        let end_ts = end_time.and_utc().timestamp();

        while current_ts <= end_ts {
            if let Some(dt) =
                chrono::DateTime::from_timestamp(current_ts, 0).map(|dt| dt.naive_utc())
            {
                if dt >= start_time && dt <= end_time {
                    key_points.push(dt);
                }
            } else {
                error!(
                    "Timestamp {} out of range for DateTime conversion",
                    current_ts
                );
            }

            if let Some(next_interval_ts) = current_ts.checked_add(interval_seconds) {
                if next_interval_ts <= current_ts {
                    warn!("Interval addition did not advance timestamp, breaking.");
                    break;
                }
                current_ts = next_interval_ts;
            } else {
                error!("Timestamp addition overflow detected");
                break;
            }

            if key_points.len() > max_labels * 3 && max_labels > 0 {
                warn!(
                    "Generated significantly more key points than hint suggested, breaking early."
                );
                break;
            }
        }

        if key_points.is_empty() {
            key_points.push(start_time);
            if end_time > start_time {
                key_points.push(end_time);
            }
        }

        if key_points.len() > max_labels {
            let step = (key_points.len() as f64 / max_labels as f64).ceil() as usize;
            if step > 1 {
                key_points = key_points.into_iter().step_by(step).collect();
            } else {
                key_points.truncate(max_labels);
            }
        }

        debug!(
            "Generated {} key points for range {:?} to {:?}",
            key_points.len(),
            start_time,
            end_time
        );
        key_points
    }

    /// Returns the range of datetime values covered by this coordinate system.
    ///
    /// # Returns
    ///
    /// A `Range` from the start datetime to the end datetime.
    fn range(&self) -> Range<Self::ValueType> {
        self.0..self.1
    }
}

/// Creates a chart displaying exchange rate data over time.
///
/// This function generates an SVG chart that visualizes exchange rate data
/// over a specified time period. It plots the current exchange rate, as well
/// as 3-hour and 12-hour weighted average rates, against time. The chart
/// includes labels, a title, and markers for the start and end points of the
/// data.
///
/// # Arguments
///
/// * `pair` - The trading pair (e.g., "BTC/EUR") for which to generate the chart.
/// * `data` - A slice of tuples, where each tuple contains a `NaiveDateTime`
///            timestamp, the current exchange rate, the 3-hour weighted average
///            rate, and the 12-hour weighted average rate.
/// * `output_file` - The path to the SVG file where the chart will be saved.
///
/// # Returns
///
pub fn create_chart(
    pair: &str,
    data: &[(NaiveDateTime, f64, f64, f64)],
    output_file: &str,
) -> Result<(), Box<dyn Error>> {
    if data.is_empty() {
        warn!(pair, "No data provided. Skipping chart generation.");
        return Ok(());
    }

    let mut sorted_data = data.to_vec();
    sorted_data.sort_by_key(|k| k.0);

    let valid_ys: Vec<f64> = sorted_data
        .iter()
        .flat_map(|(_, current, rate_3h, rate_12h)| vec![*current, *rate_3h, *rate_12h])
        .filter(|&v| v.is_finite() && v > 0.0)
        .collect();

    let min_y = valid_ys.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_y = valid_ys.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    let (mut y_min, mut y_max) = if min_y.is_infinite() || max_y.is_infinite() {
        warn!(
            pair,
            "No valid Y-axis data found. Using default range [0,1]."
        );
        (0.0, 1.0)
    } else if (max_y - min_y).abs() < f64::EPSILON {
        let center = (min_y + max_y) / 2.0;
        let half_span = 0.5;
        (center - half_span, center + half_span)
    } else {
        let y_range = max_y - min_y;
        let padding = (y_range * 0.05).max(0.01);
        (min_y - padding, max_y + padding)
    };
    y_min = y_min.max(0.0);
    if y_max - y_min < 0.1 {
        y_max = y_min + 0.1;
    }

    let first_time = sorted_data.first().map_or(NaiveDateTime::MIN, |d| d.0);
    let last_time = sorted_data.last().map_or(NaiveDateTime::MAX, |d| d.0);

    let final_last_time = if first_time >= last_time {
        warn!(
            pair,
            "Chart start time >= end time. Adjusting range slightly."
        );
        first_time
            .checked_add_signed(Duration::seconds(1))
            .unwrap_or(last_time)
    } else {
        last_time
    };
    // Store the range used for axis generation
    let time_range_spec = TimeRangedDateTime(first_time, final_last_time);
    debug!(
        pair,
        ?first_time,
        ?last_time,
        ?final_last_time,
        "Calculated chart time range"
    );

    let root = SVGBackend::new(output_file, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let title = format!(
        "{} Exchange Rate\n{} to {}",
        pair.to_uppercase(),
        first_time.format("%Y-%m-%d %H:%M"),
        last_time.format("%Y-%m-%d %H:%M")
    );

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 20))
        .margin(20)
        .x_label_area_size(90)
        .y_label_area_size(80)
        .build_cartesian_2d(time_range_spec.clone(), y_min..y_max)?; // Clone spec for axis

    let time_range_duration = final_last_time - first_time;

    let time_format = if time_range_duration > Duration::days(7) {
        "%Y-%m-%d"
    } else if time_range_duration > Duration::days(1) {
        "%m-%d %Hh"
    } else if time_range_duration > Duration::hours(1) {
        "%H:%M"
    } else {
        "%H:%M:%S"
    };

    debug!(pair, %time_format, "Configuring chart mesh");
    chart
        .configure_mesh()
        .x_label_formatter(&|dt: &NaiveDateTime| dt.format(time_format).to_string())
        .y_labels(10)
        .x_label_style(
            ("sans-serif", 12)
                .into_font()
                .color(&BLACK)
                .transform(FontTransform::Rotate90),
        )
        .y_label_formatter(&|y| {
            if *y >= 1_000_000.0 {
                format!("{:.1}M", y / 1_000_000.0)
            } else if *y >= 10_000.0 {
                format!("{:.0}K", y / 1000.0)
            } else if *y >= 1000.0 {
                format!("{:.1}K", y / 1000.0)
            } else if *y >= 1.0 {
                format!("{:.2}", y)
            } else {
                format!("{:.6}", y)
            }
        })
        .y_label_style(("sans-serif", 12).into_font().color(&BLACK))
        .axis_desc_style(("sans-serif", 15))
        .bold_line_style(BLACK.mix(0.1))
        .light_line_style(TRANSPARENT)
        .x_desc("Time")
        .y_desc("Rate")
        .draw()?;

    let valid_current_data: Vec<(NaiveDateTime, f64)> = sorted_data
        .iter()
        .map(|(time, current, _, _)| (*time, *current))
        .filter(|(_, y)| y.is_finite())
        .collect();
    let valid_3h_data: Vec<(NaiveDateTime, f64)> = sorted_data
        .iter()
        .map(|(time, _, rate_3h, _)| (*time, *rate_3h))
        .filter(|(_, y)| y.is_finite())
        .collect();
    let valid_12h_data: Vec<(NaiveDateTime, f64)> = sorted_data
        .iter()
        .map(|(time, _, _, rate_12h)| (*time, *rate_12h))
        .filter(|(_, y)| y.is_finite())
        .collect();

    // Ensure we have valid data before trying to draw lines or markers
    if valid_current_data.is_empty() {
        warn!(pair, "No valid (finite) data points found to plot.");
        // --- Use DrawingArea::draw_text with styled TextStyle ---
        // --- Use DrawingArea::draw_text with styled TextStyle ---
        // --- Use DrawingArea::draw_text with styled TextStyle ---
        // --- Use DrawingArea::draw_text directly with font and color ---
        root.draw_text(
            "No valid data to display",
            &("sans-serif", 20).into_font().color(&RED.mix(1.0)),
            (512, 384),
        )?;
        // --- End Fix ---
        info!(pair, output_file, "Generated empty chart (no valid data)");
        return Ok(());
    }

    chart
        .draw_series(LineSeries::new(valid_current_data.clone(), RED))?
        .label("Current Rate")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .draw_series(LineSeries::new(valid_3h_data.clone(), BLUE))?
        .label("3h Weighted Avg")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    chart
        .draw_series(LineSeries::new(valid_12h_data.clone(), GREEN))?
        .label("12h Weighted Avg")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], GREEN));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .position(SeriesLabelPosition::UpperRight)
        .draw()?;

    // --- Add markers and text for Start and End points ---
    if let (Some(start_data), Some(end_data)) =
        (valid_current_data.first(), valid_current_data.last())
    {
        let start_time_marker = time_range_spec.0;
        let end_time_marker = time_range_spec.1;
        let start_point_value = start_data.1;
        let end_point_value = end_data.1;
        let marker_shape_style = ShapeStyle::from(&BLACK).filled();
        // Draw Start Point Marker & Text
        chart.draw_series(PointSeries::of_element(
            vec![(start_time_marker, start_point_value)],
            5,
            marker_shape_style.clone(),
            &|coord, size, style| {
                let text = Text::new(
                    format!("Start: {}", start_time_marker.format("%Y-%m-%d %H:%M")),
                    (10, -15),
                    ("sans-serif", 12).into_font().color(&BLACK.mix(0.7)),
                );

                EmptyElement::at(coord) + Circle::new((0, 0), size, style) + text
            },
        ))?;

        // Draw End Point Marker & Text
        chart.draw_series(PointSeries::of_element(
            vec![(end_time_marker, end_point_value)],
            5,
            marker_shape_style,
            &|coord, size, style| {
                // Use a color directly without creating a local variable
                let text_element = Text::new(
                    format!("End: {}", end_time_marker.format("%Y-%m-%d %H:%M")),
                    (-10, -15),
                    ("sans-serif", 12).into_font().color(&BLACK.mix(0.7)),
                );

                EmptyElement::at(coord) + Circle::new((0, 0), size, style) + text_element
            },
        ))?;
    }
    // --- End Start/End markers ---

    info!(pair, output_file, "Successfully generated chart");
    Ok(())
}

/// Generates exchange rate charts from CSV data, including a portfolio summary chart.
///
/// This function reads cryptocurrency exchange rate data from a CSV file, processes it,
/// and generates SVG charts for each trading pair as well as a portfolio summary chart.
/// The CSV file is expected to have at least 9 columns with specific data in each column:
/// - Column 0: Timestamp in format "%Y-%m-%d %H:%M:%S"
/// - Column 1: Trading pair (e.g., "BTC/EUR")
/// - Column 2: Current exchange rate
/// - Column 3: 3-hour weighted average rate
/// - Column 4: 12-hour weighted average rate
/// - Column 6: Current weighted value
/// - Column 7: 3-hour weighted value
/// - Column 8: 12-hour weighted value
///
/// # Parameters
///
/// * `csv_file` - Path to the CSV file containing exchange rate data
/// * `output_dir` - Directory where the generated SVG charts will be saved
/// * `time_range` - Optional time range filter (start_time, end_time). If provided,
///                  only data points within this range will be included in the charts
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - Ok(()) if charts were generated successfully,
///                                  or an error if any part of the process failed
///
/// # Examples
///
/// ```no_run
/// use chrono::NaiveDateTime;
/// let start_time = NaiveDateTime::parse_from_str("2023-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
/// let end_time = NaiveDateTime::parse_from_str("2023-01-31 23:59:59", "%Y-%m-%d %H:%M:%S").unwrap();
/// generate_charts_from_csv("data.csv", "charts", Some((start_time, end_time)));
/// ```
pub fn generate_charts_from_csv(
    csv_file: &str,
    output_dir: &str,
    time_range: Option<(NaiveDateTime, NaiveDateTime)>,
) -> Result<(), Box<dyn Error>> {
    info!(
        csv_file,
        output_dir,
        ?time_range,
        "Generating charts from CSV"
    );

    std::fs::create_dir_all(output_dir)?;

    let mut pair_data: HashMap<String, Vec<(NaiveDateTime, f64, f64, f64)>> = HashMap::new();
    let mut portfolio_data: HashMap<NaiveDateTime, (f64, f64, f64)> = HashMap::new();

    let file = File::open(csv_file)?;
    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_reader(file);

    for (line_num, result) in rdr.records().enumerate() {
        let record = match result {
            Ok(rec) => rec,
            Err(e) => {
                error!(line = line_num + 2, error = %e, "Error reading CSV");
                continue;
            }
        };
        if record.len() < 9 {
            warn!(
                line = line_num + 2,
                cols = record.len(),
                "Skipping CSV line: insufficient columns"
            );
            continue;
        }

        let timestamp_str = &record[0];
        let timestamp = match NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S") {
            Ok(ts) => ts,
            Err(e) => {
                error!(line = line_num + 2, timestamp = timestamp_str, error = %e, "Error parsing timestamp");
                continue;
            }
        };

        if let Some((start_time, end_time)) = time_range {
            if timestamp < start_time || timestamp > end_time {
                continue;
            }
        }

        let pair = record[1].to_string();
        let parse_f64 = |col_idx: usize, field_name: &str| -> f64 {
            match record[col_idx].parse::<f64>() {
                Ok(val) if val.is_finite() => val,
                Ok(val) => {
                    warn!(line = line_num + 2, field = field_name, value = %val, "Non-finite value found");
                    0.0
                }
                Err(_) => {
                    error!(line = line_num + 2, field = field_name, value = %record[col_idx], "Error parsing float");
                    0.0
                }
            }
        };

        let current_rate = parse_f64(2, "current_rate");
        let rate_3h = parse_f64(3, "rate_3h");
        let rate_12h = parse_f64(4, "rate_12h");
        let amount = parse_f64(5, "amount");
        // Fix these  csv-values are bollocks, it is simple math and shall be removed in the future
        // let value_current = parse_f64(6, "value_weighted");
        // let value_3h = parse_f64(7, "value_weighted_3h");
        // let value_12h = parse_f64(8, "value_weighted_12h");

        // Calculate the values ourselves instead of using the pre-calculated values
        // This ensures consistency and avoids any errors in the CSV data
        let value_current = current_rate * amount;
        let value_3h = rate_3h * amount;
        let value_12h = rate_12h * amount;

        // Add data point for individual pair chart
        pair_data.entry(pair.clone()).or_default().push((
            timestamp,
            current_rate,
            rate_3h,
            rate_12h,
        ));

        let minute_timestamp = NaiveDateTime::new(
            timestamp.date(),
            NaiveTime::from_hms_opt(timestamp.hour(), timestamp.minute(), 0)
                .unwrap_or(NaiveTime::MIN),
        );
        let entry = portfolio_data.entry(minute_timestamp).or_default();
        entry.0 += value_current;
        entry.1 += value_3h;
        entry.2 += value_12h;
    }

    for (pair, data) in pair_data {
        if data.is_empty()
            || data
                .iter()
                .all(|(_, r, r3, r12)| *r == 0.0 && *r3 == 0.0 && *r12 == 0.0)
        {
            info!(pair, "Skipping chart generation: no non-zero rate data.");
            continue;
        }
        let output_file = format!(
            "{}/{}_chart.svg",
            output_dir,
            pair.replace('/', "_").to_lowercase()
        );
        if let Err(e) = create_chart(&pair, &data, &output_file) {
            error!(pair, error = %e, "Failed to generate chart");
        }
    }

    if !portfolio_data.is_empty() {
        let mut summary_data: Vec<(NaiveDateTime, f64, f64, f64)> = portfolio_data
            .into_iter()
            .map(|(timestamp, (current, h3, h12))| (timestamp, current, h3, h12))
            .collect();
        summary_data.sort_by_key(|k| k.0);

        if summary_data.is_empty()
            || summary_data
                .iter()
                .all(|(_, c, h3, h12)| *c == 0.0 && *h3 == 0.0 && *h12 == 0.0)
        {
            info!("Skipping portfolio summary chart: no non-zero value data.");
        } else {
            let summary_file = format!("{}/portfolio_summary.svg", output_dir);
            if let Err(e) = create_chart("Portfolio Total (EUR)", &summary_data, &summary_file) {
                error!(error = %e, "Failed to generate portfolio summary chart");
            } else {
                info!(summary_file, "Generated portfolio summary chart");
            }
        }
    } else {
        info!("No portfolio data found to generate summary chart.");
    }

    Ok(())
}
