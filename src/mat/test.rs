#[cfg(test)]
mod tests {
    use crate::mat::checks::args_to_icognities;
    use crate::mat::{calc::r3, structs::Variables};

    #[test]
    fn works() {
        let correct_vars = Variables {
            a: 20.,
            b: 100.,
            c: 2.,
        };

        assert_eq!(r3(correct_vars), 10.);
    }

    #[test]
    fn correct_args() {
        let args = vec![
            String::from("program_name"),
            String::from("20"),
            String::from("100"),
            String::from("2"),
        ];

        let vars = args_to_icognities(args);

        assert_eq!(r3(vars.unwrap()), 10.);
    }

    #[test]
    fn wrong_args() {
        let args = vec![String::from("20"), String::from("100a"), String::from("2")];

        let vars = args_to_icognities(args);

        assert!(vars.is_none());
    }
}
