use std::path::PathBuf;

use owo_colors::{OwoColorize, Style, StyledList};

struct Theme {
    pwd: Style,
    brackets: Style,
    branch: Style,
    sep: Style,
    operation: Style,
    ratio: Style,
}

impl Theme {
    fn get() -> Self {
        Self {
            pwd: Style::new().bright_green(),
            brackets: Style::new().cyan(),
            branch: Style::new().bright_cyan(),
            sep: Style::new().bright_black(),
            operation: Style::new().bright_red(),
            ratio: Style::new().bright_yellow(),
        }
    }
}

struct Git {
    root: Option<PathBuf>,
}

struct Stat {
    operation: &'static str,
    ratio: String,
    _branch: String,
}

impl Git {
    fn get() -> Self {
        let pwd = std::env::current_dir().unwrap();
        let root = pwd.ancestors().map(|p| p.join(".git")).find(|p| p.exists());

        Self { root }
    }

    fn branch(&self) -> String {
        let mut s = String::new();

        if let Some(root) = &self.root {
            let head = root.join("HEAD");
            if head.exists() {
                let contents = std::fs::read_to_string(head).unwrap();
                const REFS: &str = "ref: refs/heads/";
                if contents.contains(REFS) {
                    s = contents.trim().replace(REFS, "");
                } else {
                    s = format!("HEAD-{}", &contents[..8]);
                }
            }
        }

        s
    }

    fn stat(&self) -> Stat {
        let mut operation = "";
        let mut step = String::new();
        let mut total = String::new();
        let mut branch = String::new();

        if let Some(root) = &self.root {
            let rebase_merge = root.join("rebase-merge");
            if rebase_merge.exists() {
                branch = std::fs::read_to_string(rebase_merge.join("head-name")).unwrap();
                step = std::fs::read_to_string(rebase_merge.join("msgnum")).unwrap();
                total = std::fs::read_to_string(rebase_merge.join("end")).unwrap();

                if rebase_merge.join("").exists() {
                    operation = "REBASE-i";
                } else {
                    operation = "REBASE-m";
                }
            } else {
                let rebase_apply = root.join("rebase-apply");
                if rebase_apply.exists() {
                    step = std::fs::read_to_string(rebase_apply.join("next")).unwrap();
                    total = std::fs::read_to_string(rebase_apply.join("last")).unwrap();

                    if rebase_apply.join("rebasing").exists() {
                        branch = std::fs::read_to_string(rebase_apply.join("head-name")).unwrap();
                        operation = "REBASE";
                    } else if rebase_apply.join("applying").exists() {
                        operation = "AM";
                    } else {
                        operation = "AM/REBASE";
                    }
                } else if root.join("MERGE_HEAD").exists() {
                    operation = "MERGING";
                } else if root.join("CHERRY_PICK_HEAD").exists() {
                    operation = "CHERRY-PICKING";
                } else if root.join("REVERT_HEAD").exists() {
                    operation = "REVERTING";
                } else if root.join("BISECT_LOG").exists() {
                    operation = "BISECTING";
                }
            }
        }

        let mut ratio = String::new();
        if !step.trim().is_empty() {
            ratio = format!(" {}/{}", step.trim(), total.trim());
        }

        Stat {
            operation,
            ratio,
            _branch: branch,
        }
    }
}

fn git_portion(t: &Theme) -> String {
    let git = Git::get();

    if git.root.is_some() {
        let git_stat = git.stat();
        format!(
            " {}",
            StyledList::from([
                t.brackets.style("["),
                t.branch.style(&git.branch()),
                t.sep.style(if git_stat.operation.is_empty() {
                    ""
                } else {
                    " | "
                }),
                t.operation.style(git_stat.operation),
                t.ratio.style(&git_stat.ratio),
                t.brackets.style("]"),
            ])
        )
    } else {
        "".to_string()
    }
}

fn build_prompt() -> String {
    let t = Theme::get();

    format!(
        "{}{}",
        std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .style(t.pwd),
        git_portion(&t),
    )
}

fn main() {
    println!("{}", build_prompt());
}
