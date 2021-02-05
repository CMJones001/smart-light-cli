//! Mananger for complex scene commands
use crate::common::{GetSig, Lamp};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct SceneList {
    pub names: Vec<String>,
}

impl SceneList {
    pub fn new() -> SceneList {
        let names = vec![];
        SceneList { names }
    }
    pub fn from_scene<S: Scene>(s: &S) -> SceneList {
        let names = s.get_scenes();
        SceneList { names }
    }
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<SceneList, io::Error> {
        let file = File::open(path)?;
        let json = serde_json::from_reader(&file)?;
        Ok(json)
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) {
        let file = File::create(path).expect("Unable to open file");
        serde_json::to_writer(&file, &self).expect("Unable to save name list to file");
    }
}

pub trait Scene: Lamp {
    //! Read the list of scenes from the api
    fn get_scenes(&self) -> Vec<String> {
        let signal = GetSig::Scene;
        let scene_text = self.get(signal);
        serde_json::from_str(&scene_text).unwrap()
    }
}
