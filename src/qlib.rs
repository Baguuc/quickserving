use axum::{
    response::Html,
    routing::{get, options},
    Router,
};
use serde_derive::Deserialize;
use std::{
    fs::{self, ReadDir},
    process::exit,
    vec::Vec,
};
use tokio::net::TcpListener;

fn get_pages(dir: &str) -> Vec<String> {
    let files = match fs::read_dir(dir) {
        Ok(x) => x,
        Err(_) => {
            println!("Error reading dir {}!\n Try using another, check if you have permissions this directory requires, also check if specified directory exists.", dir);
            exit(404);
        }
    };

    let mut total_files: Vec<String> = Vec::new();
    for file_unsafe in files {
        let file = match file_unsafe {
            Ok(x) => x,
            Err(x) => {
                println!("Something went wrong reading file {}, skipping.", x);
                exit(404);
            }
        };

        let path: String = file.path().display().to_string();
        if file.path().is_dir() {
            let path: &str = path.as_str();
            let nested_files: Vec<String> = get_pages(path);
            total_files.extend(nested_files);
            continue;
        }

        total_files.push(path);
    }

    let pages = total_files
        .into_iter()
        .filter(|file| file.ends_with(".html"))
        .collect();

    return pages;
}

fn get_static_files(dir: String) -> ReadDir {
    let mut static_dir = dir
        .trim_start_matches('.')
        .trim_start_matches('/')
        .to_string();

    if !static_dir.ends_with('/') {
        static_dir = format!("{}/", static_dir);
    }

    let files_read = match fs::read_dir(static_dir.clone()) {
        Ok(files) => files,
        Err(_) => {
            println!("Error reading dir {}!\n Try using another, check if you have permissions this directory requires, also check if specified directory exists.", dir);
            exit(404);
        }
    };

    return files_read;
}

#[derive(Deserialize)]
pub struct QServeOptions {
    pub port: String,
    pub dir: String,
    pub static_dir: String,
}

impl QServeOptions {
    pub fn get_config() -> QServeOptions {
        let config_content = match fs::read_to_string("./Quickserving.toml") {
            Ok(content) => content.to_string(),
            Err(_) => "".to_string(),
        };

        let config: QServeOptions = match toml::from_str(&(config_content.as_str())) {
            Ok(table) => table,
            Err(_) => {
                println!("Couldn't read config file Quickserving.toml, deafulting the values.");
                QServeOptions {
                    port: String::from("2500"),
                    dir: String::from("./my_project_html"),
                    static_dir: String::from("./my_project_html/static"),
                }
            }
        };

        return config;
    }

    pub fn to_string(&self) -> String {
        let string = format!(
            "port = \"{}\"\ndir = \"{}\"\nstatic_dir = \"{}\" ",
            self.port, self.dir, self.static_dir
        );
        return string;
    }
}

pub async fn qserve(options: QServeOptions) {
    let mut formattion = String::from("{%dir%}");

    /* add symbols to avoid substring creation errors.
     * if the dir supplied was just '.' symbol it would create problems with routing
     */
    if !options.dir.eq("./") && !options.dir.starts_with("./") {
        formattion = format!("./{}", formattion);
    }

    if !options.dir.eq("./") && !options.dir.ends_with("/") {
        formattion = format!("{}/", formattion);
    }

    let formatted = formattion.replace("{%dir%}", options.dir.as_str());

    let dir = formatted.as_str();

    let mut app: Router = Router::new();

    let pages = get_pages(dir);

    let read_resource = |resource| {
        // callback to read resource for request's response
        let callback = || async {
            let file_content = match fs::read_to_string(resource) {
                Ok(content) => content.to_string(),
                Err(_) => "".to_string(),
            };

            Html(file_content)
        };

        return callback.clone();
    };

    for page in pages {
        // format route for the page

        let route = (page.clone()).to_string().clone().replace(dir, "/");
        // clone page as resource to reuse
        let resource = page.clone();

        // route
        app = app.route(
            &route.clone().as_str(),
            get(read_resource(resource.clone())),
        );
        println!("Registered route: \"{}\"", route);

        let route = route.trim_end_matches(".html");

        app = app.route(&route, get(read_resource(resource.clone())));
        println!("Registered route: \"{}\"", route);

        if route.ends_with("/index") {
            let route_len = route.len();

            // route the alias with just slash instead of "/index" at the end
            let alias: String = (route.to_string()).chars().take(route_len - 5).collect();

            app = app.route(&alias, get(read_resource(resource.clone())));

            println!("also registered alias route: {}", alias);

            // create second alias without slash on the end
            let alias: String = (alias.clone()).chars().take(alias.len() - 1).collect();

            // check if second alias isn't empty
            if alias.len() > 1 {
                app = app.route(&alias, get(read_resource(resource.clone())));
                println!("also registered alias route: {}", alias);
            }
        }
    }

    let static_files = get_static_files(options.static_dir.clone());

    /*
     * route static files
     */
    for file_unsafe in static_files {
        let file = match file_unsafe {
            Ok(x) => x,
            Err(_) => {
                println!("Something went wrong reading file exiting.");
                exit(404);
            }
        };

        if file.path().is_dir() {
            continue;
        }

        let fullpath = file.path().display().to_string();
        let last_slash = match fullpath.rfind("/") {
            Some(pos) => pos,
            _ => 0,
        };

        let resource: String = fullpath.clone().chars().skip(last_slash + 1).collect();
        let path: String = fullpath
            .clone()
            .chars()
            .take(fullpath.len() - last_slash)
            .collect();

        let path: &str = path.as_str();
        let resource_path = path.replace(path, "/static");

        let route = format!("{}/{}", resource_path.clone(), resource.clone());

        app = app.route(&route, get(read_resource(fullpath.clone())));
    }

    println!(
        "Registered directory {} as static files folder.",
        options.static_dir.clone()
    );

    println!("\nServing app with settings:\n{}\n", options.to_string());

    let address = format!("0.0.0.0:{}", options.port);
    let address = address.as_str();

    let listener = match TcpListener::bind(address).await {
        Ok(listener) => listener,
        Err(_) => {
            println!(
                "Cannot bind server to address {}, try changing the port.",
                address
            );
            exit(400);
        }
    };
    axum::serve(listener, app).await.unwrap();
}
