use std::ops::Index;

use crate::{library, nameutil, version::Version};

pub type NsId = u16;
pub const MAIN: NsId = library::MAIN_NAMESPACE;
pub const INTERNAL: NsId = library::INTERNAL_NAMESPACE;

#[derive(Debug)]
pub struct Namespace {
    pub name: String,
    pub crate_name: String,
    pub sys_crate_name: String,
    pub package_name: Option<String>,
    pub shared_libs: Vec<String>,
    pub versions: Vec<Version>,
    pub is_main: bool,
}

impl Namespace {
    pub fn higher_crate_name(&self) -> &str {
        nameutil::higher_crate_name(&self.crate_name)
    }

    pub fn local_crate_name(&self) -> &str {
        if self.is_main {
            "crate"
        } else {
            self.higher_crate_name()
        }
    }
}

#[derive(Debug)]
pub struct Info {
    namespaces: Vec<Namespace>,
    glib_ns_id: NsId,
    pub gstring_name: String,
}

impl Info {
    pub fn main(&self) -> &Namespace {
        &self[MAIN]
    }

    pub fn glib(&self) -> &Namespace {
        &self[self.glib_ns_id]
    }
}

impl Index<NsId> for Info {
    type Output = Namespace;

    fn index(&self, index: NsId) -> &Namespace {
        &self.namespaces[index as usize]
    }
}

pub fn run(gir: &library::Library) -> Info {
    let mut glib_ns_id = None;

    let namespaces: Vec<_> = gir
        .namespaces
        .iter()
        .enumerate()
        .map(|(ns_id, ns)| {
            let ns_id = ns_id as NsId;
            let name = ns.name.clone();
            let crate_name = nameutil::crate_name(&name);
            let sys_crate_name = format!("{}_sys", crate_name);

            if name == "GLib" {
                glib_ns_id = Some(ns_id);
            }

            Namespace {
                name,
                crate_name,
                sys_crate_name,
                package_name: ns.package_name.clone(),
                shared_libs: ns.shared_library.clone(),
                versions: ns.versions.iter().cloned().collect(),
                is_main: ns_id == MAIN,
            }
        })
        .collect();

    let glib_ns_id = glib_ns_id.expect("Missing `GLib` namespace!");
    let gstring_name = format!(
        "{}::GString",
        namespaces[glib_ns_id as usize].local_crate_name()
    );

    Info {
        namespaces,
        glib_ns_id,
        gstring_name,
    }
}
