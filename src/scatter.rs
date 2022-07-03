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
pub struct ScatterValidator {
    
    /// The units in the x and y axis of the chart (they are supposed to be the same)
    pub units: Option<&'static str>,
            
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


impl Validate for ScatterValidator {
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


       
        let n = self.expected.len();        
        let data = |i: usize|{
            [self.expected[i], self.found[i]]
        };
        let range = (0..n).map(|x| x as usize);
       
        let exp_legend = self.expected_legend.unwrap_or(&"Expected");
        let found_legend = self.found_legend.unwrap_or(&"Found");                
        let origin = poloto::build::origin();
        

        
        
        let chart_title = self.chart_title.unwrap_or(&"");
        let p = quick_fmt!(chart_title, &exp_legend, &found_legend, range.map(data).buffered_plot().scatter("some name"), origin);

        let file = format!(
            "\n\n{}",                        
            poloto::disp(|w| p.simple_theme(w))
        );

        if !err_msg.is_empty() {
            ValidationResult::Err(file, err_msg)
        } else {
            ValidationResult::Ok(file)
        }
    }
}
