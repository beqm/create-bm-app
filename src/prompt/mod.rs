use inquire::{MultiSelect, Text};
use std::{env, path::PathBuf};

const DEFAULT_DIR: &str = "./bm-app";

pub struct Inputs<'a> {
    pub directory: PathBuf,
    pub features: Vec<&'a str>,
}

pub fn get_inputs<'a>() -> Inputs<'a> {
    let dir = project_location();
    let feats = project_features();
    let inputs = Inputs {
        directory: dir,
        features: feats,
    };
    return inputs;
}

pub fn project_location() -> PathBuf {
    let dir = Text::new("Choose your new project directory:").with_default(DEFAULT_DIR).prompt();

    let mut current_dir = env::current_dir().unwrap();
    current_dir.push(dir.unwrap());
    return current_dir;
}

pub fn project_features<'a>() -> Vec<&'a str> {
    let options = vec!["Prisma", "Kysely", "TailwindCSS", "Lucia"];
    let features = MultiSelect::new("Choose your features:", options).prompt();
    return features.unwrap();
}
