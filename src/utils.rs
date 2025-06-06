use colored::{Color, Colorize};

pub fn colorize(color: Color, message: &str) -> String
{
  match color
  {
    Color::Red => return format!("{}", Colorize::bold(Colorize::red(message))),
    Color::Yellow => return format!("{}", Colorize::bold(Colorize::yellow(message))),
    Color::Blue => return format!("{}", Colorize::bold(Colorize::blue(message))),
    Color::Green => return format!("{}", Colorize::bold(Colorize::green(message))),
    Color::BrightWhite => return format!("{}", Colorize::bold(Colorize::bright_white(message))),
    _ => return message.to_string(),
  }
}