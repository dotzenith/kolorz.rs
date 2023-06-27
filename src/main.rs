use kolorz::{get_all_colorschemes, get_colorscheme};

fn main() {
    let colorschemes = get_all_colorschemes();

    for colorscheme in colorschemes.iter() {
        let cur = get_colorscheme(colorscheme);
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
