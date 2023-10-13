<p align="center"><img src="https://github.com/pwnwriter/kanha/blob/logos/shree.svg" width="250px" height="100px" >
<h4 align="center"><strong><code>Kanha</code></strong> - A web-app pentesting suite written in rust 🦀</h4> </h6>

<h6 align="center">
  <a href="https://github.com/pwnwriter/kanha#-installation"><code>Installation</code></a>
  ⦾
  <a href="https://github.com/pwnwriter/kanha#-Subcommands"><code>Subcommands</code></a>
  ⦾
  <a href="https://github.com/pwnwriter/kanha#-contributing"><code>Contribute</code></a>
 </h6>
<p align="center">
<a href="https://crates.io/crates/kanha/"><img src="https://img.shields.io/crates/v/kanha?style=flat&amp;labelColor=56534b&amp;color=c1c1b6&amp;logo=Rust&amp;logoColor=white" alt="Crate Release"></a>
<a href="https://github.com/pwnwriter/kanha/issues"><img src="https://img.shields.io/github/issues/pwnwriter/kanha.svg?style=flat-square&label=Issues&color=d77982"></a>
<a href="https://github.com/pwnwriter/pwnwriter/blob/main/LICENSE"><img src="https://img.shields.io/badge/License-MIT-white.svg" alt="MIT LICENSE"></a>
<a href="https://ko-fi.com/pwnwriter"><img src="https://img.shields.io/badge/support-pwnwriter%20-pink?logo=kofi&logoColor=white" alt="Ko-fi"></a>

