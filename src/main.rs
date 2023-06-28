use kolorz::Kolor;

fn main() {
    let colorschemes = Kolor::get_all_kolorschemes();

    for colorscheme in colorschemes.iter() {
        let cur = Kolor::new(colorscheme);
        println!("{}", colorscheme);
        println!("{}", cur.red("red"));
        println!("{}", cur.purple("purple"));
        println!("{}", cur.blue("blue"));
        println!("{}", cur.green("green"));
        println!("{}", cur.orange("orange"));
        println!("{}", cur.yellow("yellow"));
        println!("{}", cur.white("white\n"));
    }
}
