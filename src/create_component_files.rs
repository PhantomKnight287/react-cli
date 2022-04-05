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
            "export {{ default }} from \"./{}\";",
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
                    "import styles from './{:}';
                    export default function {:}() {{
                        return (
                            <div className={{styles.h1Container}}>
                            <h1>Hello World</h1>
                            </div>
                        );
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
                    "import './{:}';
                    export default function {:}() {{
                        return (
                            <div className=\"h1Container\" >
                            <h1>Hello, world!</h1>
            </div>
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
