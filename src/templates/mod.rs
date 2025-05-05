use include_dir::{Dir, include_dir};

/// 빌드 시점에 src/templates/* 를 모두 포함
pub static TEMPLATES: Dir = include_dir!("src/templates");
