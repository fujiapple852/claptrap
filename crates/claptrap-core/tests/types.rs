use clap::builder::StyledStr;
use claptrap_core::Command;

#[test]
fn it_can_deserialize_command_and_convert() {
    let spec = include_str!("resources/types/fullspec.toml");
    let cmd: Command = toml::from_str(spec).expect("could not deserialize spec");
    let clap_cmd = clap::Command::from(cmd);
    assert_eq!(clap_cmd.get_name(), "fullspec");
    assert_eq!(clap_cmd.get_version(), Some("1.0.0"));
    assert_eq!(clap_cmd.get_long_version(), Some("1.0.0 (build abcdef)"));
    assert_eq!(clap_cmd.get_author(), Some("Example <example@example.com>"));
    assert_eq!(
        clap_cmd.get_about(),
        Some(&StyledStr::from("Example spec with every field"))
    );
    assert_eq!(
        clap_cmd.get_long_about(),
        Some(&StyledStr::from("Detailed description"))
    );
    assert_eq!(clap_cmd.get_bin_name(), Some("fullspec"));
    assert_eq!(clap_cmd.get_display_name(), Some("Full Spec CLI"));
    assert_eq!(clap_cmd.get_after_help(), Some(&StyledStr::from("after")));
    assert_eq!(
        clap_cmd.get_after_long_help(),
        Some(&StyledStr::from("after long"))
    );
    assert_eq!(clap_cmd.get_before_help(), Some(&StyledStr::from("before")));
    assert_eq!(
        clap_cmd.get_before_long_help(),
        Some(&StyledStr::from("before long"))
    );
}
