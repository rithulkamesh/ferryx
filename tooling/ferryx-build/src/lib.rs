use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use ferryx_ir::{validate_ir_compatibility, IrPackage};

#[derive(Debug, Clone)]
pub struct BuildConfig {
    pub package_name: String,
    pub input_rust: PathBuf,
    pub out_dir: PathBuf,
    pub run_maturin: bool,
}

#[derive(Debug, Clone)]
pub struct BuildArtifacts {
    pub ir_json: PathBuf,
    pub emitted_files: Vec<PathBuf>,
}

pub fn run_build(config: &BuildConfig) -> Result<BuildArtifacts> {
    let source = fs::read_to_string(&config.input_rust)
        .with_context(|| format!("failed to read Rust source {}", config.input_rust.display()))?;
    let package = ferryx_parser::parse_source_to_ir(&config.package_name, &source)
        .context("failed to parse Rust source into IR")?;
    validate_ir_compatibility(&package).map_err(anyhow::Error::msg)?;

    fs::create_dir_all(&config.out_dir)
        .with_context(|| format!("failed to create output dir {}", config.out_dir.display()))?;

    let ir_json = config.out_dir.join("ferryx-ir.json");
    write_ir_json(&ir_json, &package)?;

    let emission = ferryx_python::emit_python(&package);
    let mut emitted_files = Vec::new();
    for file in emission.files {
        let path = config.out_dir.join(file.path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create dir {}", parent.display()))?;
        }
        fs::write(&path, file.content)
            .with_context(|| format!("failed to write generated file {}", path.display()))?;
        emitted_files.push(path);
    }

    if config.run_maturin {
        run_maturin_build(&config.out_dir)?;
    }

    Ok(BuildArtifacts { ir_json, emitted_files })
}

pub fn write_ir_json(path: &Path, package: &IrPackage) -> Result<()> {
    let data = serde_json::to_string_pretty(package).context("failed to serialize IR package")?;
    fs::write(path, data).with_context(|| format!("failed to write IR JSON {}", path.display()))?;
    Ok(())
}

fn run_maturin_build(out_dir: &Path) -> Result<()> {
    let status = std::process::Command::new("maturin")
        .arg("build")
        .arg("--out")
        .arg(out_dir)
        .status()
        .context("failed to execute maturin build command")?;

    if status.success() {
        Ok(())
    } else {
        anyhow::bail!("maturin build exited with status: {status}");
    }
}

#[cfg(test)]
mod tests {
    use super::{run_build, BuildConfig};

    #[test]
    fn build_generates_tensor_python() {
        let temp = std::env::temp_dir().join(format!("ferryx-build-test-{}", std::process::id()));
        std::fs::create_dir_all(&temp).expect("create temp dir");
        let input = temp.join("tensor.rs");
        std::fs::write(
            &input,
            r#"
            pub struct Tensor { pub data: Vec<f32> }
            impl Tensor {
                pub fn add(&self, other: Tensor) -> Tensor { other }
            }
            "#,
        )
        .expect("write source fixture");

        let config = BuildConfig {
            package_name: "ferryx_tensor".into(),
            input_rust: input,
            out_dir: temp.join("out"),
            run_maturin: false,
        };
        let artifacts = run_build(&config).expect("build should pass");
        assert!(artifacts.ir_json.exists());
        assert!(artifacts
            .emitted_files
            .iter()
            .any(|path| path.to_string_lossy().ends_with("__init__.pyi")));
    }
}

