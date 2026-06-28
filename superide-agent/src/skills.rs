/// A skill is a reusable capability template for the agent.
/// Skills bundle instructions, example usage, and metadata together
/// so the agent can consistently perform complex tasks.
#[derive(Debug, Clone)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: SkillCategory,
    pub instructions: Vec<String>,
    pub tools_required: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillCategory {
    Development,
    Research,
    Debugging,
    CodeReview,
    Refactoring,
    Documentation,
    DevOps,
    Custom(String),
}

impl std::fmt::Display for SkillCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkillCategory::Development => write!(f, "Development"),
            SkillCategory::Research => write!(f, "Research"),
            SkillCategory::Debugging => write!(f, "Debugging"),
            SkillCategory::CodeReview => write!(f, "Code Review"),
            SkillCategory::Refactoring => write!(f, "Refactoring"),
            SkillCategory::Documentation => write!(f, "Documentation"),
            SkillCategory::DevOps => write!(f, "DevOps"),
            SkillCategory::Custom(s) => write!(f, "{}", s),
        }
    }
}

pub struct SkillRegistry {
    skills: Vec<Skill>,
}

impl Default for SkillRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl SkillRegistry {
    pub fn new() -> Self {
        let mut registry = Self { skills: Vec::new() };
        registry.register_builtins();
        registry
    }

    pub fn register(&mut self, skill: Skill) {
        self.skills.push(skill);
    }

    pub fn get(&self, id: &str) -> Option<&Skill> {
        self.skills.iter().find(|s| s.id == id)
    }

    pub fn all(&self) -> &[Skill] {
        &self.skills
    }

    pub fn by_category(&self, category: &SkillCategory) -> Vec<&Skill> {
        self.skills.iter().filter(|s| &s.category == category).collect()
    }

    pub fn search(&self, query: &str) -> Vec<&Skill> {
        let q = query.to_lowercase();
        self.skills
            .iter()
            .filter(|s| {
                s.name.to_lowercase().contains(&q)
                    || s.description.to_lowercase().contains(&q)
                    || s.id.to_lowercase().contains(&q)
            })
            .collect()
    }

