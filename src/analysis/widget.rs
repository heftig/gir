use env::Env;
use gobjects::*;
use library;
use nameutil::*;
use super::*;
use super::type_kind::*;

pub struct Info {
    pub full_name: String,
    pub class_tid: library::TypeId,
    pub kind: type_kind::TypeKind,
    pub name: String,
    pub parents: Vec<general::StatusedTypeId>,
    pub has_ignored_parents: bool,
}

impl Info {
    //TODO: add test in tests/ for panic
    pub fn type_<'a>(&self, library: &'a library::Library) -> &'a library::Class {
        let type_ = library.type_(self.class_tid).as_class()
            .unwrap_or_else(|| panic!("{} is not a class.", self.full_name));
        type_
    }
}

pub fn new(env: &Env, obj: &GObject) -> Info {
    let full_name = obj.name.clone();

    let class_tid = env.library.find_type_unwrapped(0, &full_name, "Class");

    let type_ = env.library.type_(class_tid);
    let kind = type_.to_type_kind(&env.library);
    assert_eq!(kind, TypeKind::Widget);

    let name = split_namespace_name(&full_name).1.into();

    let klass = type_.to_class();
    let (parents, has_ignored_parents) = parents::analyze(env, klass);

    Info {
        full_name: full_name,
        class_tid: class_tid,
        kind: kind,
        name: name,
        parents: parents,
        has_ignored_parents: has_ignored_parents,
    }
}