use colored::Colorize;
use rand::Rng;
pub static SPLASHES: &[&str] = &[
    "There are reasons to use rust. - PwnWriter",
    "whatsoever a man soweth, that shall he also reap. - Dylanaraps",
    "Bounty plss, this time - Knight_yagami",
    "Hacking in a nutshell",
    "Compile once, Hack forever - PwnWriter",
    "I want my system to hack!! woo",
    "Hello everyone, this is your daily dose for Bounty - PwnWriter",
    "Why no work?, Bro RTFM :/",
];

pub fn show_splashes() -> String {
    let rng = rand::thread_rng().gen_range(0..SPLASHES.len());
    let logo = r"
    ╦╔═╔═╗╔╗╔╦ ╦╔═╗
    ╠╩╗╠═╣║║║╠═╣╠═╣
    ╩ ╩╩ ╩╝╚╝╩ ╩╩ ╩ v.0.1.0
"
    .bold()
    .purple();
    let splash = SPLASHES[rng].italic();
    format!("{logo} {splash}")
}
