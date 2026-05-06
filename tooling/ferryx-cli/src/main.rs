use std::path::{Path, PathBuf};
use std::time::Instant;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use ferryx_ir::validate_ir_compatibility;
use ferryx_build::{run_build, BuildConfig};
use ferryx_rewrite::{default_python_rewrite_pipeline, RewriteContext};
use owo_colors::OwoColorize;
use sha2::{Digest, Sha256};

#[derive(Debug, Parser)]
#[command(name = "cargo-ferryx")]
#[command(about = "ferryx build and inspection tooling")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    New {
        #[arg(long)]
        name: String,
        #[arg(long, default_value = ".")]
        dir: PathBuf,
    },
    Build {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        out_dir: PathBuf,
        #[arg(long, default_value = "ferryx_project")]
        package: String,
        #[arg(long, default_value_t = false)]
        maturin: bool,
    },
    Inspect {
        #[arg(long)]
        input: PathBuf,
        #[arg(long, default_value = "ferryx_project")]
        package: String,
    },
    InspectIr {
        #[arg(long)]
        input: PathBuf,
        #[arg(long, default_value = "ferryx_project")]
        package: String,
        #[arg(long)]
        out_json: Option<PathBuf>,
    },
    EmitPython {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        out_dir: PathBuf,
        #[arg(long, default_value = "ferryx_project")]
        package: String,
    },
    GenerateArtifacts {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        example_dir: PathBuf,
        #[arg(long, default_value = "ferryx_project")]
        package: String,
    },
    VerifyArtifacts {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        example_dir: PathBuf,
        #[arg(long, default_value = "ferryx_project")]
        package: String,
    },
    Graph {
        #[arg(long)]
        input: PathBuf,
        #[arg(long, default_value = "ferryx_project")]
        package: String,
        #[arg(long, default_value = "mermaid")]
        format: String,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    Doctor,
    Explain {
        #[arg(long)]
        input: PathBuf,
        #[arg(long, default_value = "ferryx_project")]
        package: String,
    },
    Trace {
        #[arg(long)]
        input: PathBuf,
        #[arg(long, default_value = "ferryx_project")]
        package: String,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    InspectRewrite,
    InspectRegistry,
    Benchmark {
        #[arg(long, default_value = "all")]
        suite: String,
        #[arg(long, default_value = "evaluation/results/latest.json")]
        output: PathBuf,
    },
    Dev {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        out_dir: PathBuf,
        #[arg(long, default_value = "ferryx_project")]
        package: String,
    },
    Docs {
        #[arg(long)]
        input: PathBuf,
        #[arg(long, default_value = "ferryx_project")]
        package: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::New { name, dir } => {
            let root = dir.join(&name);
            std::fs::create_dir_all(root.join("src"))?;
            std::fs::write(
                root.join("Cargo.toml"),
                format!(
                    "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nferryx-macros = {{ path = \"../core/ferryx-macros\" }}\n"
                ),
            )?;
            std::fs::write(
                root.join("src/lib.rs"),
                "use ferryx_macros::ferryx;\n\n#[ferryx]\npub struct Example { pub value: i32 }\n",
            )?;
            println!("{}", format!("created {}", root.display()).green());
        }
        Command::Build {
            input,
            out_dir,
            package,
            maturin,
        } => {
            let artifacts = run_build(&BuildConfig {
                package_name: package,
                input_rust: input,
                out_dir,
                run_maturin: maturin,
            })?;
            println!("{}", artifacts.ir_json.display().to_string().green());
            for file in artifacts.emitted_files {
                println!("{}", file.display());
            }
        }
        Command::Inspect { input, package } => {
            let source =
                std::fs::read_to_string(&input).with_context(|| format!("reading {}", input.display()))?;
            let ir = ferryx_parser::parse_source_to_ir(&package, &source)?;
            validate_ir_compatibility(&ir).map_err(anyhow::Error::msg)?;
            if matches!(ir.stability, ferryx_ir::StabilityLevel::Experimental) {
                eprintln!("{}", "semantic warning: IR stability is experimental".yellow());
            }
            println!("{}", serde_json::to_string_pretty(&ir)?);
        }
        Command::InspectIr {
            input,
            package,
            out_json,
        } => {
            let source = std::fs::read_to_string(&input)?;
            let ir = ferryx_parser::parse_source_to_ir(&package, &source)?;
            validate_ir_compatibility(&ir).map_err(anyhow::Error::msg)?;
            if matches!(ir.stability, ferryx_ir::StabilityLevel::Experimental) {
                eprintln!("{}", "semantic warning: IR stability is experimental".yellow());
            }
            let json = serde_json::to_string_pretty(&ir)?;
            if let Some(path) = out_json {
                std::fs::write(&path, json)?;
                println!("{}", format!("wrote {}", path.display()).green());
            } else {
                println!("{json}");
            }
        }
        Command::EmitPython {
            input,
            out_dir,
            package,
        } => {
            let artifacts = run_build(&BuildConfig {
                package_name: package,
                input_rust: input,
                out_dir,
                run_maturin: false,
            })?;
            println!(
                "{}",
                format!("generated {} files", artifacts.emitted_files.len()).green()
            );
        }
        Command::GenerateArtifacts {
            input,
            example_dir,
            package,
        } => {
            let started = Instant::now();
            let generated_dir = example_dir.join("generated");
            let artifacts = run_build(&BuildConfig {
                package_name: package.clone(),
                input_rust: input.clone(),
                out_dir: generated_dir.clone(),
                run_maturin: false,
            })?;
            let source = std::fs::read_to_string(&input)?;
            let ir = ferryx_parser::parse_source_to_ir(&package, &source)?;
            let mmd = render_mermaid(&ir);
            let dot = render_dot(&ir);
            let docs = render_docs_md(&ir);
            write_text(&generated_dir.join("graph.mmd"), &mmd)?;
            write_text(&generated_dir.join("graph.dot"), &dot)?;
            write_text(&generated_dir.join("docs.md"), &docs)?;
            std::fs::rename(artifacts.ir_json, generated_dir.join("ir.json"))?;
            let metadata = build_metadata(&generated_dir)?;
            write_text(
                &generated_dir.join("metadata.json"),
                &serde_json::to_string_pretty(&metadata)?,
            )?;
            println!(
                "{}",
                format!(
                    "generated artifacts in {} ({:.2?})",
                    generated_dir.display(),
                    started.elapsed()
                )
                .green()
            );
        }
        Command::VerifyArtifacts {
            input,
            example_dir,
            package,
        } => {
            let generated_dir = example_dir.join("generated");
            let before = std::fs::read_to_string(generated_dir.join("metadata.json"))
                .context("missing metadata.json, run generate-artifacts first")?;
            let before_meta: ArtifactMetadata = serde_json::from_str(&before)?;

            let tmp = example_dir.join(".generated-verify");
            if tmp.exists() {
                std::fs::remove_dir_all(&tmp)?;
            }
            run_build(&BuildConfig {
                package_name: package.clone(),
                input_rust: input.clone(),
                out_dir: tmp.clone(),
                run_maturin: false,
            })?;
            let source = std::fs::read_to_string(&input)?;
            let ir = ferryx_parser::parse_source_to_ir(&package, &source)?;
            write_text(&tmp.join("graph.mmd"), &render_mermaid(&ir))?;
            write_text(&tmp.join("graph.dot"), &render_dot(&ir))?;
            write_text(&tmp.join("docs.md"), &render_docs_md(&ir))?;
            std::fs::rename(tmp.join("ferryx-ir.json"), tmp.join("ir.json"))?;
            let after_meta = build_metadata(&tmp)?;
            std::fs::remove_dir_all(&tmp)?;

            if before_meta.hashes != after_meta.hashes {
                anyhow::bail!("artifact drift detected for {}", example_dir.display());
            }
            println!("{}", "artifact verification passed".green());
        }
        Command::Graph {
            input,
            package,
            format,
            output,
        } => {
            let source = std::fs::read_to_string(&input)?;
            let ir = ferryx_parser::parse_source_to_ir(&package, &source)?;
            let rendered = if format == "dot" { render_dot(&ir) } else { render_mermaid(&ir) };
            if let Some(path) = output {
                write_text(&path, &rendered)?;
                println!("{}", format!("wrote {}", path.display()).green());
            } else {
                println!("{rendered}");
            }
        }
        Command::Doctor => {
            println!("{}", "ferryx doctor".bold());
            println!(
                "workspace:{}",
                if Path::new("Cargo.toml").exists() {
                    "ok".green().to_string()
                } else {
                    "missing".red().to_string()
                }
            );
            println!(
                "benchmarks:{}",
                if Path::new("tooling/ferryx-bench/Cargo.toml").exists() {
                    "ok".green().to_string()
                } else {
                    "missing".red().to_string()
                }
            );
            println!(
                "maturin:{}",
                command_exists("maturin")
                    .then_some("ok".green().to_string())
                    .unwrap_or_else(|| "missing".red().to_string())
            );
            println!(
                "python:{}",
                command_exists("python3")
                    .then_some("ok".green().to_string())
                    .unwrap_or_else(|| "missing".red().to_string())
            );
        }
        Command::Explain { input, package } => {
            let source = std::fs::read_to_string(&input)?;
            let mut ir = ferryx_parser::parse_source_to_ir(&package, &source)?;
            default_python_rewrite_pipeline().run(&mut ir, &RewriteContext::default());
            println!("{}", "Rewrite pipeline explanation".bold());
            for module in &ir.modules {
                for imp in &module.impls {
                    for method in &imp.methods {
                        println!(
                            "{} -> {}",
                            format!("{}::{}", imp.target.rust, method.name).cyan(),
                            method.output.rust.yellow()
                        );
                    }
                }
            }
        }
        Command::Trace {
            input,
            package,
            output,
        } => {
            let source = std::fs::read_to_string(&input)?;
            let mut ir = ferryx_parser::parse_source_to_ir(&package, &source)?;
            let mut trace = String::new();
            trace.push_str("flowchart LR\n");
            for pass in default_python_rewrite_pipeline().pass_names() {
                trace.push_str(&format!("  rustSemantics --> {}[{}]\n", pass, pass));
            }
            trace.push_str("  async_projection --> emittedPython[PythonAPI]\n");
            default_python_rewrite_pipeline().run(&mut ir, &RewriteContext::default());
            if let Some(path) = output {
                write_text(&path, &trace)?;
            } else {
                println!("{trace}");
            }
        }
        Command::InspectRewrite => {
            println!("{}", "rewrite passes".bold());
            for pass in default_python_rewrite_pipeline().pass_names() {
                println!("- {}", pass);
            }
        }
        Command::InspectRegistry => {
            let items = ferryx_runtime::all_items();
            println!("registered_items: {}", items.len());
            for item in items {
                println!("{}::{}", item.module_path, item.item_name);
            }
        }
        Command::Benchmark { suite, output } => {
            let status = std::process::Command::new("cargo")
                .arg("run")
                .arg("--manifest-path")
                .arg("tooling/ferryx-bench/Cargo.toml")
                .arg("--")
                .arg("--suite")
                .arg(suite)
                .arg("--output")
                .arg(&output)
                .status()?;
            if !status.success() {
                anyhow::bail!("benchmark run failed");
            }
            println!("{}", format!("benchmark results at {}", output.display()).green());
        }
        Command::Dev {
            input,
            out_dir,
            package,
        } => {
            let artifacts = run_build(&BuildConfig {
                package_name: package,
                input_rust: input,
                out_dir,
                run_maturin: false,
            })?;
            println!("generated {} files", artifacts.emitted_files.len());
        }
        Command::Docs { input, package } => {
            let source =
                std::fs::read_to_string(&input).with_context(|| format!("reading {}", input.display()))?;
            let ir = ferryx_parser::parse_source_to_ir(&package, &source)?;
            for module in ir.modules {
                for class in module.classes {
                    println!("## {}", class.name);
                    if !class.docs.summary.is_empty() {
                        println!("{}", class.docs.summary);
                    }
                    for field in class.fields {
                        println!("- {}: {}", field.name, field.ty.rust);
                    }
                }
            }
        }
    }
    Ok(())
}

fn write_text(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, content)?;
    Ok(())
}

