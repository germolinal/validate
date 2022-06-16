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

#![deny(missing_docs)]

//! This crate was developed with the intent of helping you to validate scientific
//! tools; for exmple, for comparing the results of the temperature calculated
//! by an algorithm and those measured in an experiment. It is supposed to be embedded
//! the unit or integration testing.
//!
//! Different Validation tasks can be created by implementing the [`Validate`] trait.  
//! These tasks will be stored into a [`Validations`] object which will write a report
//! into a Markdown File.
//!
//! # Example
//!
//! ```
//! use validate::{Validator, Validate, ValidationResult};
//! use std::{fs::File, io::Write};
//!
//! // This checks that two numbers are equal
//! struct CustomValidator {
//!     expected: u8,
//!     found: u8,
//!     title: &'static str,
//! }
//!
//! impl Validate for CustomValidator {
//!     fn validate(&self)->ValidationResult{
//!         let valid = self.expected == self.found;
//!         let mut ret = String::new();
//!         let mut err = String::new();
//!         
//!         
//!         // We return an error if the validation is not succesful, but we still
//!         // write something into the report. Even if this particular Validation
//!         // fails, the Validations object will run all the validations and print
//!         // the error messages into the STDERR
//!         if valid {
//!             let ret = format!("## {}\n\n * Passed! {} and {} are equal", self.title, self.expected, self.found);
//!         }else{
//!             err = format!("{}\n{} and {} aren't equal", err, self.expected, self.found);
//!             ret = format!("{}\n\n# {}\n\n * Failed... {} and {} aren't equal",ret, self.title, self.expected, self.found );
//!         };
//!         if err.len()>0{
//!             ValidationResult::Err(err, ret)
//!         }else{
//!             ValidationResult::Ok(ret)
//!         }
//!     }
//! }
//!
//! let v = CustomValidator{
//!     expected: 2,
//!     found: 2,
//!     title: "Check that 2 and 3 are equal"
//! };
//! let mut validator = Validator::new("Test Validation", "report.md");
//! validator.push(Box::new(v));
//! validator.validate().unwrap()
//!
//! ```

use std::{fs::File, io::Write};

/// A Validator that plots two time series and calculates—if required—the
/// Root Mean Squared Error and Mean Bias Error between them.
///
/// # Example
///
/// ```
/// use validate::{Validator, SeriesValidator};
///
/// let expected = vec![1., 2., 3.];
/// let found = vec![5., 6., 6.];
///
/// let mut validator = Validator::new("Validate Time series", "report.md");    
/// // Note that we are not defining a maximum allowed error
/// let v = validate::SeriesValidator {
///     x_label: Some("time step"),
///     y_label: Some("Zone Temperature"),
///     y_units: Some("C"),
///     title: "Compare Series!",
///     expected,
///     found,
///     ..validate::SeriesValidator::default()
/// };
/// validator.push(Box::new(v));
///
/// // This will not fail because we did not set a maximum allowed
/// // Root Mean Square Error or Mean Bias Error... if we did, it would return
/// // an error and the unwrap woul
/// validator.validate().unwrap();
/// ```
mod time_series;
pub use time_series::SeriesValidator;

/// Asserts whether two numbers are close enough
/// by comparing the first argument with the second, and
/// the threshold being the third.
///
/// # Examples
/// ```
/// use validate::assert_close;
/// assert_close!(1., 1.01, 0.1);
/// assert_close!(1., 1.0000001); // This assumes a threshold of 1e-6
/// ```
#[macro_export]
macro_rules! assert_close {
    ($left:expr, $right:expr, $allowed_diff: expr ) => {
        match ($left, $right, $allowed_diff) {
            (left_val, right_val, allowed_diff) => {
                let diff = (left_val as f64 - right_val as f64).abs();
                if (diff > allowed_diff) {
                    panic!(
                        "{} and {} are not close enough (allowed difference was {}... found {})",
                        left_val, right_val, allowed_diff, diff
                    );
                }
            }
        }
    };
    ($left:expr, $right:expr ) => {
        match ($left, $right) {
            (left_val, right_val) => {
                let allowed_diff: f64 = 1e-6;
                let diff = (left_val as f64 - right_val as f64).abs();
                if (diff > allowed_diff) {
                    panic!(
                        "{} and {} are not close enough (allowed difference was {}... found {})",
                        left_val, right_val, allowed_diff, diff
                    );
                }
            }
        }
    };
}

