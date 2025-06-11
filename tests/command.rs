use claptrap::command::Command;
use claptrap::parse;
use std::ffi::OsString;

#[test]
fn test_no_args() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
        "#,
    )
    .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_arg() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [args]
            debug = { short = 'd', help = "turns on debugging mode" }
            config = { short = 'c', long = "config", value-name = "CONFIG", help = "Optionally sets a config file to use" }
        "#,
    )
        .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_group() {
    let app: Command = toml::from_str(
        r#"
            name = "cmd"
            [groups]
            vers = { args = ["set-ver", "major", "minor", "patch"], required = true }
            [args]
            set-ver = { long = "set-ver", help = "set the version manually", required = false }
            major = { long = "major", help = "auto increase major" }
            minor = { long = "minor", help = "auto increase minor" }
            patch = { long = "patch", help = "auto increase patch" }
        "#,
    )
    .unwrap();
    let input = "--major 1 --minor 2 --patch 3";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_groups() {
    let app: Command = toml::from_str(
        r#"
            name = "cmd"
            [groups]
            vers = { args = ["set-ver", "major", "minor", "patch"], required = true }
            input = { args = ["config", "iface"] }
            [args]
            set-ver = { long = "set-ver", help = "set the version manually", required = false }
            major = { long = "major", help = "auto increase major" }
            minor = { long = "minor", help = "auto increase minor" }
            patch = { long = "patch", help = "auto increase patch" }
            config = { short = 'c', help = "a config file", required = false }
            iface = { short = 'i', help = "an interface", required = false }
        "#,
    )
    .unwrap();
    let input = "--major 1 -i eth0";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_subcommand() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [args]
            arg1 = {}
            [[subcommands]]
            name = "subcommand"
            about = "a sub command"            
                [subcommands.args]
                arg2 = {}
                  [[subcommands.subcommands]]
                  name  = "nested"
                  about = "A nested sub command"
                    [subcommands.subcommands.args]
                    arg3 = { long = "arg3" }
            [[subcommands]]
            name = "subcommand2"
            about = "another sub command"
                [subcommands.args]
                arg4 = {}
        "#,
    )
    .unwrap();
    let input = "subcommand nested --arg3 value";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

// TODO: error

#[test]
fn test_ignore_error() {
    let app: Command = toml::from_str(
        r#"
            name = "cmd"
            ignore-errors = true
            [args]
            config = { short = 'c', long = "config", value-name = "FILE", help = "Sets a custom config file" }
            stuff = { short = 's', long = "stuff", value-name = "FILE", help = "Sets a custom stuff file" }
            f = { short = 'f', help = "Flag" }
        "#,
    )
        .unwrap();
    let input = "-c file -f -x";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_args_override_self() {
    let app: Command = toml::from_str(
        r#"
            name = "cmd"
            args-override-self = true
            [args]
            foo = { long = "foo" }
            bar = { long = "bar" }
        "#,
    )
    .unwrap();
    let input = "--foo value --foo value";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

// TODO not clear what dont-delimit-trailing-values is doing here, changing it does not change the output, maybe a bad test case?
#[test]
fn test_dont_delimit_trailing_values() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            dont-delimit-trailing-values = true
            [args]
            cmd = {}
            rest = { trailing-var-arg = true, num-args = "1.." }
        "#,
    )
    .unwrap();
    let input = "cmd foo -- arg1 -r val1";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

/// TODO doesn't seem to disable color output
#[test]
fn test_color() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            color = "never"
            [args]
            cfg = { long = "config" }
        "#,
    )
    .unwrap();
    let input = "--cfg config";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

// TODO styles

#[test]
fn test_term_width() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            term-width = 80
            [args]
            cfg = { long = "config", help = "This is a very long help message that is longer than 80 characters and so will wrap" }
        "#,
    )
        .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_max_term_width() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            max-term-width = 80
            [args]
            cfg = { long = "config", help = "This is a very long help message that is longer than 80 characters and so will wrap" }
        "#,
    )
        .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_diable_version_flag() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            version = "1.0.0"
            disable-version-flag = true
            [args]
            version = { long = "version", action = "version", help = "Print version" }
        "#,
    )
    .unwrap();

    let input1 = "-V";
    let args1: Vec<OsString> = input1.split(" ").map(OsString::from).collect();
    let output1 = parse(app.clone(), args1);
    insta::assert_snapshot!(output1);

    let input2 = "--version";
    let args2: Vec<OsString> = input2.split(" ").map(OsString::from).collect();
    let output2 = parse(app, args2);
    insta::assert_snapshot!(output2);
}

