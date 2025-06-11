use claptrap::command::Command;
use claptrap::parse;
use std::ffi::OsString;

#[test]
fn test_short() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            config = { short = 'c', action = "set" }
        "#,
    )
    .unwrap();
    let input = "-c file.toml";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_short_help() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            disable-help-flag = true
            [args]
            host = { short = 'h', long = "host" }
            help = { long = "help", global = true, action = "help" }
        "#,
    )
    .unwrap();
    let input = "-h wikipedia.org";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_long() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            cfg = { long = "config", action = "set" }
        "#,
    )
    .unwrap();
    let input = "--config file.toml";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_alias() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            test = { long = "alias", alias = "alias", action = "set" }
        "#,
    )
    .unwrap();
    let input = "--alias cool";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_short_alias() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            test = { short = 't', short-alias = 'e', action = "set" }
        "#,
    )
    .unwrap();
    let input = "-e cool";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_aliases() {
    let app: Command = toml::from_str(
            r#"
            name = "prog"
            [args]
            test = { long = "test", aliases = ["do-tests", "tests"], action = "set-true", required = false, help = "the file to add" }
        "#,
        )
            .unwrap();
    let input = "--do-tests";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_short_aliases() {
    let app: Command = toml::from_str(
            r#"
            name = "prog"
            [args]
            test = { short = 't', short-aliases = ['e', 's'], action = "set-true", required = false, help = "the file to add" }
        "#,
        )
            .unwrap();
    let input = "-s";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_visible_alias() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            test = { long = "test", visible-alias = "something-awesome", action = "set" }
        "#,
    )
    .unwrap();
    let input = "--something-awesome coffee";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_visible_short_alias() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            test = { long = "test", visible-short-alias = 't', action = "set" }
        "#,
    )
    .unwrap();
    let input = "-t coffee";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_visible_aliases() {
    let app: Command = toml::from_str(
            r#"
            name = "prog"
            [args]
            test = { long = "test", visible-aliases = ["something", "awesome", "cool"], action = "set-true" }
        "#,
        )
            .unwrap();
    let input = "--awesome";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_visible_short_aliases() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            test = { long = "test", visible-short-aliases = ['t', 'e'], action = "set-true" }
        "#,
    )
    .unwrap();
    let input = "-t";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_index() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            mode = { index = 1 }
            debug = { long = "debug", action = "set-true" }
        "#,
    )
    .unwrap();
    let input = "--debug fast";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_trailing_var_arg() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            cmd = { trailing-var-arg = true, min-args = 1 }
        "#,
    )
    .unwrap();
    let input = "arg1 -r val1";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_last() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            first = {}
            second = {}
            third = { action = "set", last = true }
        "#,
    )
    .unwrap();

    let input1 = "one -- three";
    let args1: Vec<OsString> = input1.split(" ").map(OsString::from).collect();
    let output1 = parse(app.clone(), args1);
    insta::assert_snapshot!(output1);

    let input2 = "one two three";
    let args2: Vec<OsString> = input2.split(" ").map(OsString::from).collect();
    let output2 = parse(app, args2);
    insta::assert_snapshot!(output2);
}

#[test]
fn test_required() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            cfg = { long = "config", required = true, action = "set" }
        "#,
    )
    .unwrap();

    let input1 = "--config file.conf";
    let args1: Vec<OsString> = input1.split(" ").map(OsString::from).collect();
    let output1 = parse(app.clone(), args1);
    insta::assert_snapshot!(output1);

    let args2: Vec<OsString> = vec![];
    let output2 = parse(app.clone(), args2);
    insta::assert_snapshot!(output2);
}

#[test]
fn test_requires() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            cfg = { long = "config", requires = "input", action = "set" }
            input = {}
        "#,
    )
    .unwrap();

    let args1: Vec<OsString> = vec![];
    let output1 = parse(app.clone(), args1);
    insta::assert_snapshot!(output1);

    let input2 = "--config file.conf";
    let args2: Vec<OsString> = input2.split(" ").map(OsString::from).collect();
    let output2 = parse(app, args2);
    insta::assert_snapshot!(output2);
}

#[test]
fn test_exclusive() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            exclusive = { long  = "exclusive", action = "set", exclusive = true }
            debug = { long = "debug" }
            input = {}
        "#,
    )
    .unwrap();
    let input = "--exclusive file.conf file.txt";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

// TODO: global (required subcommand)

