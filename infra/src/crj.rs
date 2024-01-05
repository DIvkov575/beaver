use std::collections::HashMap;
use std::error::Error;
use std::process::{Command, Output};

use serde_yaml::Mapping;

use crate::config::Config;

pub fn delete(job_name: &str, config: &Config) -> Result<(), Box<dyn Error>>{
    let mut args: Vec<&str> =  Vec::from(["run", "jobs", "delete", job_name]);
    args.extend(config.flatten());

    Command::new("gcloud").args(args).status()?;
    Ok(())
}

pub fn create() -> Result<(), Box<dyn Error>>{
    create_on_gcs()?;
    mount_gcs("", "vector.yaml")?;

    Ok(())
}


pub fn create_on_gcs(job_name: &str, config: &Config) -> Result<(), Box<dyn Error>>{
    let image_url = "docker.io/timberio/vector:latest-alpine";
    let mut args: Vec<&str> =  Vec::from(["run", "jobs", "create", job_name, "--image", image_url]);
    args.extend(config.flatten());

    Command::new("gcloud").args(args).status()?;
    Ok(())
}

pub fn describe_formatted(job_name: &str, config: &Config) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut args: Vec<&str> =  Vec::from(["run", "jobs", "describe", job_name, "--format", "export"]);
    args.extend(config.flatten());

    let Output {status: _, stdout: raw_out, stderr: raw_err } = Command::new("gcloud").args(args).output()?;
    if !raw_err.is_empty() {
        return Err(String::from_utf8(raw_err)?.into())
    }

    Ok(raw_out)
}


pub fn mount_gcs(bucket_name: &str, volume_name: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    #[macro_export]
    macro_rules! gm {($a:ident, $($b:literal,)*) => {$a$(.get_mut($b).ok_or("error during description unwrap")?)*};}

    let mut description: Mapping = serde_yaml::from_slice(&describe_formatted()?)?;

    let volume_mounts=  Vec::from([
        Mapping::from_iter(HashMap::from([
            ("mountPath".into(), "/etc/vector".into()),
            ("name".into(), volume_name.into())
        ]))
    ]);

    let volumes = Vec::from([
        Mapping::from_iter([
            ("name".into(), volume_name.into()),
            ("csi".into(), serde_yaml::Value::Mapping(Mapping::from_iter([
                ("driver".into(),"gcsfuse.run.googleapis.com".into()),
                ("volumeAttributes".into(), serde_yaml::Value::Mapping(Mapping::from_iter([
                    ("bucketName".into(), bucket_name.into()),
                ])))
            ])))
        ])
    ]);


    let point_a=  gm!(description, "spec", "template", "spec", "template", "spec", "containers", 0,).as_mapping_mut().unwrap();
    point_a.insert("volumeMounts".into(), volume_mounts.into());
    let point_b =  gm!(description, "spec", "template", "spec", "template", "spec",).as_mapping_mut().unwrap();
    point_b.insert("volumes".into(), volumes.into());

    let tmp_file = tempfile::NamedTempFile::new()?;
    serde_yaml::to_writer(&tmp_file, &description).unwrap();

    let mut args: Vec<&str> =  Vec::from(["beta", "run", "jobs", "replace", tmp_file.path().to_str().unwrap()]);
    args.extend(config.flatten());
    Command::new("gcloud").args(args).status()?;

    Ok(())
}