use std::path::PathBuf;

use owo_colors::StyledList;

use crate::stat::Stat;
use crate::theme::Theme;
use crate::utility::conditional;

pub struct Git {
    root: Vec<PathBuf>,
}

impl Git {
    pub fn get() -> Self {
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

    pub fn portion(&self, t: &Theme) -> String {
        if !self.root.is_empty() {
            let git_stat = Stat::get(self.root().unwrap()).unwrap();
            let in_submodule = self.root.len() > 1;
            format!(
                " {}",
                StyledList::from([
                    t.brackets.style("["),
                    t.submodule.style(conditional("SUBMODULE", in_submodule)),
                    t.sep.style(conditional(" | ", in_submodule)),
                    t.branch.style(&self.branch()),
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
}