    fn register_builtins(&mut self) {
        self.register(Skill {
            id: "rust-dev".to_string(),
            name: "Rust Development".to_string(),
            description: "Expert Rust development: build, test, clippy, and dependency management".to_string(),
            category: SkillCategory::Development,
            instructions: vec![
                "Use `cargo check` for fast compilation checks before full builds".to_string(),
                "Run `cargo clippy` to catch common mistakes and improve code quality".to_string(),
                "Use `cargo test` to run unit tests; add `-- --nocapture` to see output".to_string(),
                "Check `Cargo.toml` for dependency versions before adding new ones".to_string(),
                "Prefer workspace-level commands from the workspace root".to_string(),
            ],
            tools_required: vec!["run_terminal".to_string(), "read_file".to_string(), "write_file".to_string()],
        });

        self.register(Skill {
            id: "git-workflow".to_string(),
            name: "Git Workflow".to_string(),
            description: "Git operations: status, diff, commit, branch management, and history".to_string(),
            category: SkillCategory::Development,
            instructions: vec![
                "Always check `git status` and `git diff` before committing".to_string(),
                "Write clear, concise commit messages summarizing the change".to_string(),
                "Use `git log --oneline -10` to review recent history".to_string(),
                "Prefer small, focused commits over large ones".to_string(),
                "Stash uncommitted changes before switching branches".to_string(),
            ],
            tools_required: vec!["run_terminal".to_string()],
        });

        self.register(Skill {
            id: "web-research".to_string(),
            name: "Web Research".to_string(),
            description: "Search the web for documentation, solutions, and current information".to_string(),
            category: SkillCategory::Research,
            instructions: vec![
                "Use specific, targeted search queries for best results".to_string(),
                "Cross-reference information from multiple sources".to_string(),
                "Prefer official documentation and well-known resources".to_string(),
                "Check the date of information for current relevance".to_string(),
            ],
            tools_required: vec!["web_search".to_string()],
        });

        self.register(Skill {
            id: "debug-rust".to_string(),
            name: "Debug Rust Errors".to_string(),
            description: "Systematic approach to debugging Rust compilation and runtime errors".to_string(),
            category: SkillCategory::Debugging,
            instructions: vec![
                "First reproduce the error with `cargo check` or `cargo build`".to_string(),
                "Read the full compiler error message — it often contains the solution".to_string(),
                "Check for missing imports, type mismatches, and borrow checker issues".to_string(),
                "Use `cargo clippy` for additional diagnostic information".to_string(),
                "For panics, look at the backtrace to identify the source location".to_string(),
            ],
            tools_required: vec!["run_terminal".to_string(), "read_file".to_string(), "web_search".to_string()],
        });

        self.register(Skill {
            id: "code-review".to_string(),
            name: "Code Review".to_string(),
            description: "Review code for correctness, performance, style, and security issues".to_string(),
            category: SkillCategory::CodeReview,
            instructions: vec![
                "Check for correct error handling — unwrap() usage should be justified".to_string(),
                "Look for performance issues: unnecessary allocations, O(n²) patterns".to_string(),
                "Verify that public APIs have documentation comments".to_string(),
                "Check for unsafe code blocks and verify their safety invariants".to_string(),
                "Ensure tests cover edge cases, not just the happy path".to_string(),
            ],
            tools_required: vec!["read_file".to_string(), "search_workspace".to_string()],
        });

        self.register(Skill {
            id: "refactor".to_string(),
            name: "Code Refactoring".to_string(),
            description: "Safely restructure code while maintaining correctness".to_string(),
            category: SkillCategory::Refactoring,
            instructions: vec![
                "Understand the full codebase structure before refactoring".to_string(),
                "Make one logical change at a time to minimize risk".to_string(),
                "Run tests after each significant change to catch regressions early".to_string(),
                "Keep the public API stable unless explicitly asked to change it".to_string(),
                "Update documentation to reflect refactored code".to_string(),
            ],
            tools_required: vec!["read_file".to_string(), "write_file".to_string(), "edit_file".to_string(), "run_terminal".to_string()],
        });

        self.register(Skill {
            id: "create-doc".to_string(),
            name: "Documentation Writing".to_string(),
            description: "Write clear, comprehensive documentation for Rust projects".to_string(),
            category: SkillCategory::Documentation,
            instructions: vec![
                "Use `///` for public API documentation with examples".to_string(),
                "Include a `# Examples` section for each public function".to_string(),
                "Document panics, errors, and safety invariants explicitly".to_string(),
                "Keep README.md up to date with project structure and usage".to_string(),
                "Run `cargo doc --no-deps` to verify documentation builds".to_string(),
            ],
            tools_required: vec!["read_file".to_string(), "write_file".to_string()],
        });

        self.register(Skill {
            id: "devops-docker".to_string(),
            name: "DevOps & Docker".to_string(),
            description: "Docker container management, Dockerfile creation, and deployment".to_string(),
            category: SkillCategory::DevOps,
            instructions: vec![
                "Use multi-stage builds for smaller Docker images".to_string(),
                "Pin base image versions for reproducible builds".to_string(),
                "Keep `.dockerignore` updated to exclude unnecessary files".to_string(),
                "Use `docker compose` for multi-service applications".to_string(),
                "Scan images for vulnerabilities before deployment".to_string(),
            ],
            tools_required: vec!["run_terminal".to_string(), "read_file".to_string(), "write_file".to_string()],
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_skills_registered() {
        let registry = SkillRegistry::new();
        assert!(registry.all().len() >= 7);
    }

    #[test]
    fn test_get_skill_by_id() {
        let registry = SkillRegistry::new();
        let skill = registry.get("rust-dev");
        assert!(skill.is_some());
        assert_eq!(skill.unwrap().name, "Rust Development");
    }

    #[test]
    fn test_search_skills() {
        let registry = SkillRegistry::new();
        let results = registry.search("rust");
        assert!(!results.is_empty());
        assert!(results.iter().any(|s| s.id == "rust-dev"));
    }

    #[test]
    fn test_by_category() {
        let registry = SkillRegistry::new();
        let dev_skills = registry.by_category(&SkillCategory::Development);
        assert!(!dev_skills.is_empty());
    }

    #[test]
    fn test_skill_has_tools() {
        let registry = SkillRegistry::new();
        let skill = registry.get("debug-rust").unwrap();
        assert!(!skill.tools_required.is_empty());
    }
}
