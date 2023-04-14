use super::structs::Variables;
use std::num::ParseFloatError;

pub fn valid(c: &Result<f64, ParseFloatError>) -> bool {
    c.is_ok()
}

pub fn args_to_icognities(args: Vec<String>) -> Option<Variables> {
    let a = args.get(1)?.parse::<f64>();
    let b = args.get(2)?.parse::<f64>();
    let c = args.get(3)?.parse::<f64>();

    if valid(&a) && valid(&b) && valid(&c) {
        let a = a.unwrap();
        let b = b.unwrap();
        let c = c.unwrap();

        return Some(Variables { a, b, c });
    }

    None
}
