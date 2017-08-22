extern crate ansi_term;
extern crate csv;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

use std::fs;
use std::process::Command;
use std::path::Path;
use std::path::PathBuf;
use ansi_term::Colour::Red;
use ansi_term::Colour::Green;

#[derive(Clone, Debug, Deserialize)]
struct Repo {
    location: String,
    repo: String,
}

fn main() {
    use clap::App;
    use std::fs::File;

    // parse cli
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    // read csv
    let location = matches.value_of("location").expect("should be there");
    print!("Reading file [{}]\n", location);

    let file = File::open(location).expect("should be there");

    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let repo: Repo = result.expect("a serializable line ...");
        update_repo(repo)
    }
}

/// check out repo's if no already there otherwise update
/// TODO call this method in different threads
/// TODO handle expection ;-)
fn update_repo(repo: Repo) {
    // check if location exits

    let mut path = PathBuf::new();
    path.push("/home/jeroen/sourcetest/");
    path.push(repo.location.clone());

    if (path.exists()) {
//        let r: &str  = repo;
        println!("{} {}", repo.location , Green.paint(" already exists"));
        path.push(".git");
        if (path.exists()) {
            // with .git ? just print status
            path.pop();

            let output = Command::new("git")
                .arg("status")
                .current_dir(path)
                .args(&["-b","-s"])
                .output()
                .expect("failed to execute process");

            println!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            println!("{}{}", Red.paint("Expected only repository locations... skipping: "), repo.location)
        }
    } else {
        // no
        // git clone
        println!("repo does not exist yet at path [{:?}]", path);
        fs::create_dir_all(path.as_path()).expect("should be created");

        let x = path.to_str().expect("should have been created by now");

        let output = Command::new("git")
            .arg("clone")
            .current_dir(&path)
            .args(&[repo.repo, x.to_string()])
            .output()
            .expect("failed to execute process");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}
