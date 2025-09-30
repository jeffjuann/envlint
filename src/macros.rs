
#[macro_export]
macro_rules! emit_error 
{
  ($($arg:tt)*) => 
  {
    println!("{}{} {}", 
      crate::utils::colorize(colored::Color::Red, "error"),
      crate::utils::colorize(colored::Color::BrightWhite, ":"),
      format!($($arg)*)
    );
    std::process::exit(1);
  };
}

#[macro_export]
macro_rules! emit_warn 
{
  ($($arg:tt)*) => 
  {
    println!("{}{} {}", 
      crate::utils::colorize(colored::Color::Yellow, "warn"),
      crate::utils::colorize(colored::Color::BrightWhite, ":"),
      format!($($arg)*)
    );
  };
}

#[macro_export]
macro_rules! emit_info 
{
  ($($arg:tt)*) => 
  {
    println!("{}{} {}",
      crate::utils::colorize(colored::Color::Blue, "info"),
      crate::utils::colorize(colored::Color::BrightWhite, ":"),
      format!($($arg)*)
    );
  };
}

#[macro_export]
macro_rules! emit_success 
{
  ($($arg:tt)*) => 
  {
    println!("{}{} {}",
      crate::utils::colorize(colored::Color::Green, "success"),
      crate::utils::colorize(colored::Color::BrightWhite, ":"),
      format!($($arg)*)
    );
  };
}