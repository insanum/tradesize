
use std::env;
use getopts::Options;
use colored::*;

fn fmt_line(price: f64, percentage: f64, cash: f64, clr: Color) -> String
{
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

    format!("{:>5}: {} {}",
            perc_txt,
            format!("{:.10}", t_price).color(clr),
            match percentage == 0.0 {
                true  => format!("{:>10.2}", cash).color(clr).to_string(),
                false => format!("{:>10.2}", (t_cash - cash)).color(clr).to_string()
            })
}

fn usage(program: &str, opts: Options)
{
    let brief = format!("Usage {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("c", "", "total capital in dollars", "<cash>");
    opts.optopt("r", "", "risk percentage", "<risk>");
    opts.optopt("p", "", "current price", "<price>");
    opts.optopt("s", "", "stop price", "<stop>");

    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => m,
        Err(e) => return Err(e.to_string())?
    };

    if matches.opt_present("h") {
        usage(&program, opts);
        return Ok(());
    }

    if !matches.opt_present("c") || !matches.opt_present("r") ||
       !matches.opt_present("p") || !matches.opt_present("s") {
        usage(&program, opts);
        return Ok(());
    }

    let cash = match matches.opt_str("c").unwrap().parse::<f64>() {
                    Ok(n)  => n,
                    Err(e) => return Err(e.to_string())?
               };

    let risk = match matches.opt_str("r").unwrap().parse::<f64>() {
                    Ok(n)  => n,
                    Err(e) => return Err(e.to_string())?
               };

    let price = match matches.opt_str("p").unwrap().parse::<f64>() {
                    Ok(n)  => n,
                    Err(e) => return Err(e.to_string())?
                };

    let stop = match matches.opt_str("s").unwrap().parse::<f64>() {
                   Ok(n)  => n,
                   Err(e) => return Err(e.to_string())?
               };

    if stop >= price {
        return Err("invalid stop price")?;
    }

    let total_risk = cash * (risk / 100.0);
    let loss_delta = price - stop;
    let max_shares = total_risk / loss_delta;
    let pos_size = max_shares * price;

    println!("");

    println!("{:<16} {}",
             "Total capital:",
             format!("{:.02}", cash).magenta());
    println!("{:<16} {}",
             "Total risk:",
             format!("{:.02} ({:.02}%)", total_risk, risk).red());
    println!("{:<16} {}",
             "Buy price:",
             format!("{:.08}", price).yellow());
    println!("{:<16} {}",
             "Stop price:",
             format!("{:.08} (-{:.08})", stop, loss_delta).yellow());
    println!("{:<16} {}",
             "Max shares:",
             format!("{:.0}", max_shares).green().bold().underline());
    println!("{:<16} {}",
             "Position size:",
             format!("{:.02}", pos_size).green().bold().underline());

    println!("");

    println!("{}", fmt_line(price, 10.0, pos_size, Color::Green));
    println!("{}", fmt_line(price, 5.0, pos_size, Color::White));
    println!("{}", fmt_line(price, 4.0, pos_size, Color::Cyan));
    println!("{} <--", fmt_line(price, 3.0, pos_size, Color::Blue));
    println!("{}", fmt_line(price, 2.0, pos_size, Color::White));
    println!("{}", fmt_line(price, 1.0, pos_size, Color::White).underline());
    println!("{}", fmt_line(price, 0.0, pos_size, Color::Magenta).underline());
    println!("{} <--", fmt_line(price, -1.0, pos_size, Color::White));
    println!("{}", fmt_line(price, -2.0, pos_size, Color::Yellow));
    println!("{}", fmt_line(price, -3.0, pos_size, Color::White));
    println!("{}", fmt_line(price, -4.0, pos_size, Color::Red));
    println!("{}", fmt_line(price, -5.0, pos_size, Color::White));
    println!("{}", fmt_line(price, -10.0, pos_size, Color::White));

    println!("");

    return Ok(());
}

