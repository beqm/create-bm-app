use crate::prompt::Inputs;
use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;

const BASE_DIRECTORY: &str = "templates/base/";
const BASE_FEATURES_DIRECTORY: &str = "templates/features/";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    name: Option<String>,
    version: Option<String>,
    r#type: Option<String>,
    scripts: Option<HashMap<String, Option<String>>>,
    dependencies: Option<HashMap<String, Option<String>>>,
    dev_dependencies: Option<HashMap<String, Option<String>>>,
}

pub fn create(inputs: Inputs) -> Result<()> {
    create_dir(&inputs)?;
    build(inputs)?;

    println!("Your project is ready!");
    Ok(())
}

pub fn create_dir(inputs: &Inputs) -> Result<()> {
    if inputs.directory.as_path().exists() {
        fs::remove_dir_all(&inputs.directory)?;
    }

    fs::create_dir(&inputs.directory).context("Could not create directory to project.")?;
    Ok(())
}

pub fn build(inputs: Inputs) -> Result<()> {
    for entry in WalkDir::new(&BASE_DIRECTORY) {
        let entry = entry?;

        let source_path = entry.path();
        let relative_path = source_path.strip_prefix(&BASE_DIRECTORY).unwrap();
        let destination_path = fs::canonicalize(&inputs.directory).unwrap_or_default().join(relative_path);

        if entry.file_type().is_file() {
            fs::copy(&source_path, &destination_path)?;
        } else if entry.file_type().is_dir() {
            fs::create_dir_all(&destination_path)?;
        }
    }

    if !inputs.features.is_empty() {
        let project_package_json_path = format!("{}/{}", &inputs.directory.to_str().unwrap(), "package.json");
        let project_content = fs::read_to_string(&project_package_json_path).unwrap();
        let mut project_package_json: PackageJson = serde_json::from_str(&project_content).unwrap();
        let mut project_dependencies: HashMap<String, Option<String>> = HashMap::new();
        let mut project_dev_dependencies: HashMap<String, Option<String>> = HashMap::new();

        for feature in inputs.features.iter() {
            let feature_directory = format!("{}{}", BASE_FEATURES_DIRECTORY, feature);
            for entry in WalkDir::new(&feature_directory) {
                let entry = entry?;

                let source_path = entry.path();
                let relative_path = source_path.strip_prefix(&feature_directory).unwrap();
                let destination_path = fs::canonicalize(&inputs.directory).unwrap_or_default().join(relative_path);

                if entry.file_type().is_file() {
                    let sufix = entry.file_name().to_str().unwrap().split(".").collect::<Vec<&str>>()[1];

                    if sufix == "json" {
                        let project_name = inputs.directory.to_str().unwrap().split("./").collect::<Vec<&str>>()[1];
                        project_package_json.name = Some(project_name.to_string());
                        let feature_package_json_path = format!("{}/{}", &feature_directory, entry.file_name().to_str().unwrap());
                        let feature_content = fs::read_to_string(feature_package_json_path).unwrap();
                        let feature_package_json: PackageJson = serde_json::from_str(&feature_content).unwrap();

                        if feature_package_json.dependencies.is_some() {
                            let dependencies = feature_package_json.dependencies.unwrap();
                            for (key, value) in dependencies.iter() {
                                project_dependencies.insert(key.to_owned(), value.to_owned());
                            }
                            if let Some(project_deps) = project_package_json.dependencies {
                                for (key, value) in project_deps.iter() {
                                    project_dependencies.insert(key.to_owned(), value.to_owned());
                                }
                            }
                            project_package_json.dependencies = Some(project_dependencies.clone());
                        }

                        if feature_package_json.dev_dependencies.is_some() {
                            let dev_dependencies = feature_package_json.dev_dependencies.unwrap();
                            for (key, value) in dev_dependencies.iter() {
                                project_dev_dependencies.insert(key.to_owned(), value.to_owned());
                            }
                            if let Some(project_deps) = project_package_json.dev_dependencies {
                                for (key, value) in project_deps.iter() {
                                    project_dev_dependencies.insert(key.to_owned(), value.to_owned());
                                }
                            }
                            project_package_json.dev_dependencies = Some(project_dev_dependencies.clone());
                        }

                        fs::write(&project_package_json_path, serde_json::to_string_pretty::<PackageJson>(&project_package_json).unwrap())?;
                    } else {
                        fs::copy(&source_path, &destination_path)?;
                    }
                } else if entry.file_type().is_dir() {
                    fs::create_dir_all(&destination_path)?;
                }
            }
        }
    }

    Ok(())
}
