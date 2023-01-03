use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};

pub const PERMITTED_CHARS: [&str; 64] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L",
    "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "0", "1", "2", "3", "4",
    "5", "6", "7", "8", "9", "$", "_",
];

pub fn create_progress_bar(length: u64) -> ProgressBar {
    let progress_bar = ProgressBar::new(length)
        .with_prefix("Progress: ")
        .with_style(
            ProgressStyle::with_template("{prefix:.bold}▕{bar:.green}({eta})")
                .unwrap()
                .progress_chars("█▉▊▋▌▍▎▏  "),
        );
    let draw_target = ProgressDrawTarget::stdout_with_hz(3);
    progress_bar.set_draw_target(draw_target);

    progress_bar
}
