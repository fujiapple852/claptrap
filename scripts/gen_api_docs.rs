use anyhow::{Context, Result};
use minijinja::{context, Environment};
use regex::Regex;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize)]
struct Example {
    toml: String,
    yaml: String,
    json: String,
    args: String,
    output: String,
}

fn parse_table(path: impl AsRef<Path>) -> Result<Vec<(String, String)>> {
    let row_re = Regex::new(r"\|\s*\[`[^`]+`\]\((?P<slug>[^)]+)\)\s*\|\s*(?P<desc>[^|]+)\|")?;
    let text =
        fs::read_to_string(path.as_ref()).context(path.as_ref().to_string_lossy().to_string())?;
    let mut entries = Vec::new();
    for line in text.lines() {
        if let Some(caps) = row_re.captures(line) {
            let slug = caps["slug"].to_string();
            let desc = caps["desc"].trim().to_string();
            entries.push((slug, desc));
        }
    }
    Ok(entries)
}

fn read_snapshot(path: &Path) -> Result<String> {
    let text = fs::read_to_string(path).context(path.to_string_lossy().to_string())?;
    let re_block = Regex::new(r"(?s)command cat <<'EOF[^\n]*\n(?P<body>.*?)\nEOF")?;
    let output = if let Some(caps) = re_block.captures(&text) {
        caps["body"].to_string()
    } else {
        let mut lines = text.lines();
        let mut dash_count = 0;
        for line in lines.by_ref() {
            if line.starts_with("---") {
                dash_count += 1;
                if dash_count == 2 {
                    break;
                }
            }
        }
        lines.collect::<Vec<_>>().join("\n")
    };
    let ansi_re = Regex::new("\x1b\\[[0-9;]*m")?;
    let output = ansi_re.replace_all(&output, "");
    Ok(output.trim().to_string())
}

fn convert_spec(toml_str: &str) -> Result<(String, String)> {
    let value: toml::Value = toml::from_str(toml_str)?;
    let yaml = serde_yaml::to_string(&value)?.trim_end().to_string();
    let json = serde_json::to_string_pretty(&value)?;
    Ok((yaml, json))
}

fn build_example(module: &str, base: &str) -> Result<Example> {
    let spec_path = format!("tests/resources/{module}/{base}.toml");
    let args_path = format!("tests/resources/{module}/{base}.args");
    let snap_path = format!("src/snapshots/claptrap__tests__{module}__{base}.snap");
    let toml = fs::read_to_string(&spec_path)
        .context(spec_path)?
        .trim_end()
        .to_string();
    let args = fs::read_to_string(&args_path)
        .context(args_path)?
        .trim_end()
        .to_string();
    let output = read_snapshot(Path::new(&snap_path))?;
    let (yaml, json) = convert_spec(&toml)?;
    Ok(Example {
        toml,
        yaml,
        json,
        args,
        output,
    })
}

fn struct_name(module: &str) -> &'static str {
    match module {
        "arg" => "Arg",
        "command" => "Command",
        "arg_group" => "ArgGroup",
        "value_parser" => "ValueParser",
        "possible_value" => "PossibleValue",
        _ => unreachable!(),
    }
}

fn doc_url(module: &str, slug: &str) -> String {
    match module {
        "arg" => format!("https://docs.rs/clap/latest/clap/struct.Arg.html#method.{slug}"),
        "command" => format!("https://docs.rs/clap/latest/clap/struct.Command.html#method.{slug}"),
        "arg_group" => {
            format!("https://docs.rs/clap/latest/clap/struct.ArgGroup.html#method.{slug}")
        }
        "value_parser" => format!("https://docs.rs/clap/latest/clap/value_parser/fn.{slug}.html"),
        "possible_value" => {
            format!(
                "https://docs.rs/clap/latest/clap/builder/struct.PossibleValue.html#method.{slug}"
            )
        }
        _ => unreachable!(),
    }
}

fn generate_docs(
    env: &Environment<'_>,
    module: &str,
    slug: &str,
    desc: &str,
    examples: &[Example],
) -> Result<String> {
    let struct_name = struct_name(module);
    let title = format!("{struct_name}::{slug}");
    let link = doc_url(module, slug);
    let tpl = env.get_template("api_doc")?;
    let rendered = tpl.render(context! {
        struct => struct_name,
        slug => slug,
        desc => desc,
        title => title,
        sidebar => slug,
        link => link,
        examples => examples,
    })?;
    Ok(rendered)
}

fn collect_examples(module: &str, slug: &str) -> Result<Vec<Example>> {
    let base_dir = Path::new("tests/resources").join(module);
    let mut examples = Vec::new();
    let direct = base_dir.join(format!("{slug}.toml"));
    let mut files: Vec<PathBuf> = Vec::new();
    if direct.exists() {
        files.push(direct);
    }
    for entry in fs::read_dir(&base_dir).context(base_dir.to_string_lossy().to_string())? {
        let entry = entry?;
        if let Some(stem) = entry.path().file_stem().and_then(|s| s.to_str()) {
            if stem.starts_with(&format!("{slug}-"))
                && entry.path().extension() == Some("toml".as_ref())
            {
                files.push(entry.path());
            }
        }
    }
    files.sort();
    for file in files {
        if let Some(stem) = file.file_stem().and_then(|s| s.to_str()) {
            if let Ok(ex) = build_example(module, stem) {
                examples.push(ex);
            } else {
                eprintln!("Failed to build example from {}", file.display());
            }
        }
    }
    Ok(examples)
}

fn main() -> Result<()> {
    let path_prefix = "docs/src/content/docs/reference/api";
    let modules = [
        ("arg", "arg.mdx"),
        ("command", "command.mdx"),
        ("arg_group", "arg_group.mdx"),
        ("value_parser", "value_parser.mdx"),
        ("possible_value", "possible_value.mdx"),
    ];
    let mut env = Environment::new();
    env.set_trim_blocks(true);
    env.set_lstrip_blocks(true);
    env.add_template("api_doc", include_str!("../docs/templates/api_doc.mdx.j2"))?;
    for (module, table) in modules {
        let entries = parse_table(Path::new(path_prefix).join(table))?;
        let out_dir = Path::new(path_prefix).join(module);
        fs::create_dir_all(&out_dir)?;
        for (slug, desc) in entries {
            let out_file = out_dir.join(format!("{slug}.mdx"));

            // hack to as index.mdx is special in the web docs
            let slug = if slug == "arg_index" {
                String::from("index")
            } else {
                slug
            };

            let examples = collect_examples(module, &slug)?;
            if examples.is_empty() {
                println!("WARNING: skipping {module}::{slug} (no examples found)");
                continue;
            }
            let content = generate_docs(&env, module, &slug, &desc, &examples)?;
            fs::write(&out_file, content)?;
            println!("wrote {}", out_file.display());
        }
    }
    Ok(())
}
