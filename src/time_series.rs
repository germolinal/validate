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

use poloto::prelude::*;
use crate::Validate;
use std::{fs::File, io::Write};

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
    pub expected_name: &'static str,
    
    /// The time series containing the expected values
    pub expected: Vec<f64>,

    /// The name of the `found` time series
    pub found_name: &'static str,
    
    /// The time series containing the found values
    pub found: Vec<f64>,

    /// the title of the chart
    pub chart_title: &'static str,

    /// The title of the section in the Markdown
    pub title: &'static str,
    
}

impl Validate for SeriesValidator{
    
    fn validate(&self, file: &mut File)->Result<(),String>{
        if self.expected.len() != self.found.len(){
            return Err(format!("Series to compare of equal length. expected.len() = {}, found.len() = {}", self.expected.len(), self.found.len()));
        }
        let n = self.expected.len() as f64;
        let num = self.expected.len();
        let bias_error : f64 = self.expected.iter().zip(self.found.iter()).map(|(x,y)| *y - *x).sum();
        let mean_bias_error = bias_error/n;
        let squared_error : f64 = self.expected.iter().zip(self.found.iter()).map(|(x, y)| (*y - *x).powi(2)).sum();
        let root_mean_squared_error = (squared_error / n).sqrt();
        if let Some(allowed_mean_bias_error) = self.allowed_mean_bias_error{
            if mean_bias_error > allowed_mean_bias_error{
                return Err(format!("Mean Bias Error is {}, which is greater than the allowed value of {}", mean_bias_error, allowed_mean_bias_error))
            }
        }
        if let Some(allowed_root_mean_squared_error) = self.allowed_root_mean_squared_error{
            if root_mean_squared_error > allowed_root_mean_squared_error {
                return Err(format!("Mean Root Squared Error is {}, which is greater than the allowed value of {}", root_mean_squared_error, allowed_root_mean_squared_error))
            }
        }

        let line_expected = poloto::build::line(self.expected_name, poloto::range_iter([0.0, n], num).map(|i| [i, self.expected[i as usize] ]));
        let line_found =    poloto::build::line(self.found_name,    poloto::range_iter([0.0, n], num).map(|i| [i, self.found[i as usize] ]));
        let m = poloto::build::origin();
        let data = plots!(line_expected, line_found, m);

        let mut x_label = match self.x_label{
            Some(v)=>v.to_string(),
            None=>"x".to_string()
        };
        if let Some(units) = self.x_units{
            x_label = format!("{} ({})", x_label, units);
        }
        let mut y_label = match self.y_label{
            Some(v)=>v.to_string(),
            None=>"y".to_string()
        };
        if let Some(units) = self.y_units{
            y_label = format!("{} ({})", y_label, units);
        }
        let p = simple_fmt!(data, self.chart_title, x_label, y_label);

        let buf = format!("# {}\n\n * Root Mean Squared Error: {:.2}\n * Mean Bias Error: {:.2}\n\n{}", self.title, root_mean_squared_error, mean_bias_error, poloto::disp(|w| p.simple_theme(w)));
        file.write_all(buf.as_bytes()).unwrap();
        

        Ok(())

    }
}
