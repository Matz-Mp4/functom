use colored::Colorize;
use functom::evaluate::evaluate_at;
use functom::integral::calc_integral;
use functom::parser;
use functom::token::Token;

use std::env;
use std::io;
use std::io::Write;

fn main() {
    if env::args().count() > 1 {
        let mut expr = String::new();
        for argument in env::args().skip(1) {
            expr = format!("{} {}", expr, argument);
        }

        /* eval_and_print(expr); */
    } else {
        let mut temp = String::new();
        print!("Forneca uma expressao: ");
        io::stdout().flush().ok().expect("failed to flush stdout");

        let mut expr = String::new();

        io::stdin()
            .read_line(&mut expr)
            .expect("failed to read line");

        print!("Forneca o inicio do intervalo: ");
        io::stdout().flush().ok().expect("failed to flush stdout");

        let x_begin = {
            io::stdin()
                .read_line(&mut temp)
                .expect("failed to read line");
            temp = temp.trim().to_string();
            temp.parse::<f64>().unwrap()
        };

        print!("Forneca o fim do intervalo: ");
        io::stdout().flush().ok().expect("failed to flush stdout");

        let x_end = {
            temp.clear();
            io::stdin()
                .read_line(&mut temp)
                .expect("failed to read line");
            temp = temp.trim().to_string();
            temp.parse::<f64>().unwrap()
        };

        print!("Forneca a quantidade de trapezios: ");
        io::stdout().flush().ok().expect("failed to flush stdout");

        let num_trapezoids = {
            temp.clear();
            io::stdin()
                .read_line(&mut temp)
                .expect("failed to read line");

            temp = temp.trim().to_string();
            temp.parse::<u32>().unwrap()
        };

        print!("Forneca a quantidade de casas decimais: ");
        io::stdout().flush().ok().expect("failed to flush stdout");

        let precision = {
            temp.clear();
            io::stdin()
                .read_line(&mut temp)
                .expect("failed to read line");
            temp = temp.trim().to_string();
            temp.parse::<u32>().unwrap()
        };

        let result = calc_integral(expr, precision as f64, x_begin, x_end, num_trapezoids as i8);
        let n = precision as usize;

        match result {
            Ok((value, error, integral_table)) => {
                println!("\n{} = {}", "Erro".red().bold(), error);
                println!("{} = {:.n$}", "Somatorio das areas".yellow().bold(), value);
                println!("{}", "(Tabela)".green().bold());

                for (x, f) in integral_table {
                    let x_temp = x.to_string().green().bold();
                    let f_temp = f.to_string().green().bold();
                    println!(" * f({:.n$}) => {:.n$} ", x_temp, f_temp);
                }
            }
            Err(e) => println!("{}\n", e),
        }
    }
}
