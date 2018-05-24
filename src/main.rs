extern crate handlebars;
extern crate argparse;
extern crate serde_yaml;
extern crate serde_json;
#[cfg(feature = "mediawiki")]
extern crate mwparser_utils;

use std::process;
use std::io;
use std::io::Read;
use std::fs;
use serde_yaml::Value;


use handlebars::Handlebars;

use argparse::{ArgumentParser, Store, List};

mod helpers;

use helpers::*;

#[derive(Default, Debug)]
struct Args {
    pub input_template: String,
    pub input_data: String,
    pub additional_data: Vec<String>,
}

fn main() {
    let mut args = Args::default();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description(
            "This program uses the template engine handlebars \
            to render a given template with the input data."
        );
        ap.refer(&mut args.input_template).add_option(
            &["-i", "--input"],
            Store,
            "Path to the input template",
        );
        ap.refer(&mut args.input_data).add_option(
            &["-d", "--data"],
            Store,
            "Path to the data file in yaml format.",
        );
        ap.refer(&mut args.additional_data).add_argument(
            "additional",
            List,
            "additional data: key value key2 val2",
        );
        ap.parse_args_or_exit();
    }

    let mut reg = Handlebars::new();
    reg.register_helper("add", Box::new(AddHelper));
    reg.register_helper("mult", Box::new(MultHelper));
    #[cfg(feature = "mediawiki")]
    {
        reg.register_helper("escape_make", Box::new(MakeEscapeHelper));
    }

    let template = if args.input_template.is_empty() {
        eprintln!("Input template must be specified!");
        process::exit(1);
    } else {
        let mut input = String::new();
        let file = fs::File::open(args.input_template)
            .expect("Could not open template file!");
        io::BufReader::new(file).read_to_string(&mut input)
            .expect("Could not read from template file!");
        input
    };

    let mut data: Value = if args.input_data.is_empty() {
        let file = io::BufReader::new(io::stdin());
        serde_yaml::from_reader(file)
            .expect("Could not parse data file!")
    } else {
        let file = io::BufReader::new(fs::File::open(args.input_data)
            .expect("Could not open data file!"));
        serde_yaml::from_reader(file)
            .expect("Could not parse data file!")
    };

    // add additional data
    for pair in args.additional_data.chunks(2) {
        if pair.len() < 2 {
            eprintln!("additional data must be supplied with key \
                       and value as separate arguments");
            process::exit(1);
        }

        if let Value::Mapping(ref mut map) = data {
            map.insert(pair[0].clone().into(), pair[1].clone().into());
        } else {
            eprintln!("input data must be a YAML object!");
            process::exit(1);
        }
    }

    println!(
        "{}",
        reg.render_template(&template, &data)
            .expect("template rendering failed!")
    );
}