#[test]
fn test_propagate_version() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            version = "v1.1"
            propagate-version = true
            [[subcommands]]
            name = "test"
        "#,
    )
    .unwrap();
    let input = "test --version";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_next_line_help() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            next-line-help = true
            [args]
            opt = { short = 'o', long = "long-option-flag", action = "set", value-names = ["value1", "value2"], help = "Some really long help and complex\nhelp that makes more sense to be\non a line after the option" }
        "#,
    )
        .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_diable_help_flag() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            disable-help-flag = true
            [args]
            help = { long = "help", action = "help", help = "Print version" }
        "#,
    )
    .unwrap();

    let input1 = "-h";
    let args1: Vec<OsString> = input1.split(" ").map(OsString::from).collect();
    let output1 = parse(app.clone(), args1);
    insta::assert_snapshot!(output1);

    let input2 = "--help";
    let args2: Vec<OsString> = input2.split(" ").map(OsString::from).collect();
    let output2 = parse(app, args2);
    insta::assert_snapshot!(output2);
}

#[test]
fn test_disable_help_subcommand() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            disable-help-subcommand = true
            [[subcommands]]
            name = "test"
        "#,
    )
    .unwrap();
    let input = "help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

/// TODO doesn't seem to disable color output
#[test]
fn test_disable_colored_help() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            disable-colored-help = true
            [args]
            cfg = { long = "config" }
        "#,
    )
    .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_help_expected() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            help-expected = true
            [args]
            foo = { help = "It does foo stuff" }
        "#,
    )
    .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_hide_possible_values() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            hide-possible-values = true
            [args]
            mode = { long = "mode", value-parser = ["fast", "slow"], action = "set" }
        "#,
    )
    .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_infer_long_args() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            infer-long-args = true
            [args]
            test = { long = "test", action = "set-true" }
        "#,
    )
    .unwrap();
    let input = "--te";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_infer_long_args_ambiguous() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            infer-long-args = true
            [args]
            test = { long = "test", action = "set-true" }
            temp = { long = "temp", action = "set-true" }
        "#,
    )
    .unwrap();
    let input = "--te";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_infer_subcommands() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            infer-subcommands = true
            [[subcommands]]
            name = "test"
            [subcommands.args]
            arg1 = { long = "arg1", action = "set-true" }
        "#,
    )
    .unwrap();
    let input = "te --arg1";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_bin_name() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            bin-name = "my_binary"
            [args]
        "#,
    )
    .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

// TODO this does not seem to change the output, maybe a bad test case?
#[test]
fn test_display_name() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            display-name = "my_program"
            [args]
            cfg = { long = "config" }
        "#,
    )
    .unwrap();
    let input = "--cfg";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_author() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            author = "Me, me@mymain.com"
            help-template = "{name} ({version}) {author} - {usage}"
            [args]
        "#,
    )
    .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_about() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            about = "Does really amazing things for great people"
            [args]
        "#,
    )
    .unwrap();
    let input = "-h";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
#[ignore]
fn test_long_about() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            long-about = "Does really amazing things to great people. Now let's talk a little more in depth about how this subcommand really works. It may take about a few lines of text, but that's ok!"
            [args]
        "#,
    )
        .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_after_help() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            after-help = "Does really amazing things for great people... but be careful with -R!"
            [args]
        "#,
    )
    .unwrap();
    let input = "-h";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
#[ignore]
fn test_after_long_help() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            after-long-help = "Does really amazing things to great people... but be careful with -R, like, for real, be careful with this!"
            [args]
        "#,
    )
        .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_before_help() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            before-help = "Some info I'd like to appear before the help info"
            [args]
        "#,
    )
    .unwrap();
    let input = "-h";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_before_long_help() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            before-long-help = "Some verbose and long info I'd like to appear before the help info"
            [args]
        "#,
    )
    .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_version() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            version = "v0.1.24"
            [args]
        "#,
    )
    .unwrap();
    let input = "-V";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_long_version() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            long-version = "v0.1.24 commit: abcdef89726d\nrevision: 123\nrelease: 2\nbinary: myprog"
            [args]
        "#,
    )
    .unwrap();
    let input = "--version";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_override_usage() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            override-usage = "myapp -X [-a] [-b] <file>\n                   myapp -Y [-c] <file1> <file2>\n                   myapp -Z [-d|-e]"
            [args]
        "#,
    )
        .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_override_help() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            override-help = "myapp v1.0\n           Does awesome things\n           (C) me@mail.com\n\n           Usage: myapp <opts> <command>\n\n           Options:\n           -h, --help       Display this message\n           -V, --version    Display version info\n           -s <stuff>       Do something with stuff\n           -v               Be verbose\n\n           Commands:\n           help             Print this message\n           work             Do some work"
            [args]
        "#,
    )
        .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_template() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            help-template = "{before-help}{name} {version}\n{author-with-newline}{about-with-newline}\n{usage-heading} {usage}\n\n{all-args}{after-help}"
            [args]
        "#,
    )
        .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_flatten_help() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            flatten-help = true
            [args]
            arg1 = {}
            [[subcommands]]
            name = "subcommand"
            about = "a sub command"
            flatten-help = true
                [subcommands.args]
                arg2 = {}
                  [[subcommands.subcommands]]
                  name  = "nested"
                  about = "A nested sub command"
                    [subcommands.subcommands.args]
                    arg3 = { long = "arg3" }
            [[subcommands]]
            name = "subcommand2"
            about = "another sub command"
                [subcommands.args]
                arg4 = {}
        "#,
    )
    .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

