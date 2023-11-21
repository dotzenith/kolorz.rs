<h2 align="center"> ━━━━━━  ❖  ━━━━━━ </h2>

<!-- BADGES -->
<div align="center">
   <p></p>
   
   <img src="https://img.shields.io/github/stars/dotzenith/kolorz.rs?color=F8BD96&labelColor=302D41&style=for-the-badge">   

   <img src="https://img.shields.io/github/forks/dotzenith/kolorz.rs?color=DDB6F2&labelColor=302D41&style=for-the-badge">   

   <img src="https://img.shields.io/github/repo-size/dotzenith/kolorz.rs?color=ABE9B3&labelColor=302D41&style=for-the-badge">
   
   <img src="https://img.shields.io/github/commit-activity/y/dotzenith/kolorz.rs?color=96CDFB&labelColor=302D41&style=for-the-badge&label=COMMITS"/>
   <br>
</div>

<p/>

---

### ❖ Information

kolorz is a silly little library for printing kolored text to the terminal 

---

### ❖ Installation

Add kolorz to your project's `Cargo.toml`:

```toml
[dependencies]
kolorz = "0.8.0"
```

---

### ❖ Basic Usage

```rust
// print kolored text
use kolorz::Kolor;

fn main() {
    let mocha = Kolor::new("catppuccin mocha");
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

- red
- purple
- blue
- orange
- yellow
- text (usually white on dark kolorschemes)

---

### ❖ Kustom Kolorz are also available

```rust
// custom kolorz from hex
use kolorz::HexKolorize;

fn main() {
    println!("{}", "This is peach".kolorize("#fab387"));
}
```

```rust
// custom kolorz from RGB
use kolorz::RGBKolorize;

fn main() {
    println!("{}", "This is red".kolorize((235, 160, 172)));
}
```

---

### ❖ What's New? 
0.8.0 - Add traits to implement kustom kolorz

---

<div align="center">

   <img src="https://img.shields.io/static/v1.svg?label=License&message=MIT&color=F5E0DC&labelColor=302D41&style=for-the-badge">

</div>
