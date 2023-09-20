use kolorz::Kolor;

fn main() {
    let colorschemes = Kolor::get_all_kolorschemes();

    for colorscheme in colorschemes.iter() {
        let cur = Kolor::new(colorscheme);
        println!("{colorscheme}");
        println!(
            "{} {} {} {} {} {} {}",
            cur.red("red"),
            cur.purple("purple"),
            cur.blue("blue"),
            cur.green("green"),
            cur.orange("orange"),
            cur.yellow("yellow"),
            cur.text("text\n")
        );
    }
}
