use super::{AppCommand, Command};

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ChangeDirectory(p) => write!(f, "{} {:?}", self.command(), p),
            Self::CommandLine(s, p) => write!(f, "{} {} {}", self.command(), s, p),
            Self::CursorMoveUp(i) => write!(f, "{} {}", self.command(), i),
            Self::CursorMoveDown(i) => write!(f, "{} {}", self.command(), i),

            Self::SearchGlob(s) => write!(f, "{} {}", self.command(), s),
            Self::SearchString(s) => write!(f, "{} {}", self.command(), s),
            Self::SelectFiles(pattern, options) => {
                write!(f, "{} {} {}", self.command(), pattern, options)
            }
            Self::Sort(t) => write!(f, "{} {}", self.command(), t),
            Self::ServerRequest(request) => write!(f, "{} {}", self.command(), request.api_path()),
            _ => write!(f, "{}", self.command()),
        }
    }
}