/// Asserts whether two numbers are close enough
///
/// # Examples
/// ```
/// use validate::assert_not_close;
/// assert_not_close!(1., 10., 0.2);
/// assert_not_close!(1., 10.); // This assumes a threshold of 1e-6
/// ```
#[macro_export]
macro_rules! assert_not_close {
    ($left:expr, $right:expr, $allowed_diff: expr ) => {
        match ($left, $right, $allowed_diff) {
            (left_val, right_val, allowed_diff) => {
                let diff = (left_val as f64 - right_val as f64).abs();
                if (diff < allowed_diff) {
                    panic!(
                        "{} and {} are too close (minimum difference was {}... found {})",
                        left_val, right_val, allowed_diff, diff
                    );
                }
            }
        }
    };
    ($left:expr, $right:expr ) => {
        match ($left, $right) {
            (left_val, right_val) => {
                let allowed_diff: f64 = 1e-6;
                let diff = (left_val as f64 - right_val as f64).abs();
                if (diff < allowed_diff) {
                    panic!(
                        "{} and {} are too close (minimum difference was {}... found {})",
                        left_val, right_val, allowed_diff, diff
                    );
                }
            }
        }
    };
}

/// Implements a validation error, where 
/// `Ok` returns just the text to write in the report,
/// but `Err` returns not only that but also an error message
pub enum ValidationResult {
    /// Returns an error; meaning that returns that it returns 
    /// something to write in the report and also an error message 
    /// (in that order)
    Err(String,String),

    /// Returns a message to write on the report
    Ok(String)
}


/// This structure holds a number of validations to be ran, runs them,
/// and writes the results into an HTML report. It has a title, which is used
/// as a Header in its report.
pub struct Validator<'a> {
    /// The title of this section
    title: &'a str,

    /// The validations to run
    validations: Vec<Box<dyn Validate>>,

    /// The file in which the report will be written
    target_file: &'a str,
    
}

impl <'a>Validator<'a> {
    /// Creates a new `Validator` that will write a report on `target_file` and put the
    /// supporting data on `report_data_dir`
    pub fn new(title: &'a str, target_file: &'a str) -> Self {
        Self {
            title,
            target_file,
            validations: Vec::new(),
        }
    }

    /// Adds a new validation to the `Validator`
    pub fn push(&mut self, v: Box<dyn Validate>) {
        self.validations.push(v)
    }

    
    /// Runs the validations, writes the report and fails the task if necessary
    pub fn validate(&self) -> Result<(), String> {
        let mut md = File::create(self.target_file).unwrap();
        let mut errors = Vec::new();
        
        
        // Write title
        let title = format!("# {}\n\n", self.title);        
        md.write(title.as_bytes() ).unwrap();
        
        // Solve
        let txt : Vec<String> =  self.validations.iter().map(|v| {
            md.write_all(b"\n\n").unwrap();
            match v.validate(){
                ValidationResult::Err(txt, e)=>{
                    errors.push(e);
                    md.write_all(txt.as_bytes()).unwrap();
                    txt
                },
                ValidationResult::Ok(txt)=>{
                    txt
                }
            }            
        }).collect();         

        // Write
        txt.iter().for_each(|t| {
            md.write_all(t.as_bytes()).unwrap();
        });

        // Return        
        if errors.is_empty() {
            return Ok(());
        } else {
            for e in errors {
                eprintln!("{}", e);
            }
            Err("Some validations failed...".to_string())
        }
    }
}

/// The main trait of this crate. All validator modules need
/// to comply with this trait.
pub trait Validate {

    /// Runs a validation procedure, returning an error message if
    /// the validation failed.
    ///
    /// Whether it fails or not,
    /// it should write the results of the validations into `file`
    /// so a full report is written.
    fn validate(&self) -> ValidationResult;
}

/// Reads a number of columns from a csv
pub fn from_csv(path: &str, cols: &[usize]) -> Vec<Vec<f64>> {
    let reader = File::open(path).unwrap();
    let mut rdr = csv::Reader::from_reader(reader);

    let mut ret: Vec<Vec<f64>> = Vec::with_capacity(cols.len());
    for _ in cols {
        ret.push(Vec::new());
    }

    for record in rdr.records() {
        let data = record.unwrap();

        for (i, col) in cols.iter().enumerate() {
            let v = &data[*col].trim();
            let v = v.parse::<f64>().unwrap();
            ret[i].push(v);
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_assert_close_correct() {
        assert_close!(1., 2., 2.);
        assert_not_close!(1., 21., 1.);
    }

    #[test]
    #[should_panic]
    fn test_assert_close_fail() {
        assert_close!(1., 2., 0.2);
    }

    #[test]
    #[should_panic]
    fn test_assert_not_close_fail() {
        assert_not_close!(1., 1., 0.1);
    }

    // #[test]
    // fn test_html(){
    //     use std::io::{Write};
    //     use std::fs;
    //     use pulldown_cmark::{Parser, Options, html};

    //     let contents = fs::read_to_string("./report.md").expect("Something went wrong reading the file");;        
        
    //     // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    //     // and we therefore must enable it explicitly.
    //     let mut options = Options::empty();
    //     options.insert(Options::ENABLE_STRIKETHROUGH);
    //     let parser = Parser::new_ext(&contents, options);

    //     // Write to String buffer.
    //     let mut html_output = String::new();
    //     html::push_html(&mut html_output, parser);

    //     let mut output = fs::File::create("./report.html").unwrap();
    //     output.write(html_output.as_bytes()).unwrap();
        
        
    // }
}
