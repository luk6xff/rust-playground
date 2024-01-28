#[derive(Debug)]
pub enum Language {
    Rust,
    Java,
    Perl,
}

#[derive(Clone, Debug)]
pub struct Dependency {
    pub name: String,
    pub version_expression: String,
}

/// A representation of a software package.
#[derive(Debug, Default)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub dependencies: Vec<Dependency>,
    pub language: Option<Language>,
}

impl Package {
    /// Return a representation of this package as a dependency, for use in
    /// building other packages.
    pub fn as_dependency(&self) -> Dependency {
        Dependency {
            name: self.name.clone(),
            version_expression: format!("^{}", self.version),
        }
    }
}

/// A builder for a Package. Use `build()` to create the `Package` itself.
pub struct PackageBuilder(Package);

impl PackageBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        PackageBuilder(Package {
            name: name.into(),
            ..Package::default()
        })
    }

    /// Set the package version.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.0.version = version.into();
        self
    }

    /// Set the package authors.
    pub fn authors(mut self, authors: Vec<String>) -> Self {
        self.0.authors = authors;
        self
    }

    /// Add an additional dependency.
    pub fn dependency(mut self, dependency: Dependency) -> Self {
        self.0.dependencies.push(dependency);
        self
    }

    /// Set the language. If not set, language defaults to None.
    pub fn language(mut self, language: Language) -> Self {
        self.0.language = Some(language);
        self
    }

    pub fn build(self) -> Package {
        self.0
    }
}
