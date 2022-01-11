use dialoguer::{
    Confirm,
    MultiSelect,
    Editor,
    Input,
    Sort,
    FuzzySelect,
    theme::ColorfulTheme
};
use console::Term;

fn main() -> std::io::Result<()> {

    let selections = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
    ];

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("Enjoy your {}!", selections[selection]);



    Ok(())
}