#[test]
fn test_action_set() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            flag = { long = "flag", action = "set" }
        "#,
    )
    .unwrap();
    let input = "--flag value";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn it_fails_when_action_set_twice() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            flag = { long = "flag", action = "set" }
        "#,
    )
    .unwrap();
    let input = "--flag value --flag value";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_action_append() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            flag = { long = "flag", action = "append" }
        "#,
    )
    .unwrap();
    let input = "--flag value1 --flag value2";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_action_set_true() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            flag = { long = "flag", action = "set-true" }
        "#,
    )
    .unwrap();
    let input = "--flag";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn it_fails_when_action_set_true_twice() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            flag = { long = "flag", action = "set-true" }
        "#,
    )
    .unwrap();
    let input = "--flag --flag";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_action_set_false() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            flag = { long = "flag", action = "set-false" }
        "#,
    )
    .unwrap();
    let input = "--flag";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn it_fails_when_action_set_false_twice() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            flag = { long = "flag", action = "set-false" }
        "#,
    )
    .unwrap();
    let input = "--flag --flag";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_action_count() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            flag = { long = "flag", action = "count" }
        "#,
    )
    .unwrap();
    let input = "--flag --flag";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_action_help() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            flag = { long = "flag", action = "help" }
        "#,
    )
    .unwrap();
    let input = "-h";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_action_help_short() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            flag = { long = "flag", action = "help-short" }
        "#,
    )
    .unwrap();
    let input = "-h";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_action_help_long() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            flag = { long = "flag", action = "help-long" }
        "#,
    )
    .unwrap();
    let input = "-h";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_action_version() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            version = "1.0.0"
            [args]
            flag = { long = "flag", action = "version" }
        "#,
    )
    .unwrap();
    let input = "--version";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

// TODO: does not cover ranges or any types other than string-ish or bool-ish types
#[test]
fn test_value_parser() {
    let app: Command = toml::from_str(
            r#"
            name = "raw"
            [args]
            color = { long = "color", value-parser = ["always", "auto", "never"], default-value = "auto" }
        "#,
        )
            .unwrap();
    let input = "--color auto";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_num_args() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            mode = { long = "mode", num-args = 1 }
        "#,
    )
    .unwrap();
    let input = "--mode fast";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_num_args_default_missing_value() {
    let app: Command = toml::from_str(r#"
            name = "prog"
            [args]
            mode = { long = "mode", default-missing-value = "slow", default-value = "plaid", min-args = 0, max-args = 1 }
        "#).unwrap();

    let input1 = "--mode fast";
    let args1: Vec<OsString> = input1.split(" ").map(OsString::from).collect();
    let output1 = parse(app.clone(), args1);
    insta::assert_snapshot!(output1);

    let input2 = "--mode";
    let args2: Vec<OsString> = input2.split(" ").map(OsString::from).collect();
    let output2 = parse(app.clone(), args2);
    insta::assert_snapshot!(output2);

    let args3: Vec<OsString> = vec![];
    let output3 = parse(app, args3);
    insta::assert_snapshot!(output3);
}

#[test]
fn test_num_args_tuples() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            file = { short = 'F', num-args = 2, action = "set" }
        "#,
    )
    .unwrap();

    let input1 = "-F in-file out-file";
    let args1: Vec<OsString> = input1.split(" ").map(OsString::from).collect();
    let output1 = parse(app.clone(), args1);
    insta::assert_snapshot!(output1);

    let input2 = "-F file1";
    let args2: Vec<OsString> = input2.split(" ").map(OsString::from).collect();
    let output2 = parse(app, args2);
    insta::assert_snapshot!(output2);
}

#[test]
fn test_num_args_multi_positional() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            file = { short = 'F', min-args = 0, action = "set" }
            word = {}
        "#,
    )
    .unwrap();

    let input1 = "-F file1 file2 file3 word";
    let args1: Vec<OsString> = input1.split(" ").map(OsString::from).collect();
    let output1 = parse(app.clone(), args1);
    insta::assert_snapshot!(output1);

    let input2 = "word -F file1 file2 file3";
    let args2: Vec<OsString> = input2.split(" ").map(OsString::from).collect();
    let output2 = parse(app, args2);
    insta::assert_snapshot!(output2);
}

