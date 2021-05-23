use crate::intermediate::Positions;

#[derive(Debug, Clone)]
pub struct DebugInfo {
    pub functions: Vec<Positions>
}

impl DebugInfo {
    pub fn new() -> DebugInfo {
        DebugInfo {
            functions: Vec::new()
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    // store filename
    pub filenames: Vec<String>,
    // store model as which file (index)
    pub model_files: Vec<usize>,
    pub model_names: Vec<String>,
    // store function at which file (index)
    pub function_files: Vec<usize>,
    pub function_names: Vec<String>
}

impl FileInfo {
    pub fn new() -> FileInfo {
        FileInfo {
            filenames: Vec::new(),
            model_files: Vec::new(),
            model_names: Vec::new(),
            function_files: Vec::new(),
            function_names: Vec::new()
        }
    }
}