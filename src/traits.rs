pub trait DiziJsonCommand<'a>: serde::Deserialize<'a> + serde::Serialize {
    fn path() -> &'static str;
}
