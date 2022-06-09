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
//! use validate::{Validations, Validate};
//! use std::{fs::File, io::Write};
//! 
//! // This checks that two numbers are equal
//! struct CustomValidation {
//!     expected: u8,
//!     found: u8,
//!     title: &'static str,
//! }
//! 
//! impl Validate for CustomValidation {
//!     fn validate(&self, file: &mut File)->Result<(),String>{
//!         let valid = self.expected == self.found;
//!         
//!         
//!         // We return an error if the validation is not succesful, but we still
//!         // write something into the report. Even if this particular Validation 
//!         // fails, the Validations object will run all the validations and print
//!         // the error messages into the STDERR
//!         let (ret, buf) = if valid {
//!             let ret = Ok(());
//!             let buf = format!("# {}\n\n * Passed! {} and {} are equal", self.title, self.expected, self.found);
//!             (ret, buf)
//!         }else{
//!             let ret = Err(format!("{} and {} aren't equal", self.expected, self.found));
//!             let buf = format!("# {}\n\n * Failed... {} and {} aren't equal", self.title, self.expected, self.found );
//!             (ret, buf)
//!         };
//!         file.write_all(buf.as_bytes()).unwrap();
//!         ret
//!     }
//! }
//! 
//! let v = CustomValidation{
//!     expected: 2,
//!     found: 2,
//!     title: "Check that 2 and 3 are equal"
//! };
//! let mut validations = Validations::new("report.md", ".");
//! validations.push(Box::new(v));
//! validations.validate().unwrap()
//! 
//! ```


use std::{fs::File, io::Write};


/// A Validator that plots two time series and calculates—if required—the 
/// Root Mean Squared Error and Mean Bias Error between them.
/// 
/// # Example
/// 
/// ```
/// use validate::{Validations, SeriesValidator};
/// 
/// let expected = vec![1., 2., 3.];
/// let found = vec![5., 6., 6.];
/// 
/// let mut validator = Validations::new("report.md", ".");    
/// // Note that we are not defining a maximum allowed error
/// let v = validate::SeriesValidator {
///     x_label: Some("time step"),
///     y_label: Some("Zone Temperature"),
///     y_units: Some("C"),
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
                    panic!("{} and {} are not close enough (allowed difference was {}... found {})", left_val, right_val, allowed_diff, diff);
                }
            }
        }
    };
    ($left:expr, $right:expr ) => {
        match ($left, $right) {
            (left_val, right_val) => {
                let allowed_diff : f64 = 1e-6;
                let diff = (left_val as f64 - right_val as f64).abs();
                if (diff > allowed_diff) {                                        
                    panic!("{} and {} are not close enough (allowed difference was {}... found {})", left_val, right_val, allowed_diff, diff);
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
                    panic!("{} and {} are too close (minimum difference was {}... found {})", left_val, right_val, allowed_diff, diff);
                }
            }
        }
    };
    ($left:expr, $right:expr ) => {
        match ($left, $right) {
            (left_val, right_val) => {
                let allowed_diff : f64 = 1e-6;
                let diff = (left_val as f64 - right_val as f64).abs();                
                if (diff < allowed_diff) {                                        
                    panic!("{} and {} are too close (minimum difference was {}... found {})", left_val, right_val, allowed_diff, diff);
                }
            }
        }
    };
}



/// This structure holds the validations to be ran, runs them, 
/// and writes the results into a Markdown report.
pub struct Validations {
    
    /// The validations to run
    validations: Vec<Box<dyn Validate>>,

    /// The file in which the report will be written
    target_file: &'static str, 

    /// The location of the data that will be used by the report (e.g., images)
    report_data_dir: &'static str,
}

impl Validations {


    /// Creates a new `Validator` that will write a report on `target_file` and put the 
    /// supporting data on `report_data_dir`
    pub fn new(target_file: &'static str, report_data_dir: &'static str) -> Self {        
        Self{
            validations: Vec::new(),
            target_file,
            report_data_dir,
        }
    }

    /// Adds a new validation to the `Validator`
    pub fn push(&mut self, v: Box<dyn Validate>){
        self.validations.push(v)
    }

    /// Given a filename thaat should be stored into as support data, this
    /// function returns path to that file inside of the directory containing 
    /// supporting data
    pub fn get_support_path(&self, filename: &'static str)->String{
        return format!("{}/{}", self.report_data_dir, filename)
    }

    /// Runs the validations, writes the report and fails the task if necessary
    pub fn validate(&self)->Result<(),String>{
        let mut md = File::create(self.target_file).unwrap();
        let mut errors = Vec::new();
        for v in self.validations.iter(){
            md.write_all(b"\n\n").unwrap();
            if let Err(msg) = v.validate(&mut md){
                errors.push(msg);
            }            
        }
        if errors.is_empty(){
            return Ok(())
        }else{
            for e in errors{
                eprintln!("{}",e);                
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
    fn validate(&self, file: &mut File)->Result<(),String>;
}



/// Reads a number of columns from a csv
pub fn from_csv(path: &'static str, cols: &[usize])->Vec<Vec<f64>>{
    let reader = File::open(path).unwrap();
    let mut rdr = csv::Reader::from_reader(reader);        
    
    let mut ret : Vec<Vec<f64>> = Vec::with_capacity(cols.len());
    for _ in cols{
        ret.push(Vec::new());
    }

    for record in rdr.records(){
        let data = record.unwrap();            
        
        for (i,col) in cols.iter().enumerate(){
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
    fn test_assert_not_close_fail(){
        assert_not_close!(1., 1., 0.1);
    }
   
}
