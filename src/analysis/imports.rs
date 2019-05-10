use std::collections::btree_map::{BTreeMap, Iter};

use super::namespaces;
use crate::{library::Library, nameutil, version::Version};

/// Provides assistance in generating use declarations.
///
/// It takes into account that use declaration referring to names within the
/// same crate will look differently. It also avoids generating spurious
/// declarations referring to names from within the same module as the one we
/// are generating code for.
#[derive(Clone, Debug, Default)]
pub struct Imports {
    /// Name of the current crate.
    crate_name: String,
    /// Name defined within current module. It doesn't need use declaration.
    ///
    /// NOTE: Currently we don't need to support more than one such name.
    defined: Option<String>,
    map: BTreeMap<String, (Option<Version>, Vec<String>)>,
}

impl Imports {
    pub fn new(gir: &Library) -> Imports {
        let crate_name = nameutil::crate_name(&gir.namespace(namespaces::MAIN).name);
        let crate_name = nameutil::higher_crate_name(&crate_name).into();
        Imports {
            crate_name,
            defined: None,
            map: BTreeMap::new(),
        }
    }

    pub fn with_defined(gir: &Library, name: &str) -> Imports {
        let mut imports = Imports::new(gir);
        imports.defined = Some(imports.make_local_name(name));
        imports
    }

    fn make_local_name(&self, name: &str) -> String {
        format!("{}::{}", self.crate_name, name)
    }

    /// Declares that name should be available through its last path component.
    ///
    /// For example, if name is `X::Y::Z` then it will be available as `Z`.
    pub fn add(&mut self, name: &str, version: Option<Version>) {
        if let Some(ref defined) = self.defined {
            if name == defined {
                return;
            }
        }
        if let Some(name) = self.strip_crate_name(name) {
            let entry = self.map.entry(name).or_insert((version, Vec::new()));
            if version < entry.0 {
                *entry = (version, Vec::new());
            } else {
                *entry = (entry.0, Vec::new());
            }
        }
    }

    /// Declares that name should be available through its last path component and provides
    /// an optional feature constraint.
    ///
    /// For example, if name is `X::Y::Z` then it will be available as `Z`.
    pub fn add_with_constraint(
        &mut self,
        name: &str,
        version: Option<Version>,
        constraint: Option<&str>,
    ) {
        if let Some(ref defined) = self.defined {
            if name == defined {
                return;
            }
        }
        if let Some(name) = self.strip_crate_name(name) {
            let entry = self.map.entry(name).or_insert((version, Vec::new()));
            if version < entry.0 {
                *entry = (version, Vec::new());
            } else {
                *entry = (entry.0, Vec::new());
            }

            if let Some(constraint) = constraint {
                let constraint = String::from(constraint);
                if !entry.1.contains(&constraint) {
                    entry.1.push(constraint);
                }
            }
        }
    }

    pub fn add_used_type(&mut self, used_type: &str, version: Option<Version>) {
        if used_type.find("::").is_none() {
            self.add(&self.make_local_name(used_type), version);
        }
    }

    pub fn add_used_types(&mut self, used_types: &[String], version: Option<Version>) {
        for s in used_types {
            self.add_used_type(s, version);
        }
    }

    /// Tries to strip crate name prefix from given name.
    ///
    /// Returns `None` if name matches crate name exactly. Otherwise returns name with crate name
    /// prefix replaced with `crate` or full name if there was no match.
    fn strip_crate_name<'a>(&self, name: &'a str) -> Option<String> {
        let prefix = &self.crate_name;
        match name.find("::") {
            Some(i) if &name[..i] == prefix => Some(format!("crate{}", &name[i..])),
            None if name == prefix => None,
            _ => Some(name.into()),
        }
    }

    pub fn iter(&self) -> Iter<String, (Option<Version>, Vec<String>)> {
        self.map.iter()
    }
}
