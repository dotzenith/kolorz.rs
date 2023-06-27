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

kolorz is a silly little library for printing colored text to the terminal 

---

### ❖ Installation

Add kolorz to your project's `Cargo.toml`:

```toml
[dependencies]
kolorz = "0.2.0"
```

---

### ❖ Basic Usage

```rust
// print colored text
use kolorz::get_colorscheme;

fn main() {
    let mocha = get_colorscheme("catppuccin mocha");
    println!("{}", mocha.red("This is red"));
}
```

```rust
// get an array of all available colorschemes
use kolorz::get_all_colorschemes;

fn main() {
    let colorschemes = get_all_colorschemes();
}
```

---
 
### ❖ The following colorschemes are available:

- catppuccin latte
- catppuccin frappe
- catppuccin macchiato
- catppuccin mocha
- dracula
- nord
- gruvbox
- onedark
- tokyonight
- ayu
- palenight
- gogh

---

### ❖ The following colors are available on all of the colorschemes:

- red
- purple
- blue
- orange
- yellow
- white

---

### ❖ What's New? 
0.2.0 - Added a helper function to get an array of all colorschemes

---

<div align="center">

   <img src="https://img.shields.io/static/v1.svg?label=License&message=MIT&color=F5E0DC&labelColor=302D41&style=for-the-badge">

</div>
