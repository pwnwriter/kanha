// LICENSE: https://en.wikipedia.org/wiki/Creative_Commons_license

use colored::Colorize;

pub static SPLASHES: &[&str] = &[
    "There are reasons to use rust. - PwnWriter",
    "whatsoever a man soweth, that shall he also reap. - Dylanaraps",
    "Bounty plss, this time - Knight_yagami",
    "Hacking in a nutshell",
    "In the world of programming, curiosity is the compass. - Ashokcpg",
    "Compile once, Hack forever - PwnWriter",
    "I want my system to hack!! wOoO",
    "Hello everyone, this is your daily dose for Bounty - PwnWriter",
    "Why no work?, Bro RTFM :/",
    "Hey PwnWriter, Your readmes are soo beautiful - plan9boys",
];

// Not using rand crate anymore
// https://users.rust-lang.org/t/cheap-random-number-generator-with-std/90589/6

fn generate_random_number() -> usize {
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_micros();

    (current_time % SPLASHES.len() as u128) as usize
}

pub fn show_splashes() -> String {
    let rng = generate_random_number();

    let kanha_version = env!("CARGO_PKG_VERSION");

    let logo = format!(
        r#"
    ╦╔═╔═╗╔╗╔╦ ╦╔═╗
    ╠╩╗╠═╣║║║╠═╣╠═╣
    ╩ ╩╩ ╩╝╚╝╩ ╩╩ ╩ v{}
"#,
        kanha_version,
    )
    .bold()
    .purple();
    let splash = SPLASHES[rng].italic();
    format!("{logo} {splash}")
}
