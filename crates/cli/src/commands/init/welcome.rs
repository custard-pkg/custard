use colored::Colorize;
use rust_i18n::t;

pub fn show() {
    println!(
        "{}",
        t!("init.welcome.short", command = "`custard init`!")
            .bold()
            .purple()
    );
    println!("{}", t!("init.welcome.long"));
}
