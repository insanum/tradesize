
use std::env;
use getopts::Options;
use colored::*;

fn usage(program: &str, opts: Options) {
    let brief = format!("Usage {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn print_size(price: f64, percentage: f64, cash: f64, clr: Color) {
    let perc_txt;
    if percentage == 0.0 {
        perc_txt = "0".to_string();
    } else if percentage < 0.0 {
        perc_txt = format!("{}%", percentage).to_string();
    } else { // percentage > 0.0
        perc_txt = format!("+{}%", percentage).to_string();
    }

    let t_price = price * (1.0 + (percentage / 100.0));
    let t_cash  = cash * (1.0 + (percentage / 100.0));

    println!("{:>5}: {} {}",
             perc_txt,
             format!("{:.10}", t_price).color(clr),
             match percentage == 0.0 {
                 true  => format!("{:>10.2}", cash).color(clr).to_string(),
                 false => format!("{:>10.2}", (t_cash - cash)).color(clr).to_string()
             });
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("p", "", "current price", "<price>");
    opts.optopt("i", "", "investment total in dollars", "<cash>");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => m,
        Err(e) => return Err(e.to_string())?
    };

    if matches.opt_present("h") {
        usage(&program, opts);
        return Ok(());
    }

    if !matches.opt_present("p") || !matches.opt_present("i") {
        usage(&program, opts);
        return Ok(());
    }

    let price = match matches.opt_str("p").unwrap().parse::<f64>() {
                    Ok(n)  => n,
                    Err(e) => return Err(e.to_string())?
                };

    let cash = match matches.opt_str("i").unwrap().parse::<f64>() {
                    Ok(n)  => n,
                    Err(e) => return Err(e.to_string())?
               };

    print_size(price, 10.0, cash, Color::Green);
    print_size(price, 5.0, cash, Color::White);
    print_size(price, 4.0, cash, Color::Cyan);
    print_size(price, 3.0, cash, Color::White);
    print_size(price, 2.0, cash, Color::White);
    print_size(price, 1.0, cash, Color::White);
    print_size(price, 0.0, cash, Color::Magenta);
    print_size(price, -1.0, cash, Color::White);
    print_size(price, -2.0, cash, Color::Yellow);
    print_size(price, -3.0, cash, Color::White);
    print_size(price, -4.0, cash, Color::Red);
    print_size(price, -5.0, cash, Color::White);
    print_size(price, -10.0, cash, Color::White);

    return Ok(());
}

