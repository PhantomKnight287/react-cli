pub mod hooks_file_creator {
    use colored::Colorize;
    use std::fs;
    pub fn hooks_file_creator(hook_file_name: String) {
        let result = fs::create_dir(hook_file_name.clone().split(".").collect::<Vec<&str>>()[0]);
        match result {
            Ok(_) => {
                println!(
                    "{}",
                    format!(
                        "{} {}",
                        "Created Directory:".green(),
                        hook_file_name.split(".").collect::<Vec<&str>>()[0]
                    )
                );
            }
            Err(_) => {
                println!(
                    "{}",
                    format!(
                        "{} {}",
                        "Directory already exists:".red(),
                        hook_file_name.split(".").collect::<Vec<&str>>()[0]
                    )
                );
                return;
            }
        }
        let result = fs::write(
            format!(
                "{:}/{:}",
                hook_file_name.clone().split(".").collect::<Vec<&str>>()[0],
                hook_file_name
            ),
            format!(
                "export default function {:}(){{}}",
                hook_file_name.clone().split(".").collect::<Vec<&str>>()[0]
            ),
        );
        match result {
            Ok(_) => {
                println!(
                    "{}",
                    format!("{} {}", "Created File:".green(), hook_file_name)
                );
            }
            Err(e) => {
                println!("{}", format!("{} {}", "Error:".red(), e));
                return;
            }
        }
        let result = fs::write(
            format!(
                "{}/index.{:}",
                hook_file_name.clone().split(".").collect::<Vec<&str>>()[0],
                hook_file_name.clone().split(".").collect::<Vec<&str>>()[1]
            ),
            format!(
                "export {{default}} from \"./{:}\";",
                hook_file_name.clone().split(".").collect::<Vec<&str>>()[0]
            ),
        );
        match result {
            Ok(_) => {
                println!("{}", format!("{} {}", "Created File:".green(), "index.js"));
            }
            Err(e) => {
                println!("{}", format!("{} {}", "Error:".red(), e));
                return;
            }
        }
    }
}
