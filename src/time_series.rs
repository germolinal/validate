/*
MIT License
Copyright (c) 2021 Germán Molina
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use crate::Validate;
use crate::ValidationResult;
use poloto::prelude::*;

/// Validates a time series based on Mean Bias Error and Root Mean Squared Error
#[derive(Default, Clone)]
pub struct SeriesValidator {
    /// The maximum allowed Mean Bias Error
    pub allowed_mean_bias_error: Option<f64>,

    /// The maximum allowed Root Mean Squared Error
    pub allowed_root_mean_squared_error: Option<f64>,

    /// The units in the y axis of the chart
    pub y_units: Option<&'static str>,

    /// The label in the y axis of the chart
    pub y_label: Option<&'static str>,

    /// The units in the x axis of the chart
    pub x_units: Option<&'static str>,

    /// The label in the x axis of the chart
    pub x_label: Option<&'static str>,

    /// The name of the series caled `expected`
    pub expected_legend: Option<&'static str>,

    /// The time series containing the expected values
    pub expected: Vec<f64>,

    /// The name of the `found` time series
    pub found_legend: Option<&'static str>,

    /// The time series containing the found values
    pub found: Vec<f64>,

    /// the title of the chart
    pub chart_title: Option<&'static str>,    
}

impl Validate for SeriesValidator {
    fn validate(&self) -> ValidationResult {
        let mut err_msg = String::new();

        if self.expected.len() != self.found.len() {
            err_msg = format!(
                "Series to compare of equal length. expected.len() = {}, found.len() = {}",
                self.expected.len(),
                self.found.len()
            );
            return ValidationResult::Err(err_msg.clone(), err_msg);
        }

        let n = self.expected.len() as f64;
        let num = self.expected.len();
        // Process Mean Bias Error
        let bias_error: f64 = self
            .expected
            .iter()
            .zip(self.found.iter())
            .map(|(x, y)| *y - *x)
            .sum();
        let mean_bias_error = bias_error / n;
        let mut mbe_msg = format!(" * Root Mean Squared Error: {:.2}", mean_bias_error);

        // Process Root Mean Squared Error
        let squared_error: f64 = self
            .expected
            .iter()
            .zip(self.found.iter())
            .map(|(x, y)| (*y - *x).powi(2))
            .sum();
        let root_mean_squared_error = (squared_error / n).sqrt();
        let mut rmse_msg = format!(" * Mean Bias Error: {:.2}", root_mean_squared_error);

        // Check compliance
        if let Some(allowed_mean_bias_error) = self.allowed_mean_bias_error {
            if mean_bias_error.abs() > allowed_mean_bias_error {
                err_msg = format!(
                    "{} * Mean Bias Error is {}, which is greater than the allowed value of {}",
                    err_msg,
                    mean_bias_error.abs(),
                    allowed_mean_bias_error
                );
                mbe_msg = format!("{} | **Failed!**", mbe_msg)
            }
        }
        if let Some(allowed_root_mean_squared_error) = self.allowed_root_mean_squared_error {
            if root_mean_squared_error > allowed_root_mean_squared_error {
                err_msg = format!(
                    "{}\n * Mean Root Squared Error is {}, which is greater than the allowed value of {}",
                    err_msg,  root_mean_squared_error, allowed_root_mean_squared_error
                );
                rmse_msg = format!("{} | **Failed!**", rmse_msg)
            }
        }

        let exp_legend = self.expected_legend.unwrap_or(&"Expected");
        let line_expected = poloto::range_iter([0.0, n], num).zip_output(|i| self.expected[i as usize]).buffered_plot().line(exp_legend);        
        let found_legend = self.found_legend.unwrap_or(&"Found");
        let line_found = poloto::range_iter([0.0, n], num).zip_output(|i| self.found[i as usize]).buffered_plot().line(found_legend);
        let origin = poloto::build::origin();
        // let data = plots!(line_expected, line_found, m);

        let mut x_label : String = self.x_label.unwrap_or(&"x").into();
        if let Some(units) = self.x_units {
            x_label = format!("{} ({})", x_label, units);
        }
        let mut y_label : String = self.y_label.unwrap_or(&"y").into();
        if let Some(units) = self.y_units {
            y_label = format!("{} ({})", y_label, units);
        }
        let chart_title = self.chart_title.unwrap_or(&"");
        let p = quick_fmt!(chart_title, &x_label, &y_label, line_expected, line_found, origin);

        let file = format!(
            "{}\n {}\n\n{}",            
            rmse_msg,
            mbe_msg,
            poloto::disp(|w| p.simple_theme(w))
        );

        if !err_msg.is_empty() {
            ValidationResult::Err(file, err_msg)
        } else {
            ValidationResult::Ok(file)
        }
    }
}
