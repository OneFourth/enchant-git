use std::path::{Path, PathBuf};

use owo_colors::{OwoColorize, Style, StyledList};

struct Theme {
    pwd: Style,
    brackets: Style,
    submodule: Style,
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
            submodule: Style::new().bright_red(),
            branch: Style::new().bright_cyan(),
            sep: Style::new().bright_black(),
            operation: Style::new().bright_red(),
            ratio: Style::new().bright_yellow(),
        }
    }
}

struct Git {
    root: Vec<PathBuf>,
}

#[derive(Default)]
struct Stat {
    operation: &'static str,
    ratio: String,
    _branch: String,
}

impl Stat {
    fn rebase_merge(&mut self, root: &Path) -> Result<bool> {
        let rebase_merge_folder = root.join("rebase-merge");
        if rebase_merge_folder.exists() {
            self._branch = read_maybe_missing_file(rebase_merge_folder.join("head-name"))?;
            let step = read_maybe_missing_file(rebase_merge_folder.join("msgnum"))?;
            let total = read_maybe_missing_file(rebase_merge_folder.join("end"))?;

            self.set_ratio(&step, &total);

            if rebase_merge_folder.join("interactive").exists() {
                self.operation = "REBASE-i";
            } else {
                self.operation = "REBASE-m";
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn rebase_apply(&mut self, root: &Path) -> Result<bool> {
        let rebase_apply_folder = root.join("rebase-apply");
        if rebase_apply_folder.exists() {
            let step = read_maybe_missing_file(rebase_apply_folder.join("next"))?;
            let total = read_maybe_missing_file(rebase_apply_folder.join("last"))?;

            self.set_ratio(&step, &total);

            if rebase_apply_folder.join("rebasing").exists() {
                self._branch = read_maybe_missing_file(rebase_apply_folder.join("head-name"))?;
                self.operation = "REBASE";
            } else if rebase_apply_folder.join("applying").exists() {
                self.operation = "AM";
            } else {
                self.operation = "AM/REBASE";
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn handle_non_rebase(&mut self, root: &Path) {
        if root.join("MERGE_HEAD").exists() {
            self.operation = "MERGING";
        } else if root.join("CHERRY_PICK_HEAD").exists() {
            self.operation = "CHERRY-PICKING";
        } else if root.join("REVERT_HEAD").exists() {
            self.operation = "REVERTING";
        } else if root.join("BISECT_LOG").exists() {
            self.operation = "BISECTING";
        }
    }

    fn set_ratio(&mut self, step: &str, total: &str) {
        if !step.trim().is_empty() {
            self.ratio = format!(" {}/{}", step.trim(), total.trim());
        }
    }
}

#[derive(Debug)]
enum EnchantError {
    MissingFile(PathBuf),
}

impl std::fmt::Display for EnchantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnchantError::MissingFile(p) => write!(f, "Missing file: {}", p.display()),
        }
    }
}

impl std::error::Error for EnchantError {}

type Result<T> = std::result::Result<T, EnchantError>;

fn read_maybe_missing_file(p: PathBuf) -> Result<String> {
    if p.exists() {
        Ok(std::fs::read_to_string(p).unwrap())
    } else {
        Err(EnchantError::MissingFile(p))
    }
}

impl Git {
    fn get() -> Self {
        let pwd = std::env::current_dir().unwrap();
        let root = pwd
            .ancestors()
            .filter_map(|p| {
                let g = p.join(".git");
                if g.exists() {
                    if g.is_file() {
                        let gitdir: PathBuf = std::fs::read_to_string(g)
                            .unwrap()
                            .trim()
                            .strip_prefix("gitdir: ")
                            .unwrap()
                            .into();
                        Some(p.join(gitdir))
                    } else {
                        Some(g)
                    }
                } else {
                    None
                }
            })
            .collect();

        Self { root }
    }

    fn root(&self) -> Option<&PathBuf> {
        self.root.first()
    }

    fn branch(&self) -> String {
        let mut s = String::new();

        if let Some(root) = self.root() {
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

    fn stat(&self) -> Result<Stat> {
        let mut stat = Stat::default();

        if let Some(root) = self.root() {
            if !stat.rebase_merge(&root)? {
                if !stat.rebase_apply(&root)? {
                    stat.handle_non_rebase(&root);
                }
            }
        }

        Ok(stat)
    }
}

fn conditional(value: &str, cond: bool) -> &str {
    if cond {
        value
    } else {
        ""
    }
}

fn git_portion(t: &Theme) -> String {
    let git = Git::get();

    if !git.root.is_empty() {
        let git_stat = git.stat().unwrap();
        let in_submodule = git.root.len() > 1;
        format!(
            " {}",
            StyledList::from([
                t.brackets.style("["),
                t.submodule.style(conditional("SUBMODULE", in_submodule)),
                t.sep.style(conditional(" | ", in_submodule)),
                t.branch.style(&git.branch()),
                t.sep
                    .style(conditional(" | ", !git_stat.operation.is_empty())),
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
