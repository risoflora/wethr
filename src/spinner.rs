use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use std::{
    fmt::{Display, Formatter, Result},
    future::Future,
    time::Duration,
};
use tokio::{select, time::interval};

const TICK_STRINGS: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SpinnerColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Display for SpinnerColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use SpinnerColor::*;
        let color = match &self {
            Black => "black",
            Red => "red",
            Green => "green",
            Yellow => "yellow",
            Blue => "blue",
            Magenta => "magenta",
            Cyan => "cyan",
            White => "white",
        };
        write!(f, "{}", color)
    }
}

#[derive(Debug)]
pub struct Spinner {
    progress_bar: ProgressBar,
    silent: bool,
}

impl Spinner {
    pub fn new() -> Self {
        Spinner {
            progress_bar: ProgressBar::with_draw_target(!0, ProgressDrawTarget::stdout()),
            silent: false,
        }
    }

    pub fn set_silent(mut self, silent: bool) -> Self {
        self.silent = silent;
        if !self.silent {
            self.progress_bar
                .set_style(ProgressStyle::default_spinner().tick_strings(TICK_STRINGS));
        }
        self
    }

    pub fn set_color(&self, color: SpinnerColor) -> &Self {
        if !self.silent {
            self.progress_bar.set_style(
                ProgressStyle::default_spinner()
                    .tick_strings(TICK_STRINGS)
                    .template(&Self::format_tpl(color)),
            );
        }
        self
    }

    pub fn set_message(&self, message: &'static str) -> &Self {
        if !self.silent {
            self.progress_bar.set_message(message);
        }
        self
    }

    pub fn print_message<T>(&self, message: T) -> &Self
    where
        T: AsRef<str> + Display,
    {
        if self.silent {
            println!("{}", message)
        } else {
            self.progress_bar.println(message);
        }
        self
    }

    pub async fn run<F, T>(&self, finish_fn: F) -> T
    where
        F: Future<Output = T>,
    {
        let infinity = async {
            let mut intv = interval(Duration::from_millis(120));
            loop {
                intv.tick().await;
                if !self.silent {
                    self.progress_bar.tick();
                }
            }
        };
        select! {
            r = infinity => {r},
            r = finish_fn => {r}
        }
    }

    #[inline]
    fn format_tpl(color: SpinnerColor) -> String {
        format!("{{spinner:.{color}}} {{msg}}", color = color)
    }
}

#[cfg(test)]
mod tests {
    use super::Spinner;

    #[tokio::test]
    async fn spinner_all() {
        let mut ran = false;
        Spinner::new()
            .run(async {
                ran = true;
            })
            .await;
        assert!(ran);
    }
}
