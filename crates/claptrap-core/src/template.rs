use crate::clap_ext::IsManyEx;
use crate::output::{PREFIX, SUBCOMMAND_PATH_SEPARATOR, SUBCOMMAND_VALUE_SEPARATOR};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::iter;

#[must_use]
pub fn template_values(clap_cmd: &clap::Command) -> TemplateValues {
    let args = clap_cmd
        .get_arguments()
        .map(|arg| Arg {
            name: format!("{}_{}", PREFIX, arg.get_id()),
            multiple: arg.is_many(),
        })
        .collect::<Vec<_>>();
    let subcommands = flatten(clap_cmd, &[]);
    TemplateValues { args, subcommands }
}

fn flatten(cmd: &clap::Command, prefix: &[String]) -> Vec<Subcommand> {
    let subs: Vec<Subcommand> = cmd
        .get_subcommands()
        .map(|sub| {
            let sub_path = prefix
                .iter()
                .chain(iter::once(&sub.to_string()))
                .join(SUBCOMMAND_PATH_SEPARATOR);
            let sub_value = prefix
                .iter()
                .chain(iter::once(&sub.to_string()))
                .join(SUBCOMMAND_VALUE_SEPARATOR);
            Subcommand {
                name: sub_value,
                args: sub
                    .get_arguments()
                    .map(|arg| Arg {
                        name: format!("{}_{}_{}", PREFIX, sub_path, arg.get_id()),
                        multiple: arg.is_many(),
                    })
                    .collect(),
            }
        })
        .collect();
    cmd.get_subcommands().fold(subs, |mut acc, sub| {
        let mut child_prefix = prefix.to_vec();
        child_prefix.push(sub.get_name().to_owned());
        let child_map = flatten(sub, &child_prefix);
        acc.extend(child_map);
        acc
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateValues {
    pub args: Vec<Arg>,
    pub subcommands: Vec<Subcommand>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subcommand {
    pub name: String,
    pub args: Vec<Arg>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Arg {
    pub name: String,
    pub multiple: bool,
}
