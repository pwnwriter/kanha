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

![-----------------------------------------------------][line]

<img src="https://github.com/pwnwriter/kanha/assets/90331517/7fbabb20-44bf-4d34-9602-fad6e0487b5e" alt="img" align="right" width="50%"></p>
    
[**`Kanha`**](/) is a tool that can help you perform, a variety of attacks based on the target domain . With just `kanha` you can do, [***`Fuzzing`***][wiki-fuzzing], [***`Reverse dns lookup`***][wiki-dns-lookup], [***`common http response`***][wiki-http], [***`subdomain takeover detection`***][wiki-subdomain] and many [**`more`**][commands]. 

The project is inspird by [`mini.nvim`][mini], basically helping you to be productive with less numbers of *tools(plugins)* installed on your system and be unobtrusive and function as a standalone **`single binary`** out of the box.

Built from the ground up with performance, ease of use, and portability in mind in your favourite programming lang [**`rust`**][rust] 💝

## Philosophy

- **KISS** - Keep things simple and stupid.
- **Ease** - Write code that can be used elsewhere as well.
- **Efficiency** - Optimize for performance without sacrificing readability.

## Installation 
    
  <details> <summary><code>Binary </code></summary>
    &nbsp;

  - *Manual* : You can directly download the binary of your arch from [**releases**][releases] and run it.
  - *One liner* : Run this script, requires `jq`,`curl`, `tar` & `wget`
   ```bash
wget -qO- "$(curl -qfsSL "https://api.github.com/repos/pwnwriter/kanha/releases/latest" | jq -r '.assets[].browser_download_url' | grep -Ei "$(uname -m).*$(uname -s).*musl" | grep -v "\.sha")" | tar -xzf - --strip-components=1
./kanha -h
``` 
  </details>

> [!IMPORTANT]
> *_For upstream updates, it's recommended to build `kanha` from source !_*

<details open> <summary><code>Source </code></summary>
&nbsp;

   
  ```bash
    git clone --depth=1 https://github.com/pwnwriter/kanha --branch=main
    cd kanha
    cargo build --release
  ``` 

</details>

<details> <summary><code>Cargo </code></summary>

- Using [crates.io][crate]
  ```bash
  cargo install kanha
  ```
- Using [binstall][binstall]
  ```bash
  cargo binstall kanha
  ```

  > **Note** ⚠️
  > This requires a working setup of rust/cargo & binstall.
</details>

<details> <summary><code>METIS Linux </code></summary>
&nbsp;
  
  ```bash
  sudo/doas pacman -Syyy kanha
  ```

</details>

<details> <summary><code>Arch user repository </code></summary>
&nbsp;
  
  ```bash
  paru/yay -S kanha-git
  ```

</details>

## Subcommands
- ➊ `Status` :- Just return the HTTP response code of URLs

  <details>
  <summary>Help</summary>
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

</details>

- ➋  `fuzz` :- Fuzz URLs and return the response codes
    
  <details>
  <summary>Help</summary>
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
  </details>

- ➌ `rdns` :- Reverse dns lookup
    <details>
  <summary>Help</summary>  
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
</details>

- ➍ `Takeover` :- Check possible subdomain takeover
    <details>
  <summary>Help</summary>  
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
    </details>


- ➎ `urldencode` :- (De|En) code urls
  <details>
  <summary> Help</summary>  
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
</details>



<!--  ➏ ➐ ➑ ➒ -->

## Contributing
  - Recommend a new feature
  - Give the project a star
  - Add new [subcommand][commands].
  - Fix docx and improve code quality

## Also see 
- [`haylxon`][haylxon] :- Blazingly fast tool to grab screenshots of your domain list right from terminal written in rust 🦀
- [`httpx`][httpx] :- httpx is a fast and multi-purpose HTTP toolkit.
- [`ffuf`][ffuf] :- Fast web fuzzer written in Go

## FAQ 
 
- **Development:**
  - Progress may be gradual, but I assure you of delivering quality code!
- **Why this?**
  - This is a way for me to continually expand my knowledge in cybersecurity and Rust!
- **I want my quote in Kanha.**
  - Please feel free to add it [here][splash].

## Support
I am a student, i like working for open-source during my free time. If you appreciate my work, kindly consider supporting me through [Ko-fi][Ko-Fi].

## Copying 
 `Kanha` is licensed under the [**`MIT LICENSE`**][license], Feel free to consider Kanha as your own!
 

<!-- Links -->

[license]:/LICENSE
[splash]:/src/interface/splashes.rs
[commands]:/src/commands
[releases]:https://github.com/pwnwriter/kanha/releases

[line]:https://github.com/pwnwriter/haylxon/blob/readme-assets/colored.png

[Ko-Fi]:https://ko-fi.com/pwnwriter
[haylxon]:https://github.com/pwnwriter/haylxon

[ffuf]:https://github.com/ffuf/ffuf
[httpx]:https://github.com/projectdiscovery/httpx

[crate]:https://crates.io/crates/kanha
[binstall]:https://github.com/cargo-bins/cargo-binstall
[mini]:https://github.com/echasnovski/mini.nvim
[rust]:https://www.rust-lang.org

[wiki-fuzzing]:https://en.wikipedia.org/wiki/Fuzzing
[wiki-dns-lookup]:https://en.wikipedia.org/wiki/Reverse_DNS_lookup
[wiki-http]:https://en.wikipedia.org/wiki/List_of_HTTP_status_codes
[wiki-subdomain]:https://en.wikipedia.org/wiki/Domain_hijacking



<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
<p align="center">Copyright &copy; 2023 - present <a href="https://pwnwriter.xyz" target="_blank"> pwnwriter xyz </a> ☘️ </p>
