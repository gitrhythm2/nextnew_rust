use std::env;
use std::fs;
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut project_name = String::new();
    let mut use_tailwind = true;

    for arg in &args[1..] {
        match arg.as_str() {
            "-h" => {
                println!("Usage: next-new [--ignore-tailwind] [project-name]");
                std::process::exit(0);
            }
            "--ignore-tailwind" => {
                use_tailwind = false;
            }
            _ => {
                project_name = arg.clone();
            }
        }
    }

    if project_name.is_empty() {
        eprintln!("プロジェクト名を指定してください");
        std::process::exit(1);
    }

    Command::new("pnpx")
        .arg("create-next-app@latest")
        .arg("--src-dir")
        .arg("--ts")
        .arg("--app")
        .arg("--tailwind")
        .arg("--use-pnpm")
        .arg("--eslint")
        .arg("--import-alias")
        .arg("@/*")
        .arg(if use_tailwind { "--tailwind" } else { "" })
        .arg(&project_name)
        .stdout(Stdio::inherit())  // 標準出力をプロセスに接続
        .spawn()  // 子プロセスを非同期に実行
        .expect("Failed to execute command")
        .wait()  // 子プロセスの終了を待つ
        .expect("Failed to wait on child");

    let current_dir = env::current_dir().unwrap();
    let project_dir = current_dir.join(&project_name);
    env::set_current_dir(&project_dir).unwrap();

    let css_content = "\
@tailwind base;
@tailwind components;
@tailwind utilities;
";
    let css_path = project_dir
        .join("src")
        .join("app")
        .join("globals.css");
    fs::write(css_path, css_content)
        .expect("Unable to write file");

    let page_content = "\
export default function Home() {
  return (
    <div className='p-6'>Hello Next!</div>
  );
}
";
    let page_path = project_dir
        .join("src")
        .join("app")
        .join("page.tsx");
    fs::write(page_path, page_content)
        .expect("Unable to write file");
}
