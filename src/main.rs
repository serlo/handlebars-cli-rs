#[macro_use]
extern crate handlebars;
#[macro_use]
extern crate structopt;
extern crate serde_yaml;
extern crate serde_json;
#[cfg(feature = "mediawiki")]
extern crate mwparser_utils;
#[cfg(feature = "mfnf")]
extern crate mfnf_sitemap;

use std::process;
use std::io;
use std::io::Read;
use std::fs;
use std::path::PathBuf;

use serde_yaml::Value;
use handlebars::Handlebars;
use structopt::StructOpt;

mod helpers;

use helpers::*;

#[derive(Default, Debug, StructOpt)]
#[structopt(name = "handlebars-cli")]
/// This program uses the template engine handlebars
/// to render a given template with the input data.
struct Args {
    /// Path to the input template file.
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    pub input_template: PathBuf,
    /// Path to the data file in yaml format.
    #[structopt(short = "d", long = "data", parse(from_os_str))]
    pub input_data: Option<PathBuf>,
    /// Disable strict mode.
    #[structopt(short = "n", long = "no-strict")]
    pub no_strict: bool,
    /// List of files allowed as base templates.
    #[structopt(short = "b", long = "base-templates", parse(from_os_str))]
    pub base_templates: Vec<PathBuf>,
    /// additional data as key-value-pairs. (k1 v1 k2 v2 ...)
    #[structopt(name = "additional")]
    pub additional_data: Vec<String>,
}

fn main() {
    let args = Args::from_args();

    let mut reg = Handlebars::new();
    reg.set_strict_mode(!args.no_strict);
    reg.register_helper("add", Box::new(AddHelper));
    reg.register_helper("mult", Box::new(MultHelper));
    #[cfg(feature = "mediawiki")]
    {
        reg.register_helper("escape_make", Box::new(EscapeMake));
        reg.register_helper("urlencode", Box::new(UrlEncode));
    }
    #[cfg(feature = "mfnf")]
    {
        reg.register_helper("chapter_excluded", Box::new(is_article_excluded));
        reg.register_helper("part_excluded", Box::new(is_part_excluded));
    }

    let template_file = fs::File::open(args.input_template)
        .expect("Could not open template file!");
    let template = {
        let mut input = String::new();
        io::BufReader::new(template_file).read_to_string(&mut input)
            .expect("Could not read from template file!");
        input
    };

    let mut data: Value = if let Some(path) = args.input_data {
        let file = io::BufReader::new(fs::File::open(path)
            .expect("Could not open data file!"));
        serde_yaml::from_reader(file)
            .expect("Could not parse data file!")
    } else {
        let file = io::BufReader::new(io::stdin());
        serde_yaml::from_reader(file)
            .expect("Could not parse data file!")
    };

    for file in args.base_templates {
        let filename = file.file_name()
            .expect(&format!("template base file {:?} has no filename!", &file));
        reg.register_template_file(&filename.to_string_lossy(), &file)
            .expect(&format!("could not register template file {:?}!", &file));
    }

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