// TODO does not seem to change the output, maybe a bad test case?
#[test]
fn test_next_help_heading() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            next-help-heading = "Configuration Options"
            [args]
            cfg = { long = "config", help = "Some help text describing the --config arg" }
        "#,
    )
    .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_next_display_order() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            next-display-order = 200
            [args]
            custom-help = { short = '?', action = "help", display-order = 100, help = "Alt help" }
            airplane = { short = 'a', long = "airplane", action = "set", help = "I should be first!" }
        "#,
    )
        .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

// TODO arg_required_else_help

#[test]
fn test_allow_missing_positional() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            allow-missing-positional = true
            [args]
            arg1 = {}
            arg2 = { required = true }
        "#,
    )
    .unwrap();
    let input = "other";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_allow_missing_positional_with_default() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            allow-missing-positional = true
            [args]
            arg1 = { default-value = "something" }
            arg2 = { required = true }
        "#,
    )
    .unwrap();
    let input = "other";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_allow_missing_positional_style_2() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            allow-missing-positional = true
            [args]
            foo = {}
            bar = {}
            baz = { action = "set", num-args = "1.." }
        "#,
    )
    .unwrap();

    let input1 = "foo bar baz1 baz2 baz3";
    let args1: Vec<OsString> = input1.split(" ").map(OsString::from).collect();
    let output1 = parse(app.clone(), args1);
    insta::assert_snapshot!(output1);

    let input2 = "-- baz1 baz2 baz3";
    let args2: Vec<OsString> = input2.split(" ").map(OsString::from).collect();
    let output2 = parse(app, args2);
    insta::assert_snapshot!(output2);
}

