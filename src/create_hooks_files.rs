pub mod hooks_file_creator {
    use colored::Colorize;
    use std::fs;
    pub fn hooks_file_creator(hook_file_name: String) {
        fs::create_dir(hook_file_name.clone().split(".").collect::<Vec<&str>>()[0])
            .expect("Error creating directory");
        fs::write(
            format!(
                "{:}/{:}",
                hook_file_name.clone().split(".").collect::<Vec<&str>>()[0],
                hook_file_name
            ),
            format!(
                "export default function {:}(){{}}",
                hook_file_name.clone().split(".").collect::<Vec<&str>>()[0]
            ),
        )
        .expect(format!("Error Creating {:}", hook_file_name).as_str());
        println!(
            "{}",
            format!(
                "{} {:}/{:}",
                "CREATE".green(),
                hook_file_name.clone().split(".").collect::<Vec<&str>>()[0],
                hook_file_name,
            )
            .bold()
        );
    }
}
