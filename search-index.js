var searchIndex = JSON.parse('{\
"validate":{"doc":"This crate was developed with the intent of helping you to …","t":"NNDDIEDDMMOOLLLLMMMMMMMMMMMLLFLLLALAAMAMMLLLLLLMLMXKLMAMMMMIIIKKKDLLMLLLMMMMLLLLLLMLFFFFFFDMMLLMLLLMMMMLLLLLLLMMMMGDLLMLLLMLLLML","n":["Err","Ok","ScatterValidator","SeriesValidator","Validate","ValidationResult","Validator","ValidatorWrapper","allowed_mean_bias_error","allowed_root_mean_squared_error","assert_close","assert_not_close","borrow","borrow","borrow_mut","borrow_mut","chart_title","chart_title","description","expected","expected","expected_legend","expected_legend","found","found","found_legend","found_legend","from","from","from_csv","into","into","new","numberish","push","scatter","stats","target_file","time_series","title","title","try_from","try_from","try_into","try_into","type_id","type_id","units","unwrap","val","valid","validate","validate","validations","validator_wrapper","x_label","x_units","y_label","y_units","Nanish","Numberish","OneZero","is_it_nan","one","zero","ScatterValidator","borrow","borrow_mut","chart_title","clone","clone_into","default","expected","expected_legend","found","found_legend","from","into","to_owned","try_from","try_into","type_id","units","validate","linear_coefficients","mean","mean_bias_error","min_max","root_mean_squared_error","try_into_t","SeriesValidator","allowed_mean_bias_error","allowed_root_mean_squared_error","borrow","borrow_mut","chart_title","clone","clone_into","default","expected","expected_legend","found","found_legend","from","into","to_owned","try_from","try_into","type_id","validate","x_label","x_units","y_label","y_units","ValidationFn","ValidatorWrapper","borrow","borrow_mut","description","format_description","from","into","title","try_from","try_into","type_id","val","validate"],"q":["validate","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","validate::numberish","","","","","","validate::scatter","","","","","","","","","","","","","","","","","","","validate::stats","","","","","","validate::time_series","","","","","","","","","","","","","","","","","","","","","","","","validate::validator_wrapper","","","","","","","","","","","","",""],"d":["Returns an error; meaning that returns that it returns …","Returns a message to write on the report","Validates a time series based on Mean Bias Error and Root …","Validates a time series based on Mean Bias Error and Root …","The main trait of this crate. All validator modules need …","Implements a validation error, where <code>Ok</code> returns just the …","This structure holds a number of validations to be ran, …","A wrapper that contains an object that implements <code>Validate</code>","The maximum allowed Mean Bias Error","The maximum allowed Root Mean Squared Error","Asserts whether two numbers are close enough by comparing …","Asserts whether two numbers are close enough","","","","","the title of the chart","the title of the chart","A description","The time series containing the expected values","The time series containing the expected values","The name of the series caled <code>expected</code>","The name of the series caled <code>expected</code>","The time series containing the found values","The time series containing the found values","The name of the <code>found</code> time series","The name of the <code>found</code> time series","Returns the argument unchanged.","Returns the argument unchanged.","Reads a number of columns from a CSV, transforms them into …","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Creates a new <code>Validator</code> that will write a report on …","A trait defining some numerical-ish trait.","Adds a new validation to the <code>Validator</code>","A Validator that creates a scatter plot from two datasets, …","Module with some useful functions for calculating …","The file in which the report will be written","A Validator that plots two time series and calculates—if …","The title of the test","The title of this section","","","","","","","The units in the x and y axis of the chart (they are …","Panics if this <code>ValidationResult</code> is of type <code>Err</code>.","The Validator","An Attribute MAcro","Runs a validation procedure, returning an error message if …","Runs the validations, writes the report and fails the task …","The validations to run","A wrapper that contains an object that implements <code>Validate</code>","The label in the x axis of the chart","The units in the x axis of the chart","The label in the y axis of the chart","The units in the y axis of the chart","","Define the basic algebraic requirements for T","A simple trait required for initializing some matrices …","","Returns an element considered to be 1.","Returns an element considered to be 0.","Validates a time series based on Mean Bias Error and Root …","","","the title of the chart","","","","The time series containing the expected values","The name of the series caled <code>expected</code>","The time series containing the found values","The name of the <code>found</code> time series","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","The units in the x and y axis of the chart (they are …","","Calculates the coefficients $<code>a</code>$ and $<code>b</code>$ that best fit the …","Calculates the mean of a dataset","Calculates the Mean Bias Error between to datasets, …","Calculates the maximum and minimum in a series.","Calculates the Root Mean Squared Error between to …","Attempts transform a <code>usize</code> into a generic parameter <code>T</code>. …","Validates a time series based on Mean Bias Error and Root …","The maximum allowed Mean Bias Error","The maximum allowed Root Mean Squared Error","","","the title of the chart","","","","The time series containing the expected values","The name of the series caled <code>expected</code>","The time series containing the found values","The name of the <code>found</code> time series","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","The label in the x axis of the chart","The units in the x axis of the chart","The label in the y axis of the chart","The units in the y axis of the chart","","A wrapper that contains an object that implements <code>Validate</code>","","","A description","Format the description of a Validator","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","The title of the test","","","","The Validator","Validates a Wrapper"],"i":[9,9,0,0,0,0,0,0,17,17,0,0,9,4,9,4,17,13,18,17,13,17,13,17,13,17,13,9,4,0,9,4,4,0,4,0,0,4,0,18,4,9,4,9,4,9,4,13,9,18,0,5,4,4,0,17,17,17,17,0,0,0,19,20,20,0,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,0,0,0,0,0,0,0,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,0,0,18,18,18,18,18,18,18,18,18,18,18,18],"f":[0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[1,[[3,[[3,[2]]]]]],[[]],[[]],[[1,1],4],0,[[4,[6,[5]]]],0,0,0,0,0,0,[[],7],[[],7],[[],7],[[],7],[[],8],[[],8],0,[9],0,0,[[],9],[4,[[7,[10]]]],0,0,0,0,0,0,0,0,0,[[],11],[[]],[[]],0,[[]],[[]],0,[[[13,[12]]],[[13,[12]]]],[[]],[[],[[13,[14]]]],0,0,0,0,[[]],[[]],[[]],[[],7],[[],7],[[],8],0,[[[13,[2]]],9],[[]],[[],15],[[],15],[[]],[[],15],[16,2],0,0,0,[[]],[[]],0,[[[17,[[0,[12,2]]]]],[[17,[[0,[12,2]]]]]],[[]],[[],[[17,[[0,[14,2]]]]]],0,0,0,0,[[]],[[]],[[]],[[],7],[[],7],[[],8],[[[17,[2]]],9],0,0,0,0,0,0,[[]],[[]],0,[[18,10],10],[[]],[[]],0,[[],7],[[],7],[[],8],0,[18,9]],"p":[[15,"str"],[8,"Numberish"],[3,"Vec"],[3,"Validator"],[8,"Validate"],[3,"Box"],[4,"Result"],[3,"TypeId"],[4,"ValidationResult"],[3,"String"],[15,"bool"],[8,"Clone"],[3,"ScatterValidator"],[8,"Default"],[15,"f64"],[15,"usize"],[3,"SeriesValidator"],[3,"ValidatorWrapper"],[8,"Nanish"],[8,"OneZero"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
