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
    pub higher_crate_name: String,
    pub package_name: Option<String>,
    pub shared_libs: Vec<String>,
    pub versions: Vec<Version>,
}

#[derive(Debug)]
pub struct Info {
    namespaces: Vec<Namespace>,
    pub is_glib_crate: bool,
    pub glib_ns_id: NsId,
}

impl Info {
    pub fn main(&self) -> &Namespace {
        &self[MAIN]
    }
}

impl Index<NsId> for Info {
    type Output = Namespace;

    fn index(&self, index: NsId) -> &Namespace {
        &self.namespaces[index as usize]
    }
}

pub fn run(gir: &library::Library) -> Info {
    let mut namespaces = Vec::with_capacity(gir.namespaces.len());
    let mut is_glib_crate = false;
    let mut glib_ns_id = None;

    for (ns_id, ns) in gir.namespaces.iter().enumerate() {
        let ns_id = ns_id as NsId;
        let crate_name = nameutil::crate_name(&ns.name);
        let sys_crate_name = format!("{}_sys", crate_name);
        let higher_crate_name = match &crate_name[..] {
            "gobject" => "glib".to_owned(),
            _ => crate_name.clone(),
        };
        namespaces.push(Namespace {
            name: ns.name.clone(),
            crate_name,
            sys_crate_name,
            higher_crate_name,
            package_name: ns.package_name.clone(),
            shared_libs: ns.shared_library.clone(),
            versions: ns.versions.iter().cloned().collect(),
        });
        if ns.name == "GLib" {
            glib_ns_id = Some(ns_id);
            if ns_id == MAIN {
                is_glib_crate = true;
            }
        } else if ns.name == "GObject" && ns_id == MAIN {
            is_glib_crate = true;
        }
    }

    Info {
        namespaces,
        is_glib_crate,
        glib_ns_id: glib_ns_id.expect("Missing `GLib` namespace!"),
    }
}
