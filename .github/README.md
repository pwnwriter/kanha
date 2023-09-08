<p align="center"><img src="https://github.com/pwnwriter/kanha/blob/logos/kanha-default.png" width="400px" height="250px" >
<h4 align="center"><strong><code>Kanha</code></strong> - A web-app pentesting suite written in rust 🦀</h4> </h6>

<h6 align="center">
  <a href="https://github.com/pwnwriter/kanha#-installation"><code>Installation</code></a>
  ⦾
  <a href="https://kanha.pwnwriter.xyz"><code>Subcommands</code></a>
  ⦾
  <a href="https://kanha.pwnwriter.xyz"><code>Features</code></a>
  ⦾
  <a href="https://kanha.pwnwriter.xyz"><code>Contribute</code></a>
</p> </h6>
<p align="center">
<a href="https://crates.io/crates/kanha/"><img src="https://img.shields.io/crates/v/kanha?style=flat&amp;labelColor=56534b&amp;color=c1c1b6&amp;logo=Rust&amp;logoColor=white" alt="Crate Release"></a>
<a href="https://github.com/pwnwriter/kanha/issues"><img src="https://img.shields.io/github/issues/pwnwriter/kanha.svg?style=flat-square&label=Issues&color=d77982"></a>
<a href="https://github.com/pwnwriter/pwnwriter/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-MIT-white.svg" alt="MIT LICENSE"></a>
<a href="https://ko-fi.com/pwnwriter"><img src="https://img.shields.io/badge/support-pwnwriter%20-pink?logo=kofi&logoColor=white" alt="Ko-fi"></a>

![-----------------------------------------------------](https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/aqua.png)

<img src="https://github.com/pwnwriter/kanha/blob/logos/kanha-help.png" alt="img" align="right" width="50%"></p>
    
[**`Kanha`**](/) is a tool that can help you perform, a variety of attacks based on the target domain . With just `kanha` you can do, [***`Fuzzing`***](https://en.wikipedia.org/wiki/Fuzzing), [***`Reverse dns lookup`***](https://en.wikipedia.org/wiki/Reverse_DNS_lookup),
[***`common http response`***](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes), [***`subdomain takeover detection`***](https://en.wikipedia.org/wiki/Domain_hijacking) and many [**`more`**](/src/commands). 

The project is inspird by [`mini.nvim`](https://github.com/echasnovski/mini.nvim), basically helping you to be productive with less numbers of *tools(plugins)* installed on your system and be unobtrusive and function as a standalone **`single binary`** by out of the box.

Built from the ground up with performance, ease of use, and portability in mind in your favourite programming lang [**`rust`**](https://www.rust-lang.org/) 💝

### 🧠 Philosophy

- **KISS** - Keep things simple and stupid.
- **Ease** - Write code that can be used elsewhere as well.
- **Efficiency** - Optimize for performance without sacrificing readability.

### 🐱 Installation 
    
- **Binary**:
  You can directly download the [`binary`](https://github.com/pwnwriter/kanha/releases/) of your arch and run it.

- **Source**: `Recommended for upstream updates`
 
  ```bash
  git clone --depth=1 https://github.com/pwnwriter/kanha --branch=main
  cd kanha
  cargo build --release 
  ```
  Then go to `release` dir and `./kanha` or move the `binary` to your any `$PATH` for instant access from anywhere.

 
- **Cargo**:
  ```bash
  cargo (binstall)install kanha
  ```
  > **Note** 
  > This requires a working setup of rust/cargo.

  
- **[`METIS Linux`](https://metislinux.org)** (**based**):
  ```
  sudo/doas pacman -Syyy kanha
  ```




<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
<p align="center">Copyright &copy; 2023<a href="https://pwnwriter.xyz" target="_blank"> pwnwriter xyz ☘️ </a> 
  

