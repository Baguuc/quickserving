# Quickserving - host your static html quickly

## Quickserving is a static html site server software to host your own static site easily.

### Here are some use cases to explain the usage:

- You've just created a static website (e.g. about me page) in html and you want to host it
- You need to host a static website with a very minimal and lightweight tool

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

2. Run the server

```bash
quickserving
```

### CLI Arguments

Quickserving have plenty of cli arguments to customize how the server is setup.

- -p, --port <PORT> The port that server will be listening for requests on. [default: 8080]

#

- -d, --directory <DIRECTORY> The directoryectory that will be served. [default: .]

#

- -h, --help List all the arguments avaible

#

- -V, --version See the installed version of Quickserving

### Aliases

Quickserving is aliasing every .html file to routes without .html file extension on the end
/index.html => / /index.html
/about/index.html => /about /about/ /about/index.html
/about/more_about.html => /about/more_about /about/more_about.html
etc.
