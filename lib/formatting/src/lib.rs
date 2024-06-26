use serror::Serror;

pub fn muted(content: impl std::fmt::Display) -> String {
  format!("<span class=\"text-muted-foreground\">{content}</span>")
}

pub fn bold(content: impl std::fmt::Display) -> String {
  format!("<span class=\"font-bold\">{content}</span>")
}

pub fn colored(
  content: impl std::fmt::Display,
  color: Color,
) -> String {
  format!("<span class=\"{color}\">{content}</span>")
}

pub enum Color {
  Red,
  Green,
  Blue,
}

impl std::fmt::Display for Color {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Color::Red => f.write_str("text-red-700 dark:text-red-400"),
      Color::Green => {
        f.write_str("text-green-700 dark:text-green-400")
      }
      Color::Blue => f.write_str("text-blue-700 dark:text-blue-400"),
    }
  }
}

pub fn format_serror(Serror { error, trace }: &Serror) -> String {
  let trace = (!trace.is_empty())
    .then(|| {
      let mut out = format!("\n\n{}:", muted("TRACE"));

      for (i, msg) in trace.iter().enumerate() {
        out.push_str(&format!("\n\t{}: {msg}", muted(i + 1)));
      }

      out
    })
    .unwrap_or_default();
  format!("{}: {error}{trace}", colored("ERROR", Color::Red))
}
