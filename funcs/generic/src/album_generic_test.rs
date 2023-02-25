use std::fmt;
use std::fmt::{Display, Formatter};

struct Album<T>
{
    pub title: T,
    pub artist: T,
}

struct Albums<T>(pub Vec<Album<T>>);

impl<T: Display> Display for Albums<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, album| {
            result.and_then(|_| writeln!(f, "artist: {}  artist:{}", album.artist, album.artist))
        })
    }
}

#[test]
fn test_generic_struct() {
    let albums = Albums::<String>(vec![
        Album {
            title: "Sgt. Pepper's Lonely Hearts Club Band".into(),
            artist: "The Beatles".into(),
        },
        Album {
            title: "Dark Side of the Moon".into(),
            artist: "Pink Floyd".into(),
        },
    ]);

    println!("{}", albums);
}