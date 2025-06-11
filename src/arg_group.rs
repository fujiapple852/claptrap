use serde::Deserialize;

#[derive(Debug)]
pub struct NamedArgGroup {
    pub name: String,
    pub arg_group: ArgGroup,
}

impl NamedArgGroup {
    pub fn new(name: String, arg: ArgGroup) -> Self {
        Self {
            name,
            arg_group: arg,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct ArgGroup {
    id: Option<String>,
    args: Option<Vec<String>>,
    multiple: Option<bool>,
    required: Option<bool>,
    requires: Option<String>,
    requires_all: Option<Vec<String>>,
    conflicts_with: Option<String>,
    conflicts_with_all: Option<Vec<String>>,
}

impl From<NamedArgGroup> for clap::ArgGroup {
    fn from(named_arg_group: NamedArgGroup) -> Self {
        let value = named_arg_group.arg_group;
        let mut arg_group = clap::ArgGroup::new(named_arg_group.name);
        if let Some(id) = value.id {
            arg_group = arg_group.id(id);
        }
        if let Some(args) = value.args {
            arg_group = arg_group.args(args);
        }
        if let Some(multiple) = value.multiple {
            arg_group = arg_group.multiple(multiple);
        }
        if let Some(required) = value.required {
            arg_group = arg_group.required(required);
        }
        if let Some(requires) = value.requires {
            arg_group = arg_group.requires(requires);
        }
        if let Some(requires_all) = value.requires_all {
            arg_group = arg_group.requires_all(requires_all);
        }
        if let Some(conflicts_with) = value.conflicts_with {
            arg_group = arg_group.conflicts_with(conflicts_with);
        }
        if let Some(conflicts_with_all) = value.conflicts_with_all {
            arg_group = arg_group.conflicts_with_all(conflicts_with_all);
        }
        arg_group
    }
}
