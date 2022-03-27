#[allow(non_snake_case)]
pub mod files_creater {
    use colored::Colorize;
    use std::fs;
    pub fn components_files_creater(
        componentName: String,
        stylesheetName: String,
        isCssModule: bool,
    ) {
        let index_file_content = format!(
            "export * from \"./{}\";",
            componentName.split(".").collect::<Vec<&str>>()[0]
        );
        let css_file_content;
        let component_file_content;
        if isCssModule == true {
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
                stylesheetName,
                componentName.split(".").collect::<Vec<&str>>()[0]
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
                stylesheetName,
                componentName.split(".").collect::<Vec<&str>>()[0]
            );
        }
        fs::create_dir(format!(
            "{:}",
            componentName.split(".").collect::<Vec<&str>>()[0]
        ))
        .expect("Error creating directory");
        fs::write(
            format!(
                "{:}/{:}",
                componentName.split(".").collect::<Vec<&str>>()[0],
                componentName
            ),
            component_file_content.clone(),
        )
        .expect("Error writing file");
        fs::write(
            format!(
                "{:}/{:}",
                componentName.split(".").collect::<Vec<&str>>()[0],
                stylesheetName
            ),
            css_file_content.clone(),
        )
        .expect("Error writing file");
        fs::write(
            format!(
                "{:}/index.{:}",
                componentName.split(".").collect::<Vec<&str>>()[0],
                componentName.split(".").collect::<Vec<&str>>()[1]
            ),
            index_file_content.clone(),
        )
        .expect("Error writing file");

        println!(
            "{}",
            format!(
                "{} {:}/{:}",
                "CREATE".green(),
                componentName.split(".").collect::<Vec<&str>>()[0],
                componentName,
            )
            .bold()
        );
        println!(
            "{}",
            format!(
                "{} {:}/{:}",
                "CREATE".green(),
                componentName.split(".").collect::<Vec<&str>>()[0],
                stylesheetName,
            )
            .bold()
        );
        println!(
            "{}",
            format!(
                "{} {:}/{:}",
                "CREATE".green(),
                componentName.split(".").collect::<Vec<&str>>()[0],
                format!(
                    "index.{:}",
                    componentName.split(".").collect::<Vec<&str>>()[1]
                ),
            )
            .bold()
        );
    }
}
