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
kolorz = "0.4.0"
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

```rust
// get an array of all available kolorschemes
use kolorz::Kolor;

fn main() {
    let kolorschemes = Kolor::get_all_kolorschemes();
}
```

---
 
### ❖ The following kolorschemes are available:

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

### ❖ The following kolors are available on all of the kolorschemes:

- red
- purple
- blue
- orange
- yellow
- text (usually white on dark kolorschemes)

---

### ❖ What's New? 
0.4.1 - Abstracted duplicate code into private method

---

<div align="center">

   <img src="https://img.shields.io/static/v1.svg?label=License&message=MIT&color=F5E0DC&labelColor=302D41&style=for-the-badge">

</div>
