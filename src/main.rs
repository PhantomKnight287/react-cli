use clap::Parser;
use std::fs;

/// Command Line Interface to generate React Components.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the component to create
    #[clap(short, long)]
    name: String,

    /// Create a given stylesheet
    #[clap(short, long, default_value = "css")]
    stylesheet: String,

    /// Create Stylesheet module
    #[clap(short, long)]
    create_stylesheet_module: bool,

    /// Create TypeScript Component
    #[clap(short, long)]
    typescript: bool,
}

fn main() {
    let args = Args::parse();
    fs::create_dir(&args.name).expect("Failed to Create Folder");
    let component_file_name: String;
    let stylesheet_file_name: String;
    let component_file_component: String;
    let stylesheet_file_content = format!(".h1{{\n\ttext-align:center;\n}}");
    if args.typescript == true {
        component_file_name = "index.tsx".to_string();
    } else {
        component_file_name = "index.jsx".to_string();
    }

    if args.create_stylesheet_module == true {
        stylesheet_file_name = format!("{}.module.{}", args.name, args.stylesheet);
    } else {
        stylesheet_file_name = format!("{}.{}", args.name, args.stylesheet);
    }

    if stylesheet_file_name.contains("module") {
        component_file_component = format!(
            "import styles from \"./{}\";\nexport default function {}(){{\n\n\treturn(\n\t\t<h1 className={{styles.h1}} >{} works</h1>\n\t)\n}}",stylesheet_file_name,
            args.name, args.name
        );
    } else {
        component_file_component =format!(
            "import \"./{}\";\nexport default function {}(){{\n\n\treturn(\n\t\t<h1 className=\"h1\" >{} works</h1>\n\t)\n}}",stylesheet_file_name,
            args.name, args.name
        );
    }

    fs::write(
        format!("{}/{}", args.name, component_file_name),
        component_file_component,
    )
    .expect("Failed to Create Component File");
    fs::write(
        format!("{}/{}", args.name, stylesheet_file_name),
        stylesheet_file_content,
    )
    .expect("Failed To Create StyleSheet File");

    println!("Created {} Component", args.name);
}