use crate::commands::utils::{copy_dir, create_dir_all, write_file};
use std::{env, io, path::Path};
use toml::{Table, Value};

// Main initialization function
pub fn init() -> io::Result<()> {
    // Initialize config
    init_config()?;

    // Create directory structure
    let current_dir = env::current_dir()?;
    create_directory_structure(&current_dir)?;

    Ok(())
}

// Create required directories
fn create_directory_structure(base_dir: &Path) -> io::Result<()> {
    let archive_dir = base_dir.join("archive");
    let problems_dir = base_dir.join("problems");

    create_dir_all(&archive_dir)?;
    create_dir_all(&problems_dir)?;

    Ok(())
}

// Initialize configuration files and directories
fn init_config() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let config_dir = current_dir.join(".boj");

    create_dir_all(&config_dir)?;
    copy_templates(&current_dir, &config_dir)?;
    create_config_file(&config_dir)?;

    Ok(())
}

// Copy template files to config directory
fn copy_templates(current_dir: &Path, config_dir: &Path) -> io::Result<()> {
    let templates_src = current_dir.join("src/templates");
    let templates_dest = config_dir.join("templates");
    copy_dir(&templates_src, &templates_dest)?;

    Ok(())
}

// Create configuration file with default settings
fn create_config_file(config_dir: &Path) -> io::Result<()> {
    let config_path = config_dir.join("config.toml");
    let config = create_default_config();

    let default_config =
        toml::to_string(&config).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    write_file(config_path, &default_config)?;
    Ok(())
}

// Create default configuration table
fn create_default_config() -> Table {
    let mut config = Table::new();

    // Add basic sections
    config.insert("general".to_string(), create_general_section());
    config.insert("workspace".to_string(), create_workspace_section());

    // Add language-specific configurations
    add_language_configs(&mut config);

    config
}

// Define language configurations
fn add_language_configs(config: &mut Table) {
    // Python configuration
    add_filetype_section(
        config,
        "py",
        LanguageConfig {
            language: "python3",
            main_file: "main.py",
            source_templates: None,
            run_cmd: Some("python3 $file"),
            compile_cmd: None,
            execute_cmd: None,
            after_cmd: None,
        },
    );

    // Rust configuration
    add_filetype_section(
        config,
        "rs",
        LanguageConfig {
            language: "rust",
            main_file: "main.rs",
            source_templates: None,
            run_cmd: None,
            compile_cmd: Some("rustc $file -o main"),
            execute_cmd: Some("./main"),
            after_cmd: None,
        },
    );

    // C configuration
    add_filetype_section(
        config,
        "c",
        LanguageConfig {
            language: "c",
            main_file: "main.c",
            source_templates: None,
            run_cmd: None,
            compile_cmd: Some("gcc -std=c11 $file -o a.out"),
            execute_cmd: Some("./a.out"),
            after_cmd: Some("rm -rf a.out"),
        },
    );

    // C++ configuration
    add_filetype_section(
        config,
        "cpp",
        LanguageConfig {
            language: "c++17",
            main_file: "main.cpp",
            source_templates: Some(vec!["default.cpp".to_string()]),
            run_cmd: None,
            compile_cmd: Some("g++ -std=c++17 $file"),
            execute_cmd: Some("./a.out"),
            after_cmd: Some("rm -rf a.out"),
        },
    );

    // Java configuration
    add_filetype_section(
        config,
        "java",
        LanguageConfig {
            language: "java",
            main_file: "Main.java",
            source_templates: None,
            run_cmd: None,
            compile_cmd: Some("javac $file"),
            execute_cmd: Some("java Main"),
            after_cmd: None,
        },
    );

    // JavaScript configuration
    add_filetype_section(
        config,
        "js",
        LanguageConfig {
            language: "node",
            main_file: "main.js",
            source_templates: None,
            run_cmd: Some("node $file"),
            compile_cmd: None,
            execute_cmd: None,
            after_cmd: None,
        },
    );
}

// Language configuration struct
struct LanguageConfig<'a> {
    language: &'a str,
    main_file: &'a str,
    source_templates: Option<Vec<String>>,
    run_cmd: Option<&'a str>,
    compile_cmd: Option<&'a str>,
    execute_cmd: Option<&'a str>,
    after_cmd: Option<&'a str>,
}

// Convert language configuration to TOML table
fn add_filetype_section(config: &mut Table, filetype: &str, lang_config: LanguageConfig) {
    let mut section = Table::new();

    section.insert(
        "language".to_string(),
        Value::String(lang_config.language.to_string()),
    );
    section.insert(
        "main".to_string(),
        Value::String(lang_config.main_file.to_string()),
    );

    if let Some(templates) = lang_config.source_templates {
        let template_values = templates.into_iter().map(Value::String).collect::<Vec<_>>();
        section.insert(
            "source_templates".to_string(),
            Value::Array(template_values),
        );
    }

    if let Some(cmd) = lang_config.run_cmd {
        section.insert("run".to_string(), Value::String(cmd.to_string()));
    }

    if let Some(cmd) = lang_config.compile_cmd {
        section.insert("compile".to_string(), Value::String(cmd.to_string()));
    }

    if let Some(cmd) = lang_config.execute_cmd {
        section.insert("run".to_string(), Value::String(cmd.to_string()));
    }

    if let Some(cmd) = lang_config.after_cmd {
        section.insert("after".to_string(), Value::String(cmd.to_string()));
    }

    config.insert(format!("filetype.{}", filetype), Value::Table(section));
}

// Create general configuration section
fn create_general_section() -> Value {
    let mut general = Table::new();
    general.insert(
        "selenium_browser".to_string(),
        Value::String("chrome".to_string()),
    );
    general.insert(
        "default_filetype".to_string(),
        Value::String("py".to_string()),
    );
    general.insert(
        "editor_command".to_string(),
        Value::String("code $file".to_string()),
    );
    Value::Table(general)
}

// Create workspace configuration section
fn create_workspace_section() -> Value {
    let mut workspace = Table::new();
    workspace.insert(
        "ongoing_dir".to_string(),
        Value::String("problems".to_string()),
    );
    workspace.insert(
        "archive_dir".to_string(),
        Value::String("archives".to_string()),
    );
    workspace.insert("archive".to_string(), Value::Boolean(true));
    Value::Table(workspace)
}
