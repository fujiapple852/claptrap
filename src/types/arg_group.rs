use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct ArgGroup {
    #[serde(skip)]
    pub(crate) id: String,
    args: Option<Vec<String>>,
    multiple: Option<bool>,
    required: Option<bool>,
    requires: Option<String>,
    requires_all: Option<Vec<String>>,
    conflicts_with: Option<String>,
    conflicts_with_all: Option<Vec<String>>,
}

impl From<ArgGroup> for clap::ArgGroup {
    fn from(value: ArgGroup) -> Self {
        let mut arg_group = Self::new(value.id);
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
