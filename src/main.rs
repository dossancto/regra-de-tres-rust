use std::env;
mod mat;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        print!("Wrong number of args");
        return;
    }

    let vars = mat::checks::args_to_icognities(args);

    if vars.is_some() {
        println!("{}", mat::calc::r3(vars.unwrap()));
        return;
    }

    println!("Inválid Args");
}
