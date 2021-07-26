use std::io::{self, prelude::*};
use std::ops::{Deref, DerefMut};
use std::{fs::OpenOptions, path::PathBuf};

use serde::{de::DeserializeOwned, Serialize};

use crate::Persist;

#[derive(Debug)]
pub struct JsonPreserve<T>
where
    T: Serialize + DeserializeOwned,
{
    pub data: T,
    config: PathBuf,
}

impl<T> JsonPreserve<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn new(data: T, config: PathBuf) -> Self {
        JsonPreserve { data, config }
    }
}

impl<T> Persist for JsonPreserve<T>
where
    T: Serialize + DeserializeOwned,
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

impl<T> Deref for JsonPreserve<T>
where
    T: Serialize + DeserializeOwned,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for JsonPreserve<T>
where
    T: Serialize + DeserializeOwned,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
