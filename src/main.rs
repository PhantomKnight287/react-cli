use std::env;
mod create_component_files;
use crate::create_component_files::files_creater;
mod create_hooks_files;
use crate::create_hooks_files::hooks_file_creator;
use colored::Colorize;
fn main() {
    let component_name = env::args()
        .enumerate()
        .find(|(_, arg)| arg == "--name" || arg == "--n");
    let typescript = env::args()
        .enumerate()
        .find(|(_, arg)| arg == "--ts" || arg == "--typescript");
    let css_module = env::args()
        .enumerate()
        .find(|(_, arg)| arg == "--module" || arg == "--m");
    let hook = env::args()
        .enumerate()
        .find(|(_, arg)| arg == "--hook" || arg == "--h");
    if hook != None {
        let hook_file_name;
        if typescript != None {
            hook_file_name = format!(
                "{:}.tsx",
                get_args(hook.clone()).split(".").collect::<Vec<&str>>()[0]
            );
        } else {
            hook_file_name = format!(
                "{:}.jsx",
                get_args(hook.clone()).split(".").collect::<Vec<&str>>()[0]
            );
        }
        hooks_file_creator::hooks_file_creator(hook_file_name);
        return;
    }
    if component_name == None {
        println!("{}", format!("Please Provide A Component Name").red());
        return;
    }
    let mut stylesheet: String = String::new();
    let component_file_name;
    if css_module != None {
        stylesheet = format!("{:}.module.css", get_args(component_name.clone()),);
    } else if css_module == None {
        stylesheet = format!("{:}.css", get_args(component_name.clone()),);
    }
    if typescript == None {
        component_file_name = format!("{:}.jsx", get_args(component_name.clone()),);
    } else {
        component_file_name = format!("{:}.tsx", get_args(component_name),);
    }
    files_creater::components_files_creater(
        component_file_name,
        stylesheet.clone(),
        stylesheet.contains("module"),
    );
}

fn get_args(component_name: std::option::Option<(usize, std::string::String)>) -> String {
    env::args()
        .nth(component_name.unwrap().0 + 1)
        .unwrap()
        .to_string()
}
