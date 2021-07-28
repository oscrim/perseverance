#[cfg(feature = "types")]
pub mod types;
#[cfg(feature = "types")]
pub use types::JsonPersist;

/// Used to persist a struct
pub trait Persist: std::marker::Sized {
    /// The error type that will be returned
    type Error;

    /// The config that contains the information needed to persist the data
    type Config;

    /// Save the data and make it persist
    fn persist(&self) -> Result<(), Self::Error>;

    /// Load in the data if any exists
    fn load(&mut self) -> Result<(), Self::Error>;
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::path::PathBuf;

    #[cfg(feature = "types")]
    use crate::types::JsonPersist;
    use crate::Persist;

    #[test]
    #[cfg(feature = "types")]
    fn generic_struct() {
        let path = PathBuf::from("generic.json");

        let mut persist1: JsonPersist<String> =
            JsonPersist::new("Hello World!".into(), path.clone());
        persist(&persist1);

        let mut persist2: JsonPersist<String> = JsonPersist::new("".into(), path.clone());
        load(&mut persist2);
        assert_eq!(*persist1, *persist2);

        *persist1 = "Bye!".into();
        persist(&persist1);

        let mut persist3: JsonPersist<String> = JsonPersist::new("".into(), path.clone());
        load(&mut persist3);
        assert_eq!(*persist1, *persist3);
        assert_ne!(*persist2, *persist3);
    }

    #[test]
    fn test_struct() {
        let mut persist1 = TextFile::new(PathBuf::from("test.json"));

        persist1.data = "Hello World!".into();
        persist(&persist1);

        let mut persist2 = TextFile::new(PathBuf::from("test.json")); // Setup struct
        load(&mut persist2);
        assert_eq!(persist1.data, persist2.data);

        persist1.data = "Bye!".into();
        persist(&persist1);

        let mut persist3 = TextFile::new(PathBuf::from("test.json")); // Setup struct
        load(&mut persist3);
        assert_eq!(persist1.data, persist3.data);
        assert_ne!(persist2.data, persist3.data);
    }

    fn load<T>(persist: &mut T)
    where
        T: Persist,
        T::Error: Debug,
    {
        persist.load().unwrap();
    }

    fn persist<T>(persist: &T)
    where
        T: Persist,
        T::Error: Debug,
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
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&self.config)?;

            file.write(self.data.as_bytes())?;
            Ok(())
        }

        fn load(&mut self) -> Result<(), Self::Error> {
            let mut buf = String::new();
            let mut file = OpenOptions::new().read(true).open(&self.config)?;
            file.read_to_string(&mut buf)?;

            self.data = buf;
            Ok(())
        }
    }
}
