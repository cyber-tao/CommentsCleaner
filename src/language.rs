use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    C,
    Cpp,
    Java,
    JavaScript,
    TypeScript,
    Python,
    Html,
    Css,
}

impl Language {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "c" | "h" => Some(Language::C),
            "cpp" | "cc" | "cxx" | "hpp" | "hxx" => Some(Language::Cpp),
            "java" => Some(Language::Java),
            "js" | "jsx" => Some(Language::JavaScript),
            "ts" | "tsx" => Some(Language::TypeScript),
            "py" | "pyw" => Some(Language::Python),
            "html" | "htm" => Some(Language::Html),
            "css" => Some(Language::Css),
            _ => None,
        }
    }

    pub fn from_path(path: &Path) -> Option<Self> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(Self::from_extension)
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "c" => Some(Language::C),
            "cpp" | "c++" => Some(Language::Cpp),
            "java" => Some(Language::Java),
            "js" | "javascript" => Some(Language::JavaScript),
            "ts" | "typescript" => Some(Language::TypeScript),
            "python" | "py" => Some(Language::Python),
            "html" => Some(Language::Html),
            "css" => Some(Language::Css),
            _ => None,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Language::C => "C",
            Language::Cpp => "C++",
            Language::Java => "Java",
            Language::JavaScript => "JavaScript",
            Language::TypeScript => "TypeScript",
            Language::Python => "Python",
            Language::Html => "HTML",
            Language::Css => "CSS",
        }
    }
}
