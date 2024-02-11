use kolorz::{Kolor, KolorScheme};
use KolorScheme::*;

fn main() {
    let colorschemes = [
        CatppuccinLatte,
        CatppuccinFrappe,
        CatppuccinMacchiato,
        CatppuccinMocha,
        Dracula,
        Nord,
        GruvboxDark,
        GruvboxLight,
        OneDark,
        TokyoNight,
        Ayu,
        PaleNight,
        Gogh,
        BiscuitDark,
        BiscuitLight,
    ];

    for colorscheme in colorschemes.iter() {
        let cur = Kolor::new(colorscheme);
        println!("{:?}", colorscheme);
        println!(
            "{} {} {} {} {} {} {} {}",
            cur.red("red"),
            cur.purple("purple"),
            cur.blue("blue"),
            cur.green("green"),
            cur.orange("orange"),
            cur.yellow("yellow"),
            cur.text("text"),
            cur.random("random\n")
        );
    }
}
