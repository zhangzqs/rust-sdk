mod base_types;
mod base_utils;
mod client;
mod enums;
mod form;
mod header;
mod json;
mod mods;
mod multipart;
mod path;
mod query;
mod traits;

use anyhow::Result;
use client::ApiDetailedDescription;
use mods::{FileProperty, Mods};
use proc_macro2::TokenStream;
use serde_yaml::from_reader as yaml_from_reader;
use std::{
    collections::VecDeque,
    env::{self, current_dir},
    ffi::{OsStr, OsString},
    fs::{copy, create_dir_all, remove_dir_all, write, File},
    path::{Component, Path, PathBuf},
    process::{Command, Stdio},
};
use walkdir::{DirEntry, WalkDir};

fn main() -> Result<()> {
    env_logger::init();

    generate_rust_modules()
}

fn generate_rust_modules() -> Result<()> {
    let mut mods = Mods::default();
    let current_src_dir = current_dir()?.join("src");
    let parent_dir = current_dir()?.join("..");
    let api_spec_dir = parent_dir.join("api-specs");
    let apis_dir = parent_dir.join("apis");
    let apis_generated_dir = apis_dir.join("src");
    let apis_cargo_toml_path = apis_dir.join("Cargo.toml");
    remove_dir_all(&apis_generated_dir).ok();
    create_dir_all(&apis_generated_dir)?;

    for read_result in WalkDir::new(&api_spec_dir)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
    {
        let entry = read_result?;
        let namespace = extract_namespace(&entry, &api_spec_dir)?;
        let file_name = Path::new(entry.file_name());
        if matches!(
            Path::new(entry.file_name())
                .extension()
                .and_then(OsStr::to_str),
            Some("yml" | "yaml")
        ) {
            write_to_rust_mod(
                file_name.with_extension(""),
                namespace,
                entry.path(),
                &apis_generated_dir,
                &mut mods,
            )?;
        }
    }

    copy(
        current_src_dir.join("base_types.rs"),
        apis_generated_dir.join("base_types.rs"),
    )?;
    copy(
        current_src_dir.join("base_utils.rs"),
        apis_generated_dir.join("base_utils.rs"),
    )?;
    mods.add(
        OsString::from("base_types"),
        Default::default(),
        FileProperty::PublicInternal {
            documentation: "七牛 API 所用的基础类型库".to_owned(),
        },
    );
    mods.add(
        OsString::from("base_utils"),
        Default::default(),
        FileProperty::PrivateInternal,
    );
    mods.write_to_rust_mod(&apis_generated_dir)?;
    run_cargo_fmt(&apis_cargo_toml_path)?;
    run_make_build(&apis_dir)?;
    run_make_clippy(&apis_dir)?;

    return Ok(());

    fn is_hidden(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with('.'))
            .unwrap_or(false)
    }

    fn extract_namespace(entry: &DirEntry, prefix: &Path) -> Result<Vec<OsString>> {
        let namespace = entry
            .path()
            .strip_prefix(prefix)?
            .parent()
            .map(|dir_path| {
                dir_path
                    .components()
                    .map(|component| {
                        if let Component::Normal(component) = component {
                            component.to_owned()
                        } else {
                            unreachable!()
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();
        Ok(namespace)
    }

    fn write_to_rust_mod(
        base_name: PathBuf,
        namespace: Vec<OsString>,
        api_spec_path: &Path,
        apis_generated_dir: &Path,
        mods: &mut Mods,
    ) -> Result<()> {
        let output_file_path = namespace
            .iter()
            .fold(apis_generated_dir.to_owned(), |path, segment| {
                path.join(segment)
            })
            .join(Path::new(&base_name).with_extension("rs"));
        if let Some(output_dir_path) = output_file_path.parent() {
            create_dir_all(output_dir_path)?;
        }
        let api_detailed_spec: ApiDetailedDescription =
            yaml_from_reader(File::open(api_spec_path)?)?;
        let auto_generated_code =
            "// THIS FILE IS GENERATED BY api-generator, DO NOT EDIT DIRECTLY!\n//\n".to_owned()
                + &generate_rust_token_stream(&api_detailed_spec).to_string();
        write(&output_file_path, auto_generated_code.as_bytes())?;

        mods.add(
            base_name.into_os_string(),
            VecDeque::from_iter(namespace),
            FileProperty::Public {
                documentation: api_detailed_spec.documentation,
            },
        );

        Ok(())
    }

    fn generate_rust_token_stream(detailed_description: &ApiDetailedDescription) -> TokenStream {
        detailed_description.to_rust_token_stream()
    }

    fn run_cargo_fmt(cargo_toml_path: &Path) -> Result<()> {
        let status = Command::new("cargo")
            .arg("fmt")
            .arg("--manifest-path")
            .arg(cargo_toml_path)
            .stdin(Stdio::null())
            .env_remove("RUST_LOG")
            .status()?;
        assert!(status.success());
        Ok(())
    }

    fn run_make_build(dir_path: &Path) -> Result<()> {
        let status = Command::new("make")
            .arg("-C")
            .arg(dir_path)
            .arg("build")
            .stdin(Stdio::null())
            .env_remove("RUST_LOG")
            .status()?;
        assert!(status.success());
        Ok(())
    }

    fn run_make_clippy(dir_path: &Path) -> Result<()> {
        if env::var_os("SKIP_CLIPPY").is_none() {
            let status = Command::new("make")
                .arg("-C")
                .arg(dir_path)
                .arg("clippy")
                .stdin(Stdio::null())
                .env_remove("RUST_LOG")
                .status()?;
            assert!(status.success());
        }
        Ok(())
    }
}
