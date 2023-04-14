use super::structs::Variables;

pub fn r3(var: Variables) -> f64 {
    (var.b * var.c) / var.a
}
