mod day;
pub mod template;

pub use day::*;

pub fn parse_space_separated<T>(s: &str) -> Vec<T>
where
    T: Default + Clone + std::str::FromStr,
{
    s.split_whitespace()
        .filter_map(|word| word.parse::<T>().ok())
        .collect()
}
