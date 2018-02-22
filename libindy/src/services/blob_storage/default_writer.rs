extern crate indy_crypto;

use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::Write;

use base64;

use super::{Writer, WriterType};
use errors::common::CommonError;
use utils::environment::EnvironmentUtils;

use self::indy_crypto::utils::json::*;

pub struct DefaultWriter {
    base_dir: PathBuf,
    uri_pattern: String,
    file: File,
}

#[derive(Serialize, Deserialize)]
struct DefaultWriterConfig {
    base_dir: String,
    uri_pattern: String,
}

impl<'a> JsonDecodable<'a> for DefaultWriterConfig {}

impl WriterType for DefaultWriterType {
    fn create(&self, config: &str) -> Result<Box<Writer>, CommonError>  {
        let config: DefaultWriterConfig = DefaultWriterConfig::from_json(config)
            .map_err(map_err_trace!())?;
        let path = PathBuf::from(config.base_dir);
        let file = File::create(EnvironmentUtils::tmp_file_path("def_storage_tmp")) //FIXME
            .map_err(map_err_trace!())?;

        Ok(Box::new(DefaultWriter {
            base_dir: path,
            uri_pattern: config.uri_pattern,
            file,
        }))
    }
}

impl Writer for DefaultWriter {
    fn append(&mut self, bytes: &[u8]) -> Result<usize, CommonError> {
        Ok(self.file.write(bytes)?)
    }

    fn finalize(&mut self, hash: &[u8]) -> Result<String, CommonError> {
        self.file.flush()?;
        let mut path = self.base_dir.clone();
        path.push(base64::encode(hash));

        fs::rename(EnvironmentUtils::tmp_file_path("def_storage_tmp"), &path)?; //FIXME

        Ok(path.to_str().unwrap().to_owned())
    }
}

pub struct DefaultWriterType {}

impl DefaultWriterType {
    pub fn new() -> Self {
        DefaultWriterType {}
    }
}
