//! automation_tasks_rs for mem6_game workspace
//! The workspace contains 4 members: dodrio_templating, mem6, mem6_common, mem6_server

// Building on windows is just cumbersome. The result is only good for windows.
// The web server will always be on linux. Wasm/Webassembly is OS agnostic.
// It has much more sense to build always in Linux. 
// OS for code editing does not change anything.
// Building for both OS means I would need to call every utils exe and 
// every copy command in 2 different ways. So complicated.
// Not good.
// Decision: I will always build only for for Linux inside Linux subsystem for Windows !

use cargo_auto_lib::*;

// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
#[allow(dead_code)]
pub const RED: &str = "\x1b[31m";
#[allow(dead_code)]
pub const YELLOW: &str = "\x1b[33m";
#[allow(dead_code)]
pub const GREEN: &str = "\x1b[32m";
#[allow(dead_code)]
pub const RESET: &str = "\x1b[0m";


fn main() {
    exit_if_not_run_in_rust_project_root_directory();

    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args);
}

// region: match, help and completion

/// match arguments and call tasks functions
fn match_arguments_and_call_tasks(mut args: std::env::Args) {
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => print_help(),
        Some(task) => {
            if &task == "completion" {
                completion();
            } else {
                println!("{YELLOW}Running automation task: {task}{RESET}");
                if &task == "release" {
                    task_release();
                } else if &task == "doc" {
                    task_doc();
                } else if &task == "test" {
                    task_test();
                } else if &task == "commit_and_push" {
                    let arg_2 = args.next();
                    task_commit_and_push(arg_2);
                } else if &task == "publish_to_web" {
                    task_publish_to_web();
                } else {
                    println!("{RED}Error: Task {task} is unknown.{RESET}");
                    print_help();
                }
            }
        }
    }
}

/// write a comprehensible help for user defined tasks
fn print_help() {
    println!(
        r#"
    {YELLOW}Welcome to cargo-auto !
    This program automates your custom tasks when developing a Rust project.{RESET}

    User defined tasks in automation_tasks_rs:
cargo auto release - builds the crate in release mode, fmt, increment version
cargo auto doc - builds the docs, copy to docs directory
cargo auto test - runs all the tests
cargo auto commit_and_push "message" - commits with message and push with mandatory message
    (If you use SSH, it is easy to start the ssh-agent in the background and ssh-add your credentials for git.)
cargo auto publish_to_web - publish to my google VM, git tag
    (You need credentials for publishing. I use ssh-agent and ssh-add to store my credentials for SSH.)
"#
    );
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["release", "doc", "test", "commit_and_push","publish_to_web"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["x"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
}

// endregion: match, help and completion

// region: tasks


/// build release every member of workspace. One is wasm project, so instead of cargo build, I use wam-pack build
/// this workspace is basically one single application splitted into 3 projects
/// it deserves the same version number for the release build. It means that it will build all members. 
/// A little slower than only build.
fn task_release() {
    // delete_old_js_snippets
    // run_shell_command("rm -r pkg/snippets/*");

    // let cargo_toml = CargoToml::read();
    //auto_check_micro_xml("web_server_folder/mem6_game");
    auto_version_increment_semver_or_date_forced();
    // format and build all normal members
    run_shell_command("cargo fmt");
    run_shell_command("cargo build --release --workspace --exclude mem6");
    run_shell_command("rsync -avz --info=progress2 --delete-after target/release/mem6_server webfolder/");

    // mem6 is WASM and is excluded from workspace and needs separate commands
    run_shell_command("cd mem6; cargo fmt; wasm-pack build --target web --release;cd ..");
    // copy to web_server_folder/pkg
    run_shell_command("rsync -avz --info=progress2 --delete-after mem6/pkg/ webfolder/mem6/pkg/");    

    auto_cargo_toml_to_md();
    auto_lines_of_code("");
    // run_shell_command("strip target/release/mem6_game");
    
    println!(
        r#"{YELLOW}
    After `cargo auto release`, run the compiled binary, examples and/or tests
cd webfolder ; ./mem6_server; cd ..
    In the browser or in curl open 
http://localhost:8086
    if ok, then
cargo auto doc
{RESET}"#,
    );
}

/// cargo doc, then copies to /docs/ folder, because this is a github standard folder
fn task_doc() {
    let cargo_toml = CargoToml::read();
    auto_md_to_doc_comments();
    auto_plantuml(&cargo_toml.package_repository().unwrap());

    #[rustfmt::skip]
    let shell_commands = [
        "cargo doc --no-deps --document-private-items",
        // copy target/doc into docs/ because it is github standard
        "rsync -a --info=progress2 --delete-after target/doc/ docs/",
        "echo Create simple index.html file in docs directory",
        &format!("echo \"<meta http-equiv=\\\"refresh\\\" content=\\\"0; url={}/index.html\\\" />\" > docs/index.html",cargo_toml.package_name().replace("-","_")) ,
    ];
    run_shell_commands(shell_commands.to_vec());
    run_shell_command("cargo fmt");
    // message to help user with next task
    println!(
        r#"{YELLOW}
    After `cargo auto doc`, check `docs/index.html`. If ok, then test the documentation code examples
cargo auto test
{RESET}"#
    );
}

/// cargo test
fn task_test() {
    run_shell_command("cargo test");
    println!(
        r#"{YELLOW}
    After `cargo auto test`. If ok, then 
cargo auto commit_and_push "message"
    with mandatory commit message
{RESET}"#
    );
}

/// commit and push
fn task_commit_and_push(arg_2: Option<String>) {
    match arg_2 {
        None => println!("{RED}Error: Message for commit is mandatory.{RESET}"),
        Some(message) => {
            run_shell_command(&format!(r#"git add -A && git commit --allow-empty -m "{}""#, message));
            run_shell_command("git push");
            println!(
                r#"{YELLOW}
    After `cargo auto commit_and_push "message"`
cargo auto publish_to_web
{RESET}"#
            );
        }
    }
}

/// publish to web for podman container and git tag
fn task_publish_to_web() {
    println!(r#"{YELLOW}Use ssh-agent and ssh-add to store the credentials.{RESET}"#);
    let cargo_toml = CargoToml::read();
    // git tag
    let shell_command = format!(
        "git tag -f -a v{version} -m version_{version}",
        version = cargo_toml.package_version()
    );
    run_shell_command(&shell_command);

    // rsync files
    //run_shell_command("rsync -e ssh -a --info=progress2 ./target/release/mem6_game luciano_bestia@bestia.dev:/var/www/transfer_folder/mem6_game/");
    //run_shell_command("rsync -e ssh -a --info=progress2 ./.env luciano_bestia@bestia.dev:/var/www/transfer_folder/mem6_game/");
    //run_shell_command("rsync -e ssh -a --info=progress2 ./deploy/mem6_game_pod_create.sh luciano_bestia@bestia.dev:/var/www/transfer_folder/mem6_game/");

    println!(
        r#"{YELLOW}
    After `cargo auto publish_to_web`, 
    connect to the google VM bash using SSH.
ssh -i ~/.ssh/ssh_certificate username@domain -v
ssh -i ~/.ssh/luciano_googlecloud luciano_bestia@bestia.dev -v
    There run the bash scripts to create the image and to create the pod.
cd /var/www/transfer_folder/mem6_game
sh mem6_game_pod_create.sh
    Test the web application locally:
curl http://localhost:8011/mem6_game/get_svg_image/555555.svg
    Test the web application on the internet:
curl https://bestia.dev/mem6_game/get_svg_image/555555.svg
{RESET}"#
    );
}
// endregion: tasks
