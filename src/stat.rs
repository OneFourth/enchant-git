use std::path::Path;

use crate::utility::{read_maybe_missing_file, Result};

#[derive(Default)]
pub struct Stat {
    pub operation: &'static str,
    pub ratio: String,
    _branch: String,
}

impl Stat {
    pub fn get(root: &Path) -> Result<Self> {
        let mut stat = Self::default();
        if !stat.rebase_merge(root)? && !stat.rebase_apply(root)? {
            stat.handle_non_rebase(root);
        }

        Ok(stat)
    }

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
