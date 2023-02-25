### restapi

#### want to display Vec<Album<T>>, need to create a wrapper struct named Albums

```rust
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
```

####
```shell
make alpine
docker run -it --rm -p 8080:8080 rust-alpine
```