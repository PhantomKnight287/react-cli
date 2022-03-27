mod create_component_files;
use crate::create_component_files::files_creater;
mod create_hooks_files;
use crate::create_hooks_files::hooks_file_creator;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The name of the component you want to create
    #[clap(short, long)]
    name: String,

    /// Create a typescript component
    #[clap(long)]
    typescript: bool,
    /// Same as --typescript
    #[clap(long)]
    ts: bool,

    /// Create a css module instead of global css
    #[clap(short, long)]
    module: bool,

    /// Number of times to greet
    #[clap(long)]
    hook: bool,
}

fn main() {
    let Args {
        name: component_name,
        typescript,
        ts,
        module: css_module,
        hook,
    } = Args::parse();

    if hook {
        let hook_file_name;
        if typescript {
            hook_file_name = format!("{:}.jsx", hook);
        } else {
            hook_file_name = format!("{:}.tsx", hook);
        }
        hooks_file_creator::hooks_file_creator(hook_file_name);
        return;
    }
    let stylesheet: String;
    let component_file_name;
    if css_module {
        stylesheet = format!("{:}.module.css", component_name);
    } else {
        stylesheet = format!("{:}.css", component_name);
    }
    if typescript || ts {
        component_file_name = format!("{:}.tsx", component_name);
    } else {
        component_file_name = format!("{:}.jsx", component_name);
    }
    files_creater::components_files_creater(
        component_file_name,
        stylesheet.clone(),
        stylesheet.contains("module"),
    );
}
