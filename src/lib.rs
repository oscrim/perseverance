/// Used to persist a struct
pub trait Persist: std::marker::Sized {
    /// The error type thaat will be returned
    type Error: std::fmt::Debug;

    /// The config that contains the information needed to persist the data
    type Config;

    /// Setup and create the empty struct and save the config
    fn setup(config: Self::Config) -> Self;

    /// Save the data and make it persist, a duration of zero should
    /// save it once, a nonzero duration should persist the data on an
    /// interval of the duration
    fn persist(&self, time: std::time::Duration) -> Result<(), Self::Error>;

    /// Load in the data if any exists
    fn load(&self) -> Result<(), Self::Error>;
}

#[cfg(test)]
mod tests {
    use std::cmp::{self, Eq};
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex, RwLock};
    use std::time::Duration;

    use serde::{Deserialize, Serialize};
    use serde_json;

    use crate::Persist;

    #[test]
    fn test_trait() {
        let mut path = PathBuf::default();
        path.push("ZERO.json");

        let test = Test::setup(path.clone());
        *test.data.write().unwrap() = 0;
        test.persist(Duration::ZERO).unwrap();

        let test2 = Test::setup(path.clone());
        test2.load().unwrap();
        assert_eq!(test, test2);

        *test2.data.write().unwrap() = 5;
        test2.persist(Duration::ZERO).unwrap();

        let test3 = Test::setup(path);
        test3.load().unwrap();
        assert_ne!(test, test3);
        assert_eq!(test2, test3);
    }

    #[test]
    fn test_trait_interval() {
        let mut path = PathBuf::default();
        path.push("NON-ZERO.json");

        let test = Test::setup(path.clone());
        let clone = test.clone();
        std::thread::spawn(move || persist_interval(clone));

        *test.data.write().unwrap() = 5;
        std::thread::sleep(Duration::from_millis(52));
        let test2 = Test::setup(path.clone());
        load(&test2);
        assert_eq!(test, test2);

        *test.data.write().unwrap() = 0;
        let test3 = Test::setup(path.clone());
        load(&test3);
        assert_ne!(test, test3);

        std::thread::sleep(Duration::from_millis(50));
        let test4 = Test::setup(path);
        load(&test4);
        assert_eq!(test, test4);
    }

    // The following two functions will work with any struct that implements Persist
    fn persist_interval<T>(persist: T)
    where
        T: Persist + ?std::marker::Sized,
    {
        persist.persist(Duration::from_millis(50)).unwrap()
    }

    fn load<T>(persist: &T)
    where
        T: Persist + ?std::marker::Sized,
    {
        persist.load().unwrap();
    }

    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    pub struct Test {
        data: Arc<RwLock<u8>>,

        persiverance: Arc<Mutex<bool>>,
        config: Arc<RwLock<PathBuf>>,
    }

    impl Persist for Test {
        type Error = String;

        type Config = PathBuf;

        fn persist(&self, time: Duration) -> Result<(), Self::Error> {
            let once = if time.is_zero() { true } else { false };

            let path = { self.config.read().unwrap().clone() };

            loop {
                let to_write = serde_json::to_string(&*self.data.read().unwrap()).unwrap();

                let mut file = match OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&path)
                {
                    Ok(file) => file,
                    Err(e) => {
                        let msg = format!("{:?}: {}", path, e.to_string());
                        return Err(msg);
                    }
                };

                file.write(to_write.as_bytes()).unwrap();

                if once {
                    break;
                } else {
                    std::thread::sleep(time);
                }
            }
            Ok(())
        }

        fn load(&self) -> Result<(), Self::Error> {
            let mut buf = String::new();
            let buf = match OpenOptions::new()
                .write(true)
                .create(true)
                .read(true)
                .open(self.config.read().unwrap().clone())
            {
                Ok(mut file) => match file.read_to_string(&mut buf) {
                    Ok(_) => buf,
                    Err(e) => return Err(e.to_string()),
                },
                Err(e) => return Err(e.to_string()),
            };

            let data: u8 = match serde_json::from_str(&buf) {
                Ok(data) => data,
                Err(e) => return Err(e.to_string()),
            };

            *self.data.write().unwrap() = data;
            Ok(())
        }

        fn setup(config: Self::Config) -> Self {
            let t = Test::default();
            *t.config.write().unwrap() = config;
            t
        }
    }

    impl cmp::PartialEq for Test {
        fn eq(&self, other: &Self) -> bool {
            println!(
                "{:?} == {:?}",
                *self.data.read().unwrap(),
                *other.data.read().unwrap()
            );
            *self.data.read().unwrap() == *other.data.read().unwrap()
        }
    }

    impl Eq for Test {}
}
