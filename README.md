# perseverance

A rust library for simplifying the process of maintaining persistency between runtimes.

## Usage

To use the generic type that implements Persist use the feature **types**.

To use perseverance in async the **async** feature can be used which will enable the
PersistAsync trait, this will also enable the #[async_trait] macro which is re-exported
from the [async_trait](https://crates.io/crates/async-trait) crate.

Here is an example implementation for having simple persistency for a textfile that can,
either be saved directly or on a set interval.

The type of storage will not matter as any config needed will be inside the Config type,
this means that it should be possible to for example switch between saving to file and
saving to a database relatively easily.

### Example

```rust

    fn using_struct() {
        let mut persistent_text = TextFile::new(PathBuf::from("test.json")); // Setup struct
        load(&mut persistent_text);

        persistent_text.data = "Hello World!".into(); // Change Data
        persist(&persistent_text); // Persist

        // Do other work...

        persistent_text.data = "Bye!".into();
        persist(&persistent_text);
    }

    fn using_generic_struct() {
        let path = PathBuf::from("generic.json");

        let mut persistent_text: JsonPreserve<String> = JsonPreserve::new("".into(), path);
        load(&mut persistent_text);

        // Do other work...

        *persistent_text = "Bye!".into();
        persist(&persistent_text);
    }

    fn load<T>(persist: &mut T)
    where
        T: Persist,
    {
        persist.load().unwrap();
    }

    fn persist<T>(persist: &T)
    where
        T: Persist,
    {
        persist.persist().unwrap();
    }

    #[derive(Debug)]
    pub struct TextFile {
        pub data: String,
        config: PathBuf,
    }

    impl TextFile {
        fn new(config: PathBuf) -> Self {
            TextFile {
                data: "".into(),
                config,
            }
        }
    }

    impl Persist for TextFile {
        type Error = std::io::Error;

        type Config = PathBuf;

        fn persist(&self) -> Result<(), Self::Error> {
            // Save to file
        }

        fn load(&mut self) -> Result<(), Self::Error> {
            // Load from file
        }
    }
```
