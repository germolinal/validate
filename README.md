# A Rust Validation library

This crate was developed with the intent of helping you to validate scientific
tools; for exmple, for comparing the results of the temperature calculated
by an algorithm and those measured in an experiment. It is supposed to be embedded
the unit or integration testing.

Different Validation tasks can be created by implementing the [`Validate`] trait.  
These tasks will be stored into a [`Validations`] object which will write a report 
into a Markdown File.


# Example 1: Built-in Time Series validator

Validate has a built-in time series validator that plots two time series and calculates—if required—the 
Root Mean Squared Error (RSME) and Mean Bias Error (MBE) between them. Setting a maximum RMSE or MBE
is optional and it will make it return an error if these values are exceded

```rs
use validate::{Validations, SeriesValidator};

let expected = vec![1., 2., 3.];
let found = vec![5., 6., 6.];

let mut validator = Validations::new("report.md", ".");    
// Note that we are not defining a maximum allowed error
let v = validate::SeriesValidator {
    x_label: Some("time step"),
    y_label: Some("Zone Temperature"),
    y_units: Some("C"),
    expected,
    found,
    ..validate::SeriesValidator::default()
};
validator.push(Box::new(v));

// This will not fail because we did not set a maximum allowed 
// Root Mean Square Error or Mean Bias Error... if we did, it would return 
// an error and the unwrap woul
validator.validate().unwrap();
```
This produces a result as follows

## Title

 * Root Mean Squared Error: 3.70
 * Mean Bias Error: -3.67

<svg class="poloto" width="800" height="500" viewBox="0 0 800 500" xmlns="http://www.w3.org/2000/svg"><style>.poloto{stroke-linecap:round;stroke-linejoin:round;font-family:Roboto,sans-serif;font-size:16px;}.poloto_background{fill:AliceBlue;}.poloto_scatter{stroke-width:7}.poloto_tick_line{stroke:gray;stroke-width:0.5}.poloto_line{stroke-width:2}.poloto_text{fill: black;}.poloto_axis_lines{stroke: black;stroke-width:3;fill:none;stroke-dasharray:none}.poloto_title{font-size:24px;dominant-baseline:start;text-anchor:middle;}.poloto_xname{font-size:24px;dominant-baseline:start;text-anchor:middle;}.poloto_yname{font-size:24px;dominant-baseline:start;text-anchor:middle;}.poloto_legend_text{font-size:20px;dominant-baseline:middle;text-anchor:start;}.poloto0stroke{stroke:blue;}.poloto1stroke{stroke:red;}.poloto2stroke{stroke:green;}.poloto3stroke{stroke:gold;}.poloto4stroke{stroke:aqua;}.poloto5stroke{stroke:lime;}.poloto6stroke{stroke:orange;}.poloto7stroke{stroke:chocolate;}.poloto0fill{fill:blue;}.poloto1fill{fill:red;}.poloto2fill{fill:green;}.poloto3fill{fill:gold;}.poloto4fill{fill:aqua;}.poloto5fill{fill:lime;}.poloto6fill{fill:orange;}.poloto7fill{fill:chocolate;}</style><circle  r="1e5" class="poloto_background" /><text  class="poloto_text poloto_legend_text" x="675" y="100" ></text><path  class="poloto_line poloto0stroke" fill="none" stroke="black" d=" M 150.00 350.00 L 400.00 300.00 L 650.00 250.00" /><text  class="poloto_text poloto_legend_text" x="675" y="150" ></text><path  class="poloto_line poloto1stroke" fill="none" stroke="black" d=" M 150.00 150.00 L 400.00 100.00 L 650.00 100.00" /><text  class="poloto_labels poloto_text poloto_title" x="400" y="37.5" ></text><text  class="poloto_labels poloto_text poloto_xname" x="400" y="481.25" >time step</text><text  class="poloto_labels poloto_text poloto_yname" transform="rotate(-90,37.5,250)" x="37.5" y="250" >Zone Temperature (C)</text><text  class="poloto_tick_labels poloto_text" dominant-baseline="middle" text-anchor="start" x="150" y="70" ></text><line  class="poloto_axis_lines" stroke="black" x1="150" x2="144" y1="400" y2="400" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="middle" text-anchor="end" x="135" y="400" >0</text><line  class="poloto_axis_lines" stroke="black" x1="150" x2="144" y1="300" y2="300" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="middle" text-anchor="end" x="135" y="300" >2</text><line  class="poloto_axis_lines" stroke="black" x1="150" x2="144" y1="200" y2="200" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="middle" text-anchor="end" x="135" y="200" >4</text><line  class="poloto_axis_lines" stroke="black" x1="150" x2="144" y1="100" y2="100" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="middle" text-anchor="end" x="135" y="100" >6</text><text  class="poloto_tick_labels poloto_text" dominant-baseline="middle" text-anchor="start" x="440.00000000000006" y="70" ></text><line  class="poloto_axis_lines" stroke="black" x1="150" x2="150" y1="400" y2="405" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="start" text-anchor="middle" x="150" y="430" >0.0</text><line  class="poloto_axis_lines" stroke="black" x1="275" x2="275" y1="400" y2="405" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="start" text-anchor="middle" x="275" y="430" >0.5</text><line  class="poloto_axis_lines" stroke="black" x1="400" x2="400" y1="400" y2="405" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="start" text-anchor="middle" x="400" y="430" >1.0</text><line  class="poloto_axis_lines" stroke="black" x1="525" x2="525" y1="400" y2="405" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="start" text-anchor="middle" x="525" y="430" >1.5</text><line  class="poloto_axis_lines" stroke="black" x1="650" x2="650" y1="400" y2="405" /><text  class="poloto_tick_labels poloto_text" dominant-baseline="start" text-anchor="middle" x="650" y="430" >2.0</text><path  stroke="black" fill="none" class="poloto_axis_lines" style="stroke-dasharray:12.5;stroke-dashoffset:-0;" d=" M 150 400 L 650 400" /><path  stroke="black" fill="none" class="poloto_axis_lines" style="stroke-dasharray:12.5;stroke-dashoffset:-0;" d=" M 150 400 L 150 100" /></svg>




# Example 2: Write custom validator

```rs
use validate::{Validations, Validate};
use std::{fs::File, io::Write};

// This checks that two numbers are equal
struct CustomValidation {
    expected: u8,
    found: u8,
    title: &'static str,
}

impl Validate for CustomValidation {
    fn validate(&self, file: &mut File)->Result<(),String>{
        let valid = self.expected == self.found;
        
        
        // We return an error if the validation is not succesful, but we still
        // write something into the report. Even if this particular Validation 
        // fails, the Validations object will run all the validations and print
        // the error messages into the STDERR
        let (ret, buf) = if valid {
            let ret = Ok(());
            let buf = format!("# {}\n\n * Passed! {} and {} are equal", self.title, self.expected, self.found);
            (ret, buf)
        }else{
            let ret = Err(format!("{} and {} aren't equal", self.expected, self.found));
            let buf = format!("# {}\n\n * Failed... {} and {} aren't equal", self.title, self.expected, self.found );
            (ret, buf)
        };
        file.write_all(buf.as_bytes()).unwrap();
        ret
    }
}

let v = CustomValidation{
    expected: 2,
    found: 2,
    title: "Check that 2 and 3 are equal"
};
let mut validations = Validations::new("report.md", ".");
validations.push(Box::new(v));
validations.validate().unwrap()

```