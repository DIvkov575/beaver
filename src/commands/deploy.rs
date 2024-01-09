use std::fs::{File, OpenOptions};
use std::path::Path;

use anyhow::{anyhow, Result};
use serde_yaml::{Mapping, Value};

use crate::lib::{bq::{
    self
}, config::Config, crj, cron, gcs};
use crate::lib::resources::Resources;

pub fn deploy(path_arg: &str) -> Result<()> {
    validate_config_path(&Path::new(path_arg))?;
    bq::check_for_bq()?;
    bq::check_for_gcloud()?;

    let path = Path::new(path_arg);
    let config: Config = Config::from_path(&path);
    let mut resources: Resources =  serde_yaml::from_reader(
        File::open(path.join("artifacts/resources.yaml"))?
    )?;

    resources.biq_query.get_mut().unwrap().create()?;
    resources.output_pubsub.get_mut().unwrap().create(&resources, &config)?;
    generate_vector_config(&path, &resources, &config)?;

    gcs::create_bucket(&resources, &config)?;
    gcs::upload_to_bucket(
        path
            .join("artifacts/vector.yaml")
            .to_str()
            .ok_or(anyhow!("path `<config>/artifacts/vector.yaml`"))?,
        &resources, &config)?;

    crj::create_vector(&resources, &config)?;
    cron::create_scheduler(&resources, &config)?;




    Ok(())
}



fn generate_vector_config(path: &Path, resources: &Resources, config: &Config ) -> Result<()> {
    let beaver_config: Mapping = serde_yaml::from_reader(&File::open(path.join("../beaver_config/beaver_config.yaml"))?)?;
    let vector_config_file = OpenOptions::new().write(true).create(true).open(path.join("artifacts/vector.yaml"))?;

    let _output_pubsub_binding= resources.output_pubsub.borrow();
    let output_pubsub = _output_pubsub_binding.as_ref().unwrap();
    let sources_yaml =  beaver_config[&Value::String("sources".into())].clone();
    let transforms_yaml =  beaver_config[&Value::String("transforms".into())].clone();

    let transforms = transforms_yaml
        .as_mapping()
        .iter()
        .map(|mapping| mapping
            .iter()
            .map(|(key ,value)|
                key.as_str().unwrap().to_string()
            ).collect::<Vec<String>>()[0].clone())
        .map(|x| Value::String(x))
        .collect::<Vec<Value>>();


    let sinks_yaml: Value = Value::Mapping(Mapping::from_iter([
        (Value::String("bq_writing_pubsub".into()), Value::Mapping(Mapping::from_iter([
            (Value::String("type".into()), Value::String("gcp_pubsub".into())),
            (Value::String("inputs".into()), Value::Sequence(serde_yaml::Sequence::from(transforms))),
            (Value::String("project".into()), Value::String(config.project.clone().into())),
            (Value::String("topic".into()), Value::String(output_pubsub.topic_id.clone().into())),
            (Value::String("encoding".into()), Value::Mapping(Mapping::from_iter([
                ("codec".into(), "json".into())
            ]))),
        ])))
    ]));

    let vector_config: Mapping = Mapping::from_iter([
        ("sources".into(), sources_yaml),
        ("transforms".into(), transforms_yaml),
        ("sinks".into(), sinks_yaml)
    ]);

    serde_yaml::to_writer(&vector_config_file, &vector_config).unwrap();

    Ok(())
}

fn validate_config_path(path: &Path) -> Result<()> {
    if !path.join("../beaver_config/beaver_config.yaml").exists() {
        return Err(anyhow::anyhow!("config path does not exist or broken"));
    }
    Ok(())
}