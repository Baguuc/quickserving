# Quickserving - host your static html quickly

## Quickserving is a static html site server software to host your own static site easily.

### Here are some use cases to explain the usage:
* You've just created a static website (e.g. about me page) in html and you want to host it
* You need to host a static website with a very minimal and lightweight tool

### Installing
1. Install prerequisites cargo and rustc from [official rust website](http://rust-lang.org)
2. Clone the repository 
```bash
git clone https://github.com/Baguuc/quickserving.git
```
3. Build the executable
```bash
cd path_to_cloned_repo
cargo build --release
```
4. Copy the executable target/release/quickserving to your systems applications path specified in enviroment variables.

### How to use
1. cd into directory you have your site's files in.
```bash
cd my_site_directory
```
2. Configure the server, more info in Configuration section of readme.
3. Run the server
```bash
quickserving
```

### Configuration
Quickserving has a basic TOML configuration file to configure your server for each project, to start create a file "Quickserving.toml" in your project directory.

###### IMPORTANT: Quickserving don't support partial config options, you will have to specify all options when configuring.

###### WARNING: when specifing directories, you are only able to use subfolders of you current location e.g. the project root is ~/quickserving_example so only possible directories are ~/quickserving_example/**/*


The avaible configuration options:
* dir - the main directory where all your html files are in, if you specify wrong dir the site could not be served correctly.
* static_dir - the directory where your styles, images, scripts etc. are located, if it's not specified your static files are not goind to work, they should be accessed regular way by using relative paths ("./", "../", "../.." etc.) if it is not working quickserving is also registering /static route for all the static files.
* port - the port server will be binded to.

Example Quickserving.toml:
```toml
port = "2500"
dir = "./my_project_html"
static_dir = "./my_project_html/static"
```
these are also the default values.

### Routing
Quickserving registers routes for all html files and static files, static files are always going to be in route /static, html files will be routed the same as the path they have, also the index are always going to be entry point of a directory  example:
* /index.html => /index.html, /index, /
* /about.html => /about.html, /about
* /portfolio/index.html => /portfolio/index.html, /portfolio/index, /portfolio

