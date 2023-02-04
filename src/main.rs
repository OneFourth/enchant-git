use owo_colors::OwoColorize;

use self::git::Git;
use self::theme::Theme;

mod git;
mod stat;
mod theme;
mod utility;

fn main() {
    let t = Theme::get();
    let pwd = std::env::current_dir().unwrap();
    let g = Git::get();

    println!("{}{}", pwd.display().style(t.pwd), g.portion(&t));
}
