use std::env;
mod create_component_files;
use crate::create_component_files::files_creater;
fn main() {
    let mut command_line_args = env::args().enumerate();
    let component_name = command_line_args.find(|(_, arg)| arg == "--name" || arg == "--n");
    let typescript = command_line_args.find(|(_, arg)| arg == "--ts" || arg == "--typescript");
    let css_module = command_line_args.find(|(_, arg)| arg == "--module" || arg == "--m");
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
