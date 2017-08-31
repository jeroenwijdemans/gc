
extern crate csv;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

#[macro_use(slog_o, slog_info, slog_log, slog_debug, slog_trace, slog_record, slog_record_static, slog_b, slog_kv)]
extern crate slog;
#[macro_use]
extern crate slog_scope;
extern crate slog_term;
extern crate slog_async;

use std::fs;
use std::process::Command;
use std::path::PathBuf;

use slog::Drain;
use clap::App;
use std::fs::File;

#[derive(Clone, Debug, Deserialize)]
struct Repo {
    location: String,
    repo: String,
}

fn main() {

    // parse cli
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    // configure logger
    let min_log_level = match matches.occurrences_of("verbose") {
        0 => slog::Level::Error,
        1 => slog::Level::Info,
        2 => slog::Level::Debug,
        3 | _ => slog::Level::Trace,
    };

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

//    let drain  = slo
    // g::Duplicate(drain, min_log_level);


    let drain = slog::LevelFilter(drain, min_log_level).fuse();

    let log = slog::Logger::root(drain, slog_o!());


    // Make sure to save the guard, see documentation for more information
    let _guard = slog_scope::set_global_logger(log);
    slog_scope::scope(&slog_scope::logger().new(slog_o!("scope" => "1")),
                      || true
    );

    info!("app_setup");
    trace!("app_setup");


    // read csv
    let location = matches.value_of("location").expect("should be there");
    debug!("Reading file [{}]\n", location);

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
    let mut path = PathBuf::new();
    path.push("/home/jeroen/sourcetest/");
    path.push(repo.location.clone());

    if path.exists() {
        debug!("{} {}", repo.location , " already exists");
        path.push(".git");
        if path.exists() {
            // with .git ? just print status
            path.pop();

            let output = Command::new("git")
                .arg("status")
                .current_dir(path)
                .args(&["-b","-s"])
                .output()
                .expect("failed to execute process");

            debug!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            debug!("{}{}", "Expected only repository locations... skipping: ", repo.location)
        }
    } else {
        // no
        // git clone
        debug!("repo does not exist yet at path [{:?}]", path);
        fs::create_dir_all(path.as_path()).expect("should be created");

        let x = path.to_str().expect("should have been created by now");

        let output = Command::new("git")
            .arg("clone")
            .current_dir(&path)
            .args(&[repo.repo, x.to_string()])
            .output()
            .expect("failed to execute process");
        debug!("{}", String::from_utf8_lossy(&output.stdout));
    }
}
