use std::{env, fs, io, path::PathBuf};
use toml::{Table, Value};

pub fn init() {
    if let Err(err) = init_config() {
        eprintln!("Error initializing configuration: {}", err);
    }
}

fn init_config() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    println!("Current directory: {:?}", current_dir);
    
    let config_dir = current_dir.join(".boj");
    fs::create_dir_all(&config_dir)?;
    
    let config_path = config_dir.join("config.toml");
    let config = create_default_config();
    
    // Convert to TOML string
    let default_config = toml::to_string(&config).expect("Failed to serialize TOML configuration");
    fs::write(config_path, default_config)?;
    
    println!("Created config file at .boj/config.toml");
    Ok(())
}
fn create_default_config() -> Table {
    // Use with_capacity to indicate we'll add several entries
    let mut config = Table::new();

    config.insert("general".to_string(), create_general_section());
    config.insert("workspace".to_string(), create_workspace_section());

    add_filetype_section(&mut config, "py", create_python_config());
    add_filetype_section(&mut config, "rs", create_rust_config());
    add_filetype_section(&mut config, "c", create_c_config());
    add_filetype_section(&mut config, "cpp", create_cpp_config());
    add_filetype_section(&mut config, "java", create_java_config());
    add_filetype_section(&mut config, "js", create_js_config());

    config
}

fn add_filetype_section(config: &mut Table, filetype: &str, section: Table) {
    config.insert(format!("filetype.{}", filetype), Value::Table(section));
}

fn create_general_section() -> Value {
    let mut general = Table::new();
    general.insert("selenium_browser".to_string(), Value::String("chrome".to_string()));
    general.insert("default_filetype".to_string(), Value::String("py".to_string()));
    general.insert("editor_command".to_string(), Value::String("code $file".to_string()));
    Value::Table(general)
}

fn create_workspace_section() -> Value {
    let mut workspace = Table::new();
    workspace.insert("ongoing_dir".to_string(), Value::String("problems".to_string()));
    workspace.insert("archive_dir".to_string(), Value::String("archives".to_string()));
    workspace.insert("archive".to_string(), Value::Boolean(true));
    Value::Table(workspace)
}

fn create_python_config() -> Table {
    let mut config = Table::new();
    config.insert("language".to_string(), Value::String("python3".to_string()));
    config.insert("main".to_string(), Value::String("main.py".to_string()));
    config.insert("run".to_string(), Value::String("python3 $file".to_string()));
    config
}

fn create_cpp_config() -> Table {
    let mut config = Table::new();
    config.insert("language".to_string(), Value::String("c++17".to_string()));
    config.insert("main".to_string(), Value::String("main.cpp".to_string()));
    config.insert(
        "source_templates".to_string(),
        Value::Array(vec![Value::String("default.cpp".to_string())]),
    );
    config.insert(
        "root_templates".to_string(),
        Value::Array(vec![Value::String("compile_flags.txt".to_string())]),
    );
    config.insert("compile".to_string(), Value::String("g++ -std=c++17 $file".to_string()));
    config.insert("run".to_string(), Value::String("./a.out".to_string()));
    config.insert("after".to_string(), Value::String("rm -rf a.out".to_string()));
    config
}

fn create_c_config() -> Table {
    let mut config = Table::new();
    config.insert("language".to_string(), Value::String("c".to_string()));
    config.insert("main".to_string(), Value::String("main.c".to_string()));
    config.insert("compile".to_string(), Value::String("gcc -std=c11 $file -o a.out".to_string()));
    config.insert("run".to_string(), Value::String("./a.out".to_string()));
    config.insert("after".to_string(), Value::String("rm -rf a.out".to_string()));
    config
}

fn create_java_config() -> Table {
    let mut config = Table::new();
    config.insert("language".to_string(), Value::String("java".to_string()));
    config.insert("main".to_string(), Value::String("Main.java".to_string()));
    config.insert("compile".to_string(), Value::String("javac $file".to_string()));
    config.insert("run".to_string(), Value::String("java Main".to_string()));
    config
}

fn create_js_config() -> Table {
    let mut config = Table::new();
    config.insert("language".to_string(), Value::String("node".to_string()));
    config.insert("main".to_string(), Value::String("main.js".to_string()));
    config.insert("run".to_string(), Value::String("node $file".to_string()));
    config
}

fn create_rust_config() -> Table {
    let mut config = Table::new();
    config.insert("language".to_string(), Value::String("rust".to_string()));
    config.insert("main".to_string(), Value::String("main.rs".to_string()));
    config.insert("compile".to_string(), Value::String("rustc $file -o main".to_string()));
    config.insert("run".to_string(), Value::String("./main".to_string()));
    config
}