#[test]
fn test_num_args_multi_positional_solution() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            file = { short = 'F', action = "append" }
            word = {}
        "#,
    )
    .unwrap();
    let input = "-F file1 -F file2 -F file3 word";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_value_name() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            config = { long = "config", value-name = "FILE" }
        "#,
    )
    .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_value_names() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            io = { long = "io-files", value-names = ["INFILE", "OUTFILE"] }
        "#,
    )
    .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_ignore_case() {
    let app: Command = toml::from_str(
            r#"
            name = "prog"
            [args]
            option = { long = "option", action = "set", ignore-case = true, value-parser = ["test123"] }
        "#,
        ).unwrap();
    let input = "--option TeSt123";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_ignore_case_multi() {
    let app: Command = toml::from_str(
            r#"
            name = "prog"
            [args]
            option = { short = 'o', long = "option", action = "set", ignore-case = true, min-args = 1, value-parser = ["test123", "test321"] }
        "#,
        ).unwrap();
    let input = "--option TeSt123 teST123 tESt321";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_allow_hyphen_values() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            pat = { long = "pattern", action = "set", allow-hyphen-values = true }
        "#,
    )
    .unwrap();
    let input = "--pattern -file";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn it_fails_when_not_allow_hyphen_values() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            pat = { long = "pattern", action = "set" }
        "#,
    )
    .unwrap();
    let input = "--pattern -file";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_allow_negative_numbers() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            num = { allow-negative-numbers = true }
        "#,
    )
    .unwrap();
    let input = "-20";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_require_equals() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            cfg = { long = "config", action = "set", require-equals = true }
        "#,
    )
    .unwrap();

    let input1 = "--config=file.conf";
    let args1: Vec<OsString> = input1.split(" ").map(OsString::from).collect();
    let output1 = parse(app.clone(), args1);
    insta::assert_snapshot!(output1);

    let input2 = "--config file.conf";
    let args2: Vec<OsString> = input2.split(" ").map(OsString::from).collect();
    let output2 = parse(app, args2);
    insta::assert_snapshot!(output2);
}

#[test]
fn test_value_delimiter() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            config = { short = 'c', long = "config", value-delimiter = "," }
        "#,
    )
    .unwrap();
    let input = "--config=val1,val2,val3";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_value_terminator() {
    let app: Command = toml::from_str(
            r#"
            name = "prog"
            [args]
            cmds = { action = "set", min-args = 0, allow-hyphen-values = true, value-terminator = ";" }
            location = {}
        "#,
        )
            .unwrap();
    let input = "find type f -name special ; /home/clap";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_default_value() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            opt = { long = "myopt", default-value = "myval" }
        "#,
    )
    .unwrap();

    let args1: Vec<OsString> = vec![];
    let output1 = parse(app.clone(), args1);
    insta::assert_snapshot!(output1);

    let input2 = "--myopt=non_default";
    let args2: Vec<OsString> = input2.split(" ").map(OsString::from).collect();
    let output2 = parse(app, args2);
    insta::assert_snapshot!(output2);
}

#[test]
fn test_default_values() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            opt = { long = "myopt", num-args = 2, default-values = ["myval1", "myval2"] }
        "#,
    )
    .unwrap();

    let args1: Vec<OsString> = vec![];
    let output1 = parse(app.clone(), args1);
    insta::assert_snapshot!(output1);

    let input2 = "--myopt non_default another_value";
    let args2: Vec<OsString> = input2.split(" ").map(OsString::from).collect();
    let output2 = parse(app, args2);
    insta::assert_snapshot!(output2);
}

#[test]
fn test_default_missing_value() {
    let app: Command = toml::from_str(
            r#"
            name = "prog"
            [args]
            color = { long = "color", value-name = "COLOR", value-parser = ["always", "auto", "never"], default-value = "auto", min-args = 0, max-args = 1, require-equals = true, default-missing-value = "always", help = "Specify WHEN to colorize output." }
        "#,
        ).unwrap();

    let args1: Vec<OsString> = vec![];
    let output1 = parse(app.clone(), args1);
    insta::assert_snapshot!(output1);

    let input2 = "--color=never";
    let args2: Vec<OsString> = input2.split(" ").map(OsString::from).collect();
    let output2 = parse(app.clone(), args2);
    insta::assert_snapshot!(output2);

    let input3 = "--color";
    let args3: Vec<OsString> = input3.split(" ").map(OsString::from).collect();
    let output3 = parse(app, args3);
    insta::assert_snapshot!(output3);
}

// TODO this test is missing `value_parser(value_parser!(bool))` which we cannot express yet
#[test]
fn test_default_missing_value_bool_literal() {
    let app: Command = toml::from_str(
            r#"
            name = "prog"
            [args]
            create = { long = "create", value-name = "BOOL", min-args = 0, max-args = 1, require-equals = true, default-missing-value = "true" }
        "#,
        ).unwrap();

    let args1: Vec<OsString> = vec![];
    let output1 = parse(app.clone(), args1);
    insta::assert_snapshot!(output1);

    let input2 = "--create=false";
    let args2: Vec<OsString> = input2.split(" ").map(OsString::from).collect();
    let output2 = parse(app.clone(), args2);
    insta::assert_snapshot!(output2);

    let input3 = "--create";
    let args3: Vec<OsString> = input3.split(" ").map(OsString::from).collect();
    let output3 = parse(app, args3);
    insta::assert_snapshot!(output3);
}

