use std::collections::VecDeque;

use crate::{error::ParseError, evaluate::evaluate_at};

pub fn calc_integral(
    //function expression
    expr: String,
    //Amount of digits
    precision: f64,
    start: f64,
    end: f64,
    //Amount of trapzoids
    num: i8,
) -> Result<(f64, f64, VecDeque<(f64, f64)>), ParseError> {
    //Height
    let inc = ((end - start).abs()) / num as f64;
    //Integral
    let mut sum = 0.0;
    let mut x_temp = start;
    let mut f_begin: f64 = 0.0;
    let mut function_values = VecDeque::new();
    let mut f_end: f64 = 0.0;
    let mut i = 1;

    loop {
        if i > num {
            break;
        }
        f_begin = set_value(precision, evaluate_at(x_temp, expr.clone())?);
        function_values.push_back((x_temp, f_begin));
        x_temp += inc;
        f_end = set_value(precision, evaluate_at(x_temp, expr.clone())?);
        sum += inc * (f_begin + f_end) / 2.0;
        i += 1;
    }

    f_end = set_value(precision, evaluate_at(end, expr.clone())?);
    function_values.push_back((end, f_end));

    let error = (num as f64) * (5.0 / 10.0f64.powf(precision + 1.0)) * inc;
    return Ok((sum, error, function_values));
}

fn set_value(precision: f64, mut value: f64) -> f64 {
    value *= 10.0f64.powf(precision);
    value = value.round();
    value = value / 10.0f64.powf(precision);

    return value;
}
