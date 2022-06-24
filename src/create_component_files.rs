pub mod files_creater {
    use colored::Colorize;
    use std::fs;
    pub fn components_files_creater(
        component_name: String,
        stylesheet_name: String,
        is_css_module: bool,
        is_class_component: bool,
    ) {
        let index_file_content = format!(
            "export * from \"./{}\";",
            component_name.split(".").collect::<Vec<&str>>()[0]
        );
        let css_file_content;
        let component_file_content;
        if is_css_module == true {
            css_file_content = "".to_string();
            if is_class_component == true {
                component_file_content = format!(
                    "import {{ Component }} from \"react\";\nimport styles from \"./{}\";\n\nclass {} extends Component {{}}\n\nexport default {}",
                    stylesheet_name, component_name.split(".").collect::<Vec<&str>>()[0], component_name.split(".").collect::<Vec<&str>>()[0]
                )
            } else {
                component_file_content = format!(
                    "import styles from './{:}';\nexport default function {:}() {{\n\t\treturn (\n\t\t\t<div>\n\t\t\t\t<h1>Hello World</h1>\n\t\t\t</div>
)
}}
                    ",
                    stylesheet_name,
                    component_name.split(".").collect::<Vec<&str>>()[0]
                );
            }
        } else {
            css_file_content = "".to_string();
            if is_class_component == true {
                component_file_content = format!(
                    "import {{ Component }} from \"react\";\nimport \"./{}\";\n\nclass {} extends Component {{}}\n\nexport default {}",
                    stylesheet_name, component_name.split(".").collect::<Vec<&str>>()[0], component_name.split(".").collect::<Vec<&str>>()[0]
                )
            } else {
                component_file_content = format!(
                    "import './{:}';\nexport default function {:}() {{\n\t\treturn (\n\t\t\t<div >\n\t\t\t\t<h1>Hello, world!</h1>\n\t\t</div>
);
}}
                ",
                    stylesheet_name,
                    component_name.split(".").collect::<Vec<&str>>()[0]
                );
            }
        }
        let result = fs::create_dir(format!(
            "{:}",
            component_name.split(".").collect::<Vec<&str>>()[0]
        ));
        match result {
            Ok(_) => {
                println!(
                    "{}",
                    format!(
                        "{} {}",
                        "Created Directory:".green(),
                        component_name.split(".").collect::<Vec<&str>>()[0]
                    )
                );
            }
            Err(e) => {
                println!("{}", format!("{} {}", "Error:".red(), e));
                return;
            }
        }
        let result = fs::write(
            format!(
                "{:}/{:}",
                component_name.split(".").collect::<Vec<&str>>()[0],
                component_name
            ),
            component_file_content.clone(),
        );
        match result {
            Ok(_) => {
                println!(
                    "{}",
                    format!("{} {}", "Created File:".green(), component_name)
                );
            }
            Err(e) => {
                println!("{}", format!("{} {}", "Error:".red(), e));
                return;
            }
        }
        let result = fs::write(
            format!(
                "{:}/{:}",
                component_name.split(".").collect::<Vec<&str>>()[0],
                stylesheet_name
            ),
            css_file_content.clone(),
        );
        match result {
            Ok(_) => {
                println!(
                    "{}",
                    format!("{} {}", "Created File:".green(), stylesheet_name)
                );
            }
            Err(e) => {
                println!("{}", format!("{} {}", "Error:".red(), e));
                return;
            }
        }
        let result = fs::write(
            format!(
                "{:}/index.{:}",
                component_name.split(".").collect::<Vec<&str>>()[0],
                component_name.split(".").collect::<Vec<&str>>()[1]
            ),
            index_file_content.clone(),
        );
        match result {
            Ok(_) => {
                println!(
                    "{}",
                    format!(
                        "{} index.{:}",
                        "Created File:".green(),
                        component_name.split(".").collect::<Vec<&str>>()[1]
                    )
                );
            }
            Err(e) => {
                println!("{}", format!("{} {}", "Error:".red(), e));
                return;
            }
        }
    }
}
