use std::fmt::format;
use std::path::Component::ParentDir;
use std::process::Command;
use anyhow::Result;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::lib::config::Config;
use crate::lib::resources::Resources;


#[derive(Deserialize, Serialize)]
pub struct BqTable {
    pub project_id: String,
    pub dataset_id: String,
    pub table_id: String,

}
impl BqTable {
    pub fn new (project_id: &str, dataset_id: &str, table_id: &str) -> Self {
        Self {
            project_id: project_id.to_string(),
            dataset_id: dataset_id.to_string(),
            table_id: table_id.to_string()}
    }
    pub fn flatten(&self) -> String {
        format!("{}:{}.{}", self.project_id, self.dataset_id, self.table_id)
    }

    pub fn formatted_flatten(&self) -> String {
        format!("--bigquery-table={}:{}.{}", self.project_id, self.dataset_id, self.table_id)
    }
    pub fn create(&mut self) -> Result<()>{
        // create bq instance from config.artifacts.resources.yaml if names were provided, otherwise names dataset dynamically "beaver_{random_string}" and table "table1"

        // create dataset & store id
        if self.dataset_id.is_empty() { self.dataset_id = create_dataset_named(&self.project_id)?;
        } else  { create_dataset_named(&self.project_id)? }

        // create table & store id
        if self.table_id.is_empty() {
            create_table(&self.dataset_id, "table1", &self.project_id)?;
            self.table_id = String::from("table1");
        } else {
            create_table(&self.dataset_id, &self.table_id, &self.project_id)?;
        }

        Ok(())
    }

}

pub fn create_dataset_named(project_id: &str) -> Result<String> {
    let mut random_string: String;
    let mut dataset_id_binding: String;

    loop {
        random_string = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(9)
            .map(char::from)
            .map(|c| c.to_ascii_lowercase())
            .collect();
        dataset_id_binding = format!("{}:beaver_datalake_{}", project_id, random_string);
        let args: Vec<&str> = Vec::from(["mk", "--dataset", &dataset_id_binding, ]);
        if Command::new("bq").args(args).status().unwrap().success() {
            break
        } else {
            continue
        }
    }

    Ok(dataset_id_binding)
}

pub fn create_dataset_unnamed(dataset_id: &str, project_id: &str) -> Result<()> {
    let id_binding = format!("{}:{}", project_id, dataset_id);
    let args: Vec<&str> = Vec::from(["mk", "--dataset", &id_binding, ]);
    Command::new("bq").args(args).spawn().unwrap().wait_with_output()?;
    Ok(())
}

pub fn create_table(dataset_id: &str, table_id: &str, project_id: &str) -> Result<()> {
    let id_binding = format!("{}:{}.{}", project_id, dataset_id, table_id);
    let args: Vec<&str> = Vec::from([
        "mk",
        "--table",
        id_binding.as_ref(),
        "data: JSON"
    ]);
    Command::new("bq").args(args).spawn().unwrap().wait_with_output()?;
    Ok(())
}



pub fn check_for_gcloud() -> Result<()> {
    match Command::new("gcloud").output() {
        Ok(_) => return Ok(()),
        Err(_) => panic!("Please ensure you have gcloud (google-cloud-sdk) installed"),
    }
}
pub fn check_for_bq() -> Result<()> {
    match Command::new("bq").output() {
        Ok(_) => return Ok(()),
        Err(_) => panic!("Please ensure you have bq (biqquery utility tool installed)"),
    }
}
