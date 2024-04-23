pub trait Config {
    fn read_from_file() -> Option<impl Config> where Self: Sized;
}

