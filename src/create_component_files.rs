pub mod files_creater {
    use colored::Colorize;
    use std::fs;
    pub fn components_files_creater(
        component_name: String,
        stylesheet_name: String,
        is_css_module: bool,
    ) {
        let index_file_content = format!(
            "export * from \"./{}\";",
            component_name.split(".").collect::<Vec<&str>>()[0]
        );
        let css_file_content;
        let component_file_content;
        if is_css_module == true {
            css_file_content = format!(
                ".h1Container{{
    text-align:center;
}}
"
            );
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
        } else {
            css_file_content = format!(
                ".h1Container{{
    text-align:center;
}}
                "
            );
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
        fs::create_dir(format!(
            "{:}",
            component_name.split(".").collect::<Vec<&str>>()[0]
        ))
        .expect("Error creating directory");
        fs::write(
            format!(
                "{:}/{:}",
                component_name.split(".").collect::<Vec<&str>>()[0],
                component_name
            ),
            component_file_content.clone(),
        )
        .expect("Error writing file");
        fs::write(
            format!(
                "{:}/{:}",
                component_name.split(".").collect::<Vec<&str>>()[0],
                stylesheet_name
            ),
            css_file_content.clone(),
        )
        .expect("Error writing file");
        fs::write(
            format!(
                "{:}/index.{:}",
                component_name.split(".").collect::<Vec<&str>>()[0],
                component_name.split(".").collect::<Vec<&str>>()[1]
            ),
            index_file_content.clone(),
        )
        .expect("Error writing file");

        println!(
            "{}",
            format!(
                "{} {:}/{:}",
                "CREATE".green(),
                component_name.split(".").collect::<Vec<&str>>()[0],
                component_name,
            )
            .bold()
        );
        println!(
            "{}",
            format!(
                "{} {:}/{:}",
                "CREATE".green(),
                component_name.split(".").collect::<Vec<&str>>()[0],
                stylesheet_name,
            )
            .bold()
        );
        println!(
            "{}",
            format!(
                "{} {:}/{:}",
                "CREATE".green(),
                component_name.split(".").collect::<Vec<&str>>()[0],
                format!(
                    "index.{:}",
                    component_name.split(".").collect::<Vec<&str>>()[1]
                ),
            )
            .bold()
        );
    }
}
