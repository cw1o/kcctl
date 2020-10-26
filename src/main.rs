#[macro_use]
extern crate clap;
extern crate config;
extern crate dirs;

pub mod list;

use clap::App;
use list::list_configs;
use std::env;
use std::fs;
use std::path::PathBuf;

// Modules
// mod config;

fn main() -> () {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut config_dir_path = dirs::config_dir().unwrap();
    config_dir_path.push("kcctl/configs");

    let mut base_config_path = dirs::config_dir().unwrap();
    base_config_path.push("kcctl");

    if base_config_path.exists() {
        fs::create_dir(&base_config_path);
        fs::create_dir(&config_dir_path);
    } else if config_dir_path.exists() {
        fs::create_dir(&config_dir_path);
    };

    let _kubeconfig_path = match env::var("KUBECONFIG") {
        Ok(v) => PathBuf::from(v),
        Err(_e) => {
            let mut temp_config_path = dirs::home_dir().unwrap();
            temp_config_path.push(".kube");
            temp_config_path.push("config");
            temp_config_path
        }
    };

    match matches.subcommand() {
        ("list", Some(_list_matches)) => {
            list_configs(config_dir_path);
        }
        ("delete", Some(delete_matches)) => {
            println!("delete subcommmand found");
            if delete_matches.is_present("current") {
                println!("current flag found");
            } else if let Some(file) = delete_matches.value_of("file") {
                println!("Value of file is: {:?}", file); 
            }
        }
        ("switch", Some(switch_matches)) => {
            println!("switch subcommmand found");
            if let f = switch_matches.value_of("file").unwrap() {
                println!("Value of file is: {:?}", f); 
            }
            // Write logic to handle Args
            // Pass 'file' arg to switch_config function
        }
        ("import", Some(import_matches)) => {
            println!("import subcommmand found");
            if let name = import_matches.value_of("name").unwrap() {
                println!("Value of name is: {:?}", name)
            } 
            // Write logic to handle import commands
            // Pass 'name' arg to import_config function
        }
        ("show", Some(show_matches)) => {
            println!("show subcommmand found");
            if show_matches.is_present("current") {
                println!("current flag found")
            } else if let Some(config) = show_matches.value_of("config") {
                println!("Value of config is {:?}", config)
            }
            // Write logic to handle show commands.
            // call show_command function with arg
        }
        ("context", Some(context_matches)) => {
            println!("context subcommmand found");
            if context_matches.is_present("list") {
                println!("List subcommand found")
            } else if context_matches.is_present("switch") {
                println!("Switch subcommand found");
                if let Some(switch_matches) = context_matches.subcommand_matches("switch") {
                    let input = switch_matches.value_of("INPUT").unwrap(); // Can safely call unwrap() as arg is required by clap
                    println!("The value of input is: {:?}", input);
                }
            }   
        }
        ("namespace", Some(namespace_matches)) => {
            println!("Namespace subcomand found");
            if let namespace = namespace_matches.value_of("name").unwrap() {
                println!("Value of name is: {:?}", namespace);
            }
            // Write Logic to Handle context arguements
        }
        // Some(&_) => println!("No valid subcommand found"),
        _ => println!("No subcommand found"),
    }
}
