use rust_i18n::t;
use colored::Colorize;

pub fn show() {
    println!(
        "{}",
        t!("init.welcome.short", command = "`custard init`!")
            .bold()
            .purple()
    );
    println!("{}", t!("init.welcome.long"));
}