pub fn float(x: &str) -> Result<f64, String> {
    let float_x = x.parse::<f64>();
    return match float_x {
        Ok(x) => Ok(x),
        Err(_) => Err(f!("Could not parse {x} to a float."))
    }
}

pub fn uint(x: &str) -> Result<usize, String> {
    let int_x =  x.parse::<usize>();
    return match int_x {
        Ok(x) => Ok(x),
        Err(_) => Err(f!("Could not parse {x} to an unsigned integer."))
    }
}

pub fn float_in_range(x: &str, range: [f64; 2]) -> Result<f64, String> {
    let float_x =  float(&x);

    let lower_bound = range.first().unwrap();
    let upper_bound = range.last().unwrap();
    return match float_x {
        Ok(x) if &x < lower_bound => Err(f!("{x} < {lower_bound}")),
        Ok(x) if &x > upper_bound => Err(f!("{x} > {upper_bound}")),
        _ => float_x
    }
}

pub fn limited_natural_num(x: &str, max_val: usize) -> Result<usize, String> {
    let int_x = uint(&x);

    return match int_x {
        Ok(x) if x > max_val => Err(f!("{x} > {max_val}")),
        _ => int_x
    }
}