fn render_mermaid(ir: &ferryx_ir::IrPackage) -> String {
    let mut out = String::from("flowchart LR\n");
    out.push_str("  package[IrPackage]\n");
    for module in &ir.modules {
        out.push_str(&format!("  package --> module_{}[{}]\n", module.id, module.id));
        for class in &module.classes {
            out.push_str(&format!("  module_{} --> class_{}[{}]\n", module.id, class.id, class.name));
        }
    }
    out
}

fn render_dot(ir: &ferryx_ir::IrPackage) -> String {
    let mut out = String::from("digraph FerryxIR {\n  package [label=\"IrPackage\"];\n");
    for module in &ir.modules {
        out.push_str(&format!("  module_{} [label=\"{}\"];\n", module.id, module.id));
        out.push_str(&format!("  package -> module_{};\n", module.id));
        for class in &module.classes {
            out.push_str(&format!("  class_{} [label=\"{}\"];\n", class.id, class.name));
            out.push_str(&format!("  module_{} -> class_{};\n", module.id, class.id));
        }
    }
    out.push_str("}\n");
    out
}

fn render_docs_md(ir: &ferryx_ir::IrPackage) -> String {
    let mut out = String::new();
    out.push_str("# Generated API docs\n\n");
    for module in &ir.modules {
        out.push_str(&format!("## Module `{}`\n\n", module.id));
        for class in &module.classes {
            out.push_str(&format!("### {}\n", class.name));
            for field in &class.fields {
                out.push_str(&format!("- `{}`: `{}`\n", field.name, field.ty.rust));
            }
            out.push('\n');
        }
    }
    out
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ArtifactMetadata {
    hashes: std::collections::BTreeMap<String, String>,
}

fn build_metadata(dir: &Path) -> Result<ArtifactMetadata> {
    let files = [
        "__init__.py",
        "__init__.pyi",
        "ir.json",
        "graph.mmd",
        "graph.dot",
        "docs.md",
    ];
    let mut hashes = std::collections::BTreeMap::new();
    for path in std::fs::read_dir(dir)? {
        let entry = path?;
        let p = entry.path();
        if p.is_dir() {
            for nested in std::fs::read_dir(&p)? {
                let nested = nested?.path();
                if nested.extension().is_some() {
                    let name = nested
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or_default()
                        .to_string();
                    if files.contains(&name.as_str()) {
                        hashes.insert(
                            format!("{}/{}", p.file_name().unwrap().to_string_lossy(), name),
                            file_hash(&nested)?,
                        );
                    }
                }
            }
        } else if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
            if files.contains(&name) {
                hashes.insert(name.to_string(), file_hash(&p)?);
            }
        }
    }
    Ok(ArtifactMetadata { hashes })
}

fn file_hash(path: &Path) -> Result<String> {
    let content = std::fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(content);
    Ok(format!("{:x}", hasher.finalize()))
}

fn command_exists(bin: &str) -> bool {
    std::process::Command::new(bin)
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

