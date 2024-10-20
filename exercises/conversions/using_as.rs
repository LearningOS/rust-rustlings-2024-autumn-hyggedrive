// using_as.rs
//
// Type casting in Rust is done via the usage of the `as` operator. Please note
// that the `as` operator is not only used when type casting. It also helps with
// renaming imports.
//
// The goal is to make sure that the division does not fail to compile and
// returns the proper type.
//
// Execute `rustlings hint using_as` or use the `hint` watch subcommand for a
// hint.


fn average(values: &[f64]) -> Option<f64> {
    let len=values.len();
    if len==0{
        return None;
    }
    let total=values.iter().sum::<f64>();
    Some(total/len as f64)
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    if let Some(result)=average(&values){
        println!("{}",result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]),Some(7.125));
    }
}
