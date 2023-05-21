// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Ana Hobden (@hoverbear) <operator@hoverbear.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// This work was derived from Structopt (https://github.com/TeXitoi/structopt)
// commit#ea76fa1b1b273e65e3b0b1046643715b49bec51f which is licensed under the
// MIT/Apache 2.0 license.

use clap::Parser;
use clap_builder::Arg;
use clap_builder::ArgGroup;
use clap_builder::ArgMatches;
use clap_builder::Args;
use clap_builder::Command;
use clap_builder::FromArgMatches;
use clap_builder::Id;
use clap_derive::Args;

use crate::utils::get_help;

#[derive(Args)]
enum DerivedA {
    A,
    B,
    C(String),
    D(String),
    E { a: String, b: u32 },
}

impl FromArgMatches for DerivedA {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, clap_builder::Error> {
        todo!()
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), clap_builder::Error> {
        todo!()
    }
}

enum BuilderA {
    A,
    B,
    C(String),
    // How should this be handled?
    //
    // D(String, String),
    //
    // I assume this should act as though this is a struct? (not related to D above)
    // i.e.,
    //
    // struct E {
    //     a: u32,
    //     b: u32,
    // }
    //
    // Or should this not be allowed at all, instead requiring
    //
    // E(EInner),
    //
    // and elsewhere...
    //
    // struct EInner {
    //     a: u32,
    //     b: u32,
    // }
    //
    // ?
    //
    // E { a: u32, b: u32 },
}

impl FromArgMatches for BuilderA {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, clap_builder::Error> {
        Self::from_arg_matches_mut(&mut matches.clone())
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), clap_builder::Error> {
        if matches.contains_id("A") {
            *self = BuilderA::A;
        } else if matches.contains_id("B") {
            *self = BuilderA::B;
        } else if matches.contains_id("C") {
            let c = matches.remove_one::<String>("C").unwrap();
            *self = BuilderA::C(c);
        }

        Ok(())
    }
}

impl Args for BuilderA {
    fn group_id() -> Option<Id> {
        Some(Id::from("A"))
    }

    fn augment_args(cmd: Command) -> Command {
        cmd.group(ArgGroup::new("A").multiple(false).required(true).args([
            Id::from("A"),
            Id::from("B"),
            Id::from("C"),
        ]))
        .arg(Arg::new("A").required(true))
        .arg(Arg::new("B").required(true))
        .arg(Arg::new("C").required(true))
    }

    fn augment_args_for_update(cmd: Command) -> Command {
        cmd.group(ArgGroup::new("A").multiple(false).required(true).args([
            Id::from("A"),
            Id::from("B"),
            Id::from("C"),
        ]))
        .arg(Arg::new("A").required(false))
        .arg(Arg::new("B").required(false))
        .arg(Arg::new("C").required(false))
    }
}

#[test]
fn required_argument() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        arg: i32,
    }
    assert_eq!(
        Opt { arg: 42 },
        Opt::try_parse_from(["test", "42"]).unwrap()
    );
    assert!(Opt::try_parse_from(["test"]).is_err());
    assert!(Opt::try_parse_from(["test", "42", "24"]).is_err());
}

#[test]
fn argument_with_default() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[arg(default_value = "42")]
        arg: i32,
    }
    assert_eq!(
        Opt { arg: 24 },
        Opt::try_parse_from(["test", "24"]).unwrap()
    );
    assert_eq!(Opt { arg: 42 }, Opt::try_parse_from(["test"]).unwrap());
    assert!(Opt::try_parse_from(["test", "42", "24"]).is_err());
}

#[test]
fn auto_value_name() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        my_special_arg: i32,
    }

    let help = get_help::<Opt>();

    assert!(help.contains("MY_SPECIAL_ARG"));
    // Ensure the implicit `num_vals` is just 1
    assert_eq!(
        Opt { my_special_arg: 10 },
        Opt::try_parse_from(["test", "10"]).unwrap()
    );
}

#[test]
fn explicit_value_name() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[arg(value_name = "BROWNIE_POINTS")]
        my_special_arg: i32,
    }

    let help = get_help::<Opt>();

    assert!(help.contains("BROWNIE_POINTS"));
    assert!(!help.contains("MY_SPECIAL_ARG"));
    // Ensure the implicit `num_vals` is just 1
    assert_eq!(
        Opt { my_special_arg: 10 },
        Opt::try_parse_from(["test", "10"]).unwrap()
    );
}

#[test]
fn option_type_is_optional() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        arg: Option<i32>,
    }
    assert_eq!(
        Opt { arg: Some(42) },
        Opt::try_parse_from(["test", "42"]).unwrap()
    );
    assert_eq!(Opt { arg: None }, Opt::try_parse_from(["test"]).unwrap());
    assert!(Opt::try_parse_from(["test", "42", "24"]).is_err());
}

#[test]
fn vec_type_is_multiple_values() {
    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        arg: Vec<i32>,
    }
    assert_eq!(
        Opt { arg: vec![24] },
        Opt::try_parse_from(["test", "24"]).unwrap()
    );
    assert_eq!(Opt { arg: vec![] }, Opt::try_parse_from(["test"]).unwrap());
    assert_eq!(
        Opt { arg: vec![24, 42] },
        Opt::try_parse_from(["test", "24", "42"]).unwrap()
    );
    assert_eq!(
        clap::error::ErrorKind::ValueValidation,
        Opt::try_parse_from(["test", "NOPE"]).err().unwrap().kind()
    );
}