![-----------------------------------------------------](https://github.com/pwnwriter/haylxon/blob/readme-assets/colored.png)

<img src="https://github.com/pwnwriter/kanha/blob/logos/kanha-help.png" alt="img" align="right" width="50%"></p>
    
[**`Kanha`**](/) is a tool that can help you perform, a variety of attacks based on the target domain . With just `kanha` you can do, [***`Fuzzing`***](https://en.wikipedia.org/wiki/Fuzzing), [***`Reverse dns lookup`***](https://en.wikipedia.org/wiki/Reverse_DNS_lookup),
[***`common http response`***](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes), [***`subdomain takeover detection`***](https://en.wikipedia.org/wiki/Domain_hijacking) and many [**`more`**](/src/commands). 

The project is inspird by [`mini.nvim`](https://github.com/echasnovski/mini.nvim), basically helping you to be productive with less numbers of *tools(plugins)* installed on your system and be unobtrusive and function as a standalone **`single binary`** out of the box.

Built from the ground up with performance, ease of use, and portability in mind in your favourite programming lang [**`rust`**](https://www.rust-lang.org/) 💝

## 🧠 Philosophy

- **KISS** - Keep things simple and stupid.
- **Ease** - Write code that can be used elsewhere as well.
- **Efficiency** - Optimize for performance without sacrificing readability.

## 🐱 Installation 
    
  <details> <summary><code>🪄 Binary </code></summary>
    &nbsp;

  - You can directly download the [**binary**](https://github.com/pwnwriter/kanha/releases) of your arch and run it.
  
  </details>
  <details> <summary><code>🌼 Source </code></summary>
  &nbsp;
 
  ```bash
  git clone --depth=1 https://github.com/pwnwriter/kanha --branch=main
  cd kanha
  cargo build --release 
  ```
  Then go to `release` dir and `./kanha` or move the `binary` to your any `$PATH` for instant access from anywhere.
</details>

<details> <summary><code>🎠 Cargo </code></summary>

- Using [crates.io](https://crates.io/crates/kanha)
  ```bash
  cargo install kanha
  ```
- Using [binstall](https://github.com/cargo-bins/cargo-binstall)
  ```bash
  cargo binstall kanha
  ```

  > **Note** ⚠️
  > This requires a working setup of rust/cargo & binstall.
</details>

<details> <summary><code>🚩 METIS Linux </code></summary>
&nbsp;
  
  ```bash
  sudo/doas pacman -Syyy kanha
  ```

</details>

## 🌈 Subcommands
- ➊ `Status` :- Just return the HTTP response code of URLs

  <details>
  <summary>👻 Help</summary>
  &nbsp;

  ```bash
  $ kanha status -h
  Just return the HTTP response code of URLs

  Usage: kanha status [OPTIONS]

  Options:
    -f, --filename <FILENAME>  A file containing multiple urls
    -t, --tasks <TASKS>        Define the maximum concurrent tasks [default: 20]
        --stdin                Reads input from the standard in
        --exclude <EXCLUDE>    Define your status code for selective exclusion
    -h, --help                 Print help
    -V, --version              Print version

  ```

  <details>
  <summary>🦊 Screenshots </summary>
      &nbsp;
    
  ![status](https://github.com/pwnwriter/kanha/assets/90331517/93f7656f-563c-4c92-a118-500b1fabae9e)
  ![status-stdin](https://github.com/pwnwriter/kanha/assets/90331517/5ac0d6c6-497a-4a8d-a1a2-d3326010d7a8)  

  </details>

</details>

- ➋  `fuzz` :- Fuzz URLs and return the response codes
    
  <details>
  <summary>👻 Help</summary>
  &nbsp;
    
  ```bash
  $ kanha fuzz -h
  Fuzz a URL and return the response codes

  Usage: kanha fuzz [OPTIONS] --payloads <PAYLOADS>

  Options:
    -p, --payloads <PAYLOADS>    A file containing a list of payloads
    -u, --url <URL>              A single url
    -f, --file-path <FILE_PATH>  Path of the file containing multiple urls
    -t, --tasks <TASKS>          Define the maximum concurrent tasks [default: 20]
        --exclude <EXCLUDE>      Define your status code for selective exclusion
        --stdin                  Reads input from the standard in
    -h, --help                   Print help
    -V, --version                Print version

  ```
    <details>
  <summary>🦊 Screenshots </summary>
      &nbsp;
      
  ![screenshot_2023-10-13_14-08-46](https://github.com/pwnwriter/kanha/assets/90331517/e3418630-b5c4-4986-95e8-57f832ee91f2)
  ![screenshot_2023-10-13_14-07-45](https://github.com/pwnwriter/kanha/assets/90331517/692e1214-8b32-40c9-bae7-9cce1ab064f0)

  </details>
  
  </details>

- ➌ `rdns` :- Reverse dns lookup
    <details>
  <summary>👻 Help</summary>  
  &nbsp;
      
  ```bash
  
  $ kanha rdns  -h
  Reverse dns lookup

  Usage: kanha rdns [OPTIONS] --filename <FILENAME>

  Options:
    -f, --filename <FILENAME>  a file containing a list of possible wordlists
        --stdin                Reads input from the standard in
    -h, --help                 Print help
    -V, --version              Print version
  ```
    <details>
  <summary>🦊 Screenshots </summary>
      &nbsp;
      
  ![rdns](https://github.com/pwnwriter/kanha/assets/90331517/44f2f7f1-9f47-4794-87e9-1366b4a3e443)
  ![rdns-stdin](https://github.com/pwnwriter/kanha/assets/90331517/9ad5e5b6-711e-4396-a46f-5c190000e185)

  </details>
</details>

- ➍ `Takeover` :- Check possible subdomain takeover
    <details>
  <summary>👻 Help</summary>  
  &nbsp;
      
  ```bash
  $ kanha takeover -h
  Check possible subdomain takeover vulnerability

  Usage: kanha takeover [OPTIONS]

  Options:
    -u, --url <URL>              A single url
    -f, --file-path <FILE_PATH>  Path of the file containing multiple urls
    -j, --json-file <JSON_FILE>  A json file containing signature values of different services
        --stdin                  Reads input from the standard in
    -h, --help                   Print help
    -V, --version                Print version

  ```
    <details>
  <summary>🦊 Screenshots </summary>
      &nbsp;
      
  ![Takeover single](https://github.com/pwnwriter/kanha/assets/90331517/b868146d-03eb-4803-81b3-5283135f3d4f)
  ![Takeover multiple](https://github.com/pwnwriter/kanha/assets/90331517/f13e0001-b39d-4132-b98f-86836f789be3)

  ![takeover-stdin](https://github.com/pwnwriter/kanha/assets/90331517/1b956c9d-2d37-4656-97ee-2aca2199750b)
  </details>


- ➎ `urldencode` :- (De|En) code urls
  <details>
  <summary>👻 Help</summary>  
  &nbsp;
      
  ```bash
  $ kanha urldencode -h
  (De|En) code urls

  Usage: kanha urldencode [OPTIONS]
  
  Options:
        --encode <ENCODE>  Provide a url to encode
        --decode <DECODE>  Provide a url to dencode
    -h, --help             Print help
    -V, --version          Print version
  
  ```
    <details>
  <summary>🦊 Screenshots </summary>
      &nbsp;
      
  ![urldencode](https://github.com/pwnwriter/kanha/assets/90331517/b757694d-a4f9-4ea2-a962-c8419fe59b56)
  ![urldencode](https://github.com/pwnwriter/kanha/assets/90331517/dcf45079-7514-4951-b264-a3e6f708ebdb)    
  </details>
</details>



<!--  ➏ ➐ ➑ ➒ -->

## 👐 Contributing
  - 🪶 Recommend a new features
  - ⭐ Give the project a star
  - 🐎 Add new [subcommand](/src/commands).
  - 🧑‍🚒 Fix docx // improve code quality

## 👀 Also see 
- [`haylxon`](https://github.com/pwnwriter/haylxon) :- Blazingly fast tool to grab screenshots of your domain list right from terminal written in rust 🦀
- [`httpx`](https://github.com/projectdiscovery/httpx) :- httpx is a fast and multi-purpose HTTP toolkit.
- [`ffuf`](https://github.com/ffuf/ffuf) :- Fast web fuzzer written in Go

## 🔏 License 
 As always, this project is also licensed under the [**`MIT LICENSE`**](/LICENSE) 
 &nbsp;
 
<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
<p align="center">Copyright &copy; 2023<a href="https://pwnwriter.xyz" target="_blank"> pwnwriter xyz </a> ☘️ </p> 
  
