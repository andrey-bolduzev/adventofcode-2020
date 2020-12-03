use std::path::Path;
use std::str::FromStr;

use std::io::BufRead;

pub fn parse<T>(path: &Path) -> std::io::Result<impl Iterator<Item = T>>
where
    T: FromStr,
    T: std::fmt::Debug,
    T::Err: std::fmt::Debug,
{
    let file = std::fs::File::open(&path)?;
    Ok(std::io::BufReader::new(file)
        .lines()
        .map(|line| T::from_str(&line.unwrap()).unwrap()))
}

/// adaptor which plugs into parse, splitting comma-separated items from the line
///
/// This can be flattened or consumed by line, as required
pub struct CommaSep<T>(Vec<T>);

impl<T> FromStr for CommaSep<T>
where
    T: FromStr,
{
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .map(CommaSep)
    }
}

impl<T> IntoIterator for CommaSep<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
