use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use std::{future::Future, time::Duration};
use tokio::{select, time::interval};

const TICK_STRINGS: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

#[derive(Debug)]
pub struct Spinner {
    inner: ProgressBar,
}

impl Spinner {
    pub fn new() -> Self {
        Spinner {
            inner: ProgressBar::with_draw_target(!0, ProgressDrawTarget::stdout()).with_style(
                ProgressStyle::default_spinner()
                    .tick_strings(TICK_STRINGS)
                    .template(&Self::format_tpl("blue")),
            ),
        }
    }

    pub fn set_color(&self, color: &'static str) -> &Self {
        self.inner.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(TICK_STRINGS)
                .template(&Self::format_tpl(color)),
        );
        self
    }

    pub fn set_message(&self, message: &'static str) -> &Self {
        self.inner.set_message(message);
        self
    }

    pub fn println<T>(&self, message: T) -> &Self
    where
        T: AsRef<str>,
    {
        self.inner.println(message);
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
                self.inner.tick();
            }
        };
        select! {
            r = infinity => {r},
            r = finish_fn => {r}
        }
    }

    #[inline]
    fn format_tpl(color: &'static str) -> String {
        format!("{{spinner:.{color}}} {{msg}}", color = color)
    }
}
