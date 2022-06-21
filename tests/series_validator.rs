use validate::{SeriesValidator, Validator};

#[test]
fn test_series_validator() {
    let expected = vec![1., 2., 3.];
    let found = vec![5., 6., 6.];

    let mut validator = Validator::new("Validate Time series", "report.html");
    // Note that we are not defining a maximum allowed error
    let v = SeriesValidator {
        x_label: Some("time step"),
        y_label: Some("Zone Temperature"),
        y_units: Some("C"),
        title: "Compare Series!",
        expected,
        found,
        ..validate::SeriesValidator::default()
    };
    validator.push(Box::new(v));

    // This will not fail because we did not set a maximum allowed
    // Root Mean Square Error or Mean Bias Error... if we did, it would return
    // an error and the unwrap woul
    validator.validate().unwrap();
}
