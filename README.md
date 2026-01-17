<h2 align="center"> ━━━━━━  ❖  ━━━━━━ </h2>

### ❖ Information

kolorz is a silly little library for printing kolored text to the terminal

It supports a number of [kolorschemes](#-the-following-kolorschemes-are-available),
and it respects [CLICOLOR and friends](https://bixense.com/clicolors/)

---

### ❖ Installation

Add kolorz to your project's `Cargo.toml`:

```toml
[dependencies]
kolorz = "1.0.0"
```

---

### ❖ Basic Usage

```rust
// print kolored text
use kolorz::Kolor;

fn main() {
    let mocha = Kolor::new("catppuccin mocha").expect("Invalid kolorscheme");
    println!("{}", mocha.red("This is red"));
}
```

---
 
### ❖ The following kolorschemes are available:

- [catppuccin](https://github.com/catppuccin) - latte, frappe, macchiato, mocha
- [nord](https://github.com/arcticicestudio/nord)
- [dracula](https://github.com/dracula/dracula-theme)
- [gruvbox](https://github.com/morhetz/gruvbox) - dark, light
- [onedark](https://github.com/joshdick/onedark.vim)
- [tokyonight](https://github.com/folke/tokyonight.nvim)
- [ayu](https://github.com/ayu-theme)
- [palenight](https://github.com/drewtempelmeyer/palenight.vim)
- [gogh](https://github.com/Mayccoll/Gogh)
- [biscuit](https://github.com/Biscuit-Colorscheme/biscuit) - dark, light

---

### ❖ The following kolors are available on all of the kolorschemes:

- red           (0)
- purple        (1)
- blue          (2)
- green         (3)
- orange        (4)
- yellow        (5)
- text          (6)
- random (picks a random kolor from above)
- numbered (allows the user to pick a kolor by number)

---

### ❖ Kustom Kolorz are also available

```rust
// If the hex code is invalid, the text will not be colored
use kolorz::HexKolorize;

fn main() {
    println!("{}", "This is peach".kolorize("#fab387"));
}
```

```rust
use kolorz::RGBKolorize;

fn main() {
    println!("{}", "This is red".kolorize((235, 160, 172)));
}
```

---

### ❖ What's New? 
1.1.0 - Remove `Result` for `HexKolorize`

---

<div align="center">

   <img src="https://img.shields.io/static/v1.svg?label=License&message=MIT&color=F5E0DC&labelColor=302D41&style=for-the-badge">

</div>
