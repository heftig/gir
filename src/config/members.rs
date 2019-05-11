use toml::Value;

use super::{error::TomlHelper, ident::Ident, parsable::Parse};
use crate::version::Version;

#[derive(Clone, Debug)]
pub struct Member {
    pub ident: Ident,
    // some enum variants have multiple names
    pub alias: bool,
    pub version: Option<Version>,
    pub deprecated_version: Option<Version>,
    pub ignore: bool,
}

impl Parse for Member {
    fn parse(toml: &Value, object_name: &str) -> Option<Member> {
        let ident = match Ident::parse(toml, object_name, "member") {
            Some(ident) => ident,
            None => {
                error!(
                    "No 'name' or 'pattern' given for member for object {}",
                    object_name
                );
                return None;
            }
        };

        toml.check_unwanted(
            &["alias", "version", "name", "pattern", "ignore"],
            &format!("member {}", object_name),
        );

        let alias = toml
            .lookup("alias")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let version = toml
            .lookup("version")
            .and_then(Value::as_str)
            .and_then(|s| s.parse().ok());
        let deprecated_version = toml
            .lookup("deprecated_version")
            .and_then(Value::as_str)
            .and_then(|s| s.parse().ok());

        let ignore = toml
            .lookup("ignore")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        Some(Member {
            ident,
            alias,
            version,
            deprecated_version,
            ignore,
        })
    }
}

impl AsRef<Ident> for Member {
    fn as_ref(&self) -> &Ident {
        &self.ident
    }
}

pub type Members = Vec<Member>;

#[cfg(test)]
mod tests {
    use super::{
        super::{ident::Ident, parsable::Parse},
        *,
    };
    use crate::version::Version;

    fn toml(input: &str) -> ::toml::Value {
        let value = ::toml::from_str(&input);
        assert!(value.is_ok());
        value.unwrap()
    }

    #[test]
    fn member_parse_alias() {
        let toml = toml(
            r#"
name = "name1"
alias = true
"#,
        );
        let f = Member::parse(&toml, "a").unwrap();
        assert_eq!(f.ident, Ident::Name("name1".into()));
        assert_eq!(f.alias, true);
    }

    #[test]
    fn member_parse_version_default() {
        let toml = toml(
            r#"
name = "name1"
"#,
        );
        let f = Member::parse(&toml, "a").unwrap();
        assert_eq!(f.version, None);
    }

    #[test]
    fn member_parse_version() {
        let toml = toml(
            r#"
name = "name1"
version = "3.20"
"#,
        );
        let f = Member::parse(&toml, "a").unwrap();
        assert_eq!(f.version, Some(Version::Full(3, 20, 0)));
    }
}