#[test]
fn test_short_flag() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [[subcommands]]
            name = "sync"
            short-flag = 'S'
            [subcommands.args]
            search = { short = 's', long = "search", action = "set-true", help = "search remote repositories for matching stringsg" }
        "#,
    )
        .unwrap();
    let input = "-Ss";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_long_flag() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [[subcommands]]
            name = "sync"
            long-flag = "sync"
            [subcommands.args]
            search = { short = 's', long = "search", action = "set-true", help = "search remote repositories for matching stringsg" }
        "#,
    )
        .unwrap();
    let input = "--sync --search";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_alias() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [[subcommands]]
            name = "test"
            alias = "do-stuff"
            [subcommands.args]
            search = { short = 's', long = "search", action = "set-true" }
        "#,
    )
    .unwrap();
    let input = "do-stuff -s";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_short_flag_alias() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [[subcommands]]
            name = "test"
            short-flag = 't'
            short-flag-alias = 'd'
            [subcommands.args]
            search = { short = 's', long = "search", action = "set-true" }
        "#,
    )
    .unwrap();
    let input = "-d --search";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_long_flag_alias() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [[subcommands]]
            name = "test"
            long-flag = "test"
            long-flag-alias = "testing"
            [subcommands.args]
            search = { short = 's', long = "search", action = "set-true" }
        "#,
    )
    .unwrap();
    let input = "--testing -s";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_aliases() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [[subcommands]]
            name = "test"
            aliases = ["do-stuff", "do-tests", "tests"]
            [subcommands.args]
            input = { required = false, help = "the file to add" }
        "#,
    )
    .unwrap();
    let input = "do-tests";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_short_flag_aliases() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [[subcommands]]
            name = "test"
            short-flag-aliases = ['a', 'b', 'c']
            [subcommands.args]
            input = { required = false, help = "the file to add" }
        "#,
    )
    .unwrap();
    let input = "-a";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_long_flag_aliases() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [[subcommands]]
            name = "test"
            long-flag = "test"
            long-flag-aliases = ["testing", "testall", "test_all"]
            [subcommands.args]
            input = { required = false, help = "the file to add" }
        "#,
    )
    .unwrap();
    let input = "--testing";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_visible_alias() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [[subcommands]]
            name = "test"
            visible-alias = "do-stuff"
            [subcommands.args]
            input = { required = false, help = "the file to add" }
        "#,
    )
    .unwrap();
    let input = "do-stuff file.txt";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_visible_short_flag_alias() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [[subcommands]]
            name = "test"
            short-flag = 't'
            visible-short-flag-alias = 'd'
            [subcommands.args]
            input = { required = false, help = "the file to add" }
        "#,
    )
    .unwrap();
    let input = "-d file.txt";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_visible_long_flag_alias() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [[subcommands]]
            name = "test"
            long-flag = "test"
            visible-long-flag-alias = "testing"
            [subcommands.args]
            input = { required = false, help = "the file to add" }
        "#,
    )
    .unwrap();
    let input = "--testing file.txt";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_visible_aliases() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [[subcommands]]
            name = "test"
            visible-aliases = ["do-stuff", "tests"]
            [subcommands.args]
            input = { required = false, help = "the file to add" }
        "#,
    )
    .unwrap();
    let input = "do-stuff config.txt";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_visible_short_flag_aliases() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [[subcommands]]
            name = "test"
            visible-short-flag-aliases = ['a', 'b', 'c']
            [subcommands.args]
            input = { required = false, help = "the file to add" }
        "#,
    )
    .unwrap();
    let input = "-a file.txt";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_visible_long_flag_aliases() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [[subcommands]]
            name = "test"
            long-flag = "test"
            visible-long-flag-aliases = ["testing", "testall", "test_all"]
            [subcommands.args]
            input = { required = false, help = "the file to add" }
        "#,
    )
    .unwrap();
    let input = "--testing file.txt";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_display_order() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [[subcommands]]
            name = "beta"
            display-order = 0
            about = "Some help and text"
            [[subcommands]]
            name = "alpha"
            display-order = 0
            about = "I should be first!"
        "#,
    )
    .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_hide() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [[subcommands]]
            name = "beta"
            about = "Some help and text"
            [[subcommands]]
            name = "alpha"
            about = "I should be hidden!"
            hide = true
        "#,
    )
    .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_subcommand_required() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            subcommand-required = true
            [[subcommands]]
            name = "test"
        "#,
    )
    .unwrap();
    let args: Vec<OsString> = vec![];
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

// the docs say:
//
// > All trailing arguments will be stored under the subcommand's sub-matches
// > using an empty string argument name
//
// we accept but do not extract this in the parser.
//
// see https://docs.rs/clap/latest/clap/struct.Command.html#examples-77
#[test]
fn test_allow_external_subcommands() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            allow-external-subcommands = true
        "#,
    )
    .unwrap();
    let input = "subcmd --option value -fff --flag";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

// TODO this does not seem to work, it does allow
//
// see https://docs.rs/clap/latest/clap/struct.Command.html#method.args_conflicts_with_subcommands
#[test]
fn test_args_conflicts_with_subcommands() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            [args]
            foo = { short = 'f' }
            [[subcommands]]
            name = "test"
            args-conflicts-with-subcommands = true
            [subcommands.args]
            input = { help = "the file to add" }
                [[subcommands.subcommands]]
                name  = "nested"
                about = "A nested sub command"
                [subcommands.subcommands.args]
                arg3 = { long = "arg3" }
        "#,
    )
    .unwrap();
    let input = "test";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_subcommand_precedence_over_arg() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            subcommand-precedence-over-arg = true
            [args]
            arg = { long = "arg", num-args = "1..", action = "set" }
            [[subcommands]]
            name = "sub"
        "#,
    )
    .unwrap();
    let input = "--arg 1 2 3 sub";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

// TODO: subcommand_negates_reqs
// TODO: multicall

#[test]
fn test_subcommand_value_name() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            subcommand-value-name = "THING"
            [[subcommands]]
            name = "sub"
        "#,
    )
    .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_subcommand_help_heading() {
    let app: Command = toml::from_str(
        r#"
            name = "myprog"
            subcommand-help-heading = "Things"
            [[subcommands]]
            name = "sub"
        "#,
    )
    .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}