#[test]
fn test_env() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            flag = { long = "flag", env = "MY_FLAG", action = "set" }
        "#,
    )
    .unwrap();
    let args: Vec<OsString> = vec![];
    unsafe {
        std::env::set_var("MY_FLAG", "env");
    }
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_env_falsey() {
    let app: Command = toml::from_str(
            r#"
            name = "prog"
            [args]
            true_flag = { long = "true_flag", env = "TRUE_FLAG", action = "set-true", typed-value-parser = "falsey" }
            false_flag = { long = "false_flag", env = "FALSE_FLAG", action = "set-true", typed-value-parser = "falsey" }
            absent_flag = { long = "absent_flag", env = "ABSENT_FLAG", action = "set-true", typed-value-parser = "falsey" }
        "#,
        )
            .unwrap();
    let args: Vec<OsString> = vec![];
    unsafe {
        std::env::set_var("TRUE_FLAG", "true");
        std::env::set_var("FALSE_FLAG", "0");
    }
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_env_option() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            flag = { long = "flag", env = "MY_FLAG", action = "set" }
        "#,
    )
    .unwrap();
    let input = "--flag opt";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    unsafe {
        std::env::set_var("MY_FLAG", "env");
    }
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_env_default_value() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            flag = { long = "flag", env = "MY_FLAG", action = "set", default-value = "default" }
        "#,
    )
    .unwrap();
    let args: Vec<OsString> = vec![];
    unsafe {
        std::env::set_var("MY_FLAG", "env");
    }
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_env_multi() {
    let app: Command = toml::from_str(
            r#"
            name = "prog"
            [args]
            flag = { long = "flag", env = "MY_FLAG_MULTI", action = "set", min-args = 1, value-delimiter = "," }
        "#,
        )
            .unwrap();
    let args: Vec<OsString> = vec![];
    unsafe {
        std::env::set_var("MY_FLAG_MULTI", "env1,env2");
    }
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_help() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
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
#[ignore]
fn test_long_help() {
    let app: Command = toml::from_str(
            r#"
            name = "prog"
            [args]
            cfg = { long = "config", long-help = "The config file used by the myprog must be in JSON format with only valid keys and may not contain other nonsense that cannot be read by this program. Obviously I'm going on and on, so I'll stop now." }
        "#,
        )
            .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_display_order() {
    let app: Command = toml::from_str(
            r#"
            name = "prog"
            [args]
            boat = { short = 'b', long = "boat", action = "set", display-order = 0, help = "Some help and text" }
            airplane = { short = 'a', long = "airplane", action = "set", display-order = 0, help = "I should be first!" }
            custom-help = { short = '?', action = "help", display-order = 100, help = "Alt help" }
        "#,
        )
            .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_help_heading() {
    let app: Command = toml::from_str(
            r#"
            name = "prog"
            [args]
            cfg = { long = "config", help-heading = "Configuration Options", help = "Some help text describing the --config arg" }
        "#,
        )
            .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_next_line_help() {
    let app: Command = toml::from_str(
            r#"
            name = "prog"
            [args]
            opt = { short = 'o', long = "long-option-flag", action = "set", next-line-help = true, value-names = ["value1", "value2"], help = "Some really long help and complex\nhelp that makes more sense to be\non a line after the option" }
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
            name = "prog"
            [args]
            cfg = { long = "config", hide = true }
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
            [args]
            mode = { long = "mode", value-parser = ["fast", "slow"], action = "set", hide-possible-values = true }
        "#,
        )
            .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_hide_default_value() {
    let app: Command = toml::from_str(
            r#"
            name = "connect"
            [args]
            host = { long = "host", default-value = "localhost", action = "set", hide-default-value = true }
        "#,
        )
            .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_hide_env() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            mode = { long = "mode", env = "MODE", action = "set", hide-env = true }
        "#,
    )
    .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_hide_env_values() {
    let app: Command = toml::from_str(
        r#"
            name = "connect"
            [args]
            host = { long = "host", env = "CONNECT", action = "set", hide-env-values = true }
        "#,
    )
    .unwrap();
    let input = "--help";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_hide_short_help() {
    let app: Command = toml::from_str(
            r#"
            name = "prog"
            [args]
            cfg = { long = "config", hide-short-help = true, help = "Some help text describing the --config arg" }
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
fn test_hide_long_help() {
    let app: Command = toml::from_str(
            r#"
            name = "prog"
            [args]
            cfg = { long = "config", hide-long-help = true, help = "Some help text describing the --config arg" }
        "#,
        )
            .unwrap();

    let input1 = "--help";
    let args1: Vec<OsString> = input1.split(" ").map(OsString::from).collect();
    let output1 = parse(app.clone(), args1);
    insta::assert_snapshot!(output1);

    let input2 = "-h";
    let args2: Vec<OsString> = input2.split(" ").map(OsString::from).collect();
    let output2 = parse(app, args2);
    insta::assert_snapshot!(output2);
}

// TODO: group
// TODO: groups
// TODO: default_value_if
// TODO: default_value_ifs

#[test]
fn test_required_unless_present() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            cfg = { long = "config", required-unless-present = "dbg", action = "set" }
            dbg = { long = "debug", action = "set-true" }
        "#,
    )
    .unwrap();

    let input1 = "--debug";
    let args1: Vec<OsString> = input1.split(" ").map(OsString::from).collect();
    let output1 = parse(app.clone(), args1);
    insta::assert_snapshot!(output1);

    let args2: Vec<OsString> = vec![];
    let output2 = parse(app, args2);
    insta::assert_snapshot!(output2);
}

#[test]
fn test_required_unless_present_all() {
    let app: Command = toml::from_str(
            r#"
            name = "prog"
            [args]
            cfg = { long = "config", required-unless-present-all = ["dbg", "infile"], action = "set" }
            dbg = { long = "debug", action = "set-true" }
            infile = { short = 'i', action = "set" }
        "#,
        )
            .unwrap();

    let input1 = "--debug -i file";
    let args1: Vec<OsString> = input1.split(" ").map(OsString::from).collect();
    let output1 = parse(app.clone(), args1);
    insta::assert_snapshot!(output1);

    let args2: Vec<OsString> = vec![];
    let output2 = parse(app, args2);
    insta::assert_snapshot!(output2);
}

#[test]
fn test_required_unless_present_any() {
    let app: Command = toml::from_str(
            r#"
            name = "prog"
            [args]
            cfg = { long = "config", required-unless-present-any = ["dbg", "infile"], action = "set" }
            dbg = { long = "debug", action = "set-true" }
            infile = { short = 'i', action = "set" }
        "#,
        )
            .unwrap();

    let input1 = "--debug";
    let args1: Vec<OsString> = input1.split(" ").map(OsString::from).collect();
    let output1 = parse(app.clone(), args1);
    insta::assert_snapshot!(output1);

    let args2: Vec<OsString> = vec![];
    let output2 = parse(app, args2);
    insta::assert_snapshot!(output2);
}

// TODO: required_if_eq
// TODO: required_if_eq_any
// TODO: required_if_eq_all
// TODO: requires_if
// TODO: requires_ifs

#[test]
fn test_conflicts_with() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            cfg = { long = "config",  action = "set", conflicts-with = "debug" }
            debug = { long = "debug", action = "set-true" }
        "#,
    )
    .unwrap();
    let input = "--debug --config file.conf";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_conflicts_with_all() {
    let app: Command = toml::from_str(
        r#"
            name = "prog"
            [args]
            cfg = { long = "config",  action = "set", conflicts-with-all = ["debug", "input"] }
            debug = { long = "debug", action = "set-true" }
            input = {}
        "#,
    )
    .unwrap();
    let input = "--config file.conf file.txt";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_overrides_with() {
    let app: Command = toml::from_str(
            r#"
            name = "prog"
            [args]
            flag = { short = 'f', long = "flag", action = "set-true", help = "some flag", conflicts-with = "debug" }
            debug = { short = 'd', long = "debug", action = "set-true", help = "other flag" }
            color = { short = 'c', long = "color", action = "set-true", help = "third flag", overrides-with = "flag" }
        "#,
        )
            .unwrap();
    let input = "-f -d -c";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}

#[test]
fn test_overrides_with_all() {
    let app: Command = toml::from_str(
            r#"
            name = "prog"
            [args]
            flag = { short = 'f', long = "flag", action = "set-true", help = "some flag", conflicts-with = "color" }
            debug = { short = 'd', long = "debug", action = "set-true", help = "other flag" }
            color = { short = 'c', long = "color", action = "set-true", help = "third flag", overrides-with-all = ["flag", "debug"] }
        "#,
        )
            .unwrap();
    let input = "-f -d -c";
    let args: Vec<OsString> = input.split(" ").map(OsString::from).collect();
    let output = parse(app, args);
    insta::assert_snapshot!(output);
}
