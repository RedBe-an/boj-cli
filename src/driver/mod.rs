use include_dir::{Dir, include_dir};

/// 빌드 시점에 src/driver/* 를 모두 포함
pub static DRIVER_FILES: Dir = include_dir!("src/driver");
