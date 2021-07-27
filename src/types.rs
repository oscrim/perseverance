use std::io::{self, prelude::*};
use std::ops::{Deref, DerefMut};
use std::{fs::OpenOptions, path::PathBuf};

use serde::{de::DeserializeOwned, Serialize};

use crate::Persist;

/// Generic struct that implements Persist and is configured to save to file
#[derive(Debug, Clone, Default)]
pub struct JsonPersist<T>
where
    T: Serialize + DeserializeOwned + Default + Clone,
{
    pub data: T,
    pub config: PathBuf,
}

impl<T> JsonPersist<T>
where
    T: Serialize + DeserializeOwned + Default + Clone,
{
    pub fn new(data: T, config: PathBuf) -> Self {
        JsonPersist { data, config }
    }
}

impl<T> Persist for JsonPersist<T>
where
    T: Serialize + DeserializeOwned + Default + Clone,
{
    type Error = io::Error;

    type Config = PathBuf;

    fn persist(&self) -> Result<(), Self::Error> {
        let to_write = match serde_json::to_string(&self.data) {
            Ok(o) => o,
            Err(e) => return Err(io::Error::from(e)),
        };

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.config)?;

        file.write(to_write.as_bytes())?;
        Ok(())
    }

    fn load(&mut self) -> Result<(), Self::Error> {
        let mut buf = String::new();
        let buf = match OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(self.config.clone())
        {
            Ok(mut file) => match file.read_to_string(&mut buf) {
                Ok(_) => buf,
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        };

        let data: T = match serde_json::from_str(&buf) {
            Ok(data) => data,
            Err(e) => return Err(io::Error::from(e)),
        };

        self.data = data;
        Ok(())
    }
}

impl<T> Deref for JsonPersist<T>
where
    T: Serialize + DeserializeOwned + Default + Clone,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for JsonPersist<T>
where
    T: Serialize + DeserializeOwned + Default + Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
