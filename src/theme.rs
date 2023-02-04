use owo_colors::Style;

pub struct Theme {
    pub pwd: Style,
    pub brackets: Style,
    pub submodule: Style,
    pub branch: Style,
    pub sep: Style,
    pub operation: Style,
    pub ratio: Style,
}

impl Theme {
    pub fn get() -> Self {
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
