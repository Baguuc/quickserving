# Docs
This page covers documentation about quickserving.json configuration file.

## Port (Optional)
The port attribute sets the port the server will try to listen on.
Default: 3000
Example:
```js
{
  "port": 5001
}
```
## Directory (Optional)
The directory attribute is used to reference the root directory all file will be searched for in.
Default: ./ (current dir)
Example:
```js
{
  // this will be the root of the project
  "directory": "/www/mywebsite"
}
```

## Index file (Optional)
The index_file attribute is used to set the alternative name to index.html file
that will be served in automatically while requesting a path.
Default: index.html
Example:
```js
{
  // when entering /somepath in the browser without specifying the file name
  // the server will try to serve this file in the path
  "index_file": "main.html"
}
```

## Routes (Optional)
The routes attribute is used to defined set of routes that will be looked up to find out how to serve the resource requested by the client.
If not set the whole application is useless.
For details about all the attributes inside every route, check out [routes.md](routes.md)
Default: {}
Example
```js
{
  "routes": {
    // this will result in looking for {directory attribute value}/index.html
    // if the client requests the "/" path
    // for example ./index.html or ./somesetdir/index.html etc.
    "/": {
      "type": "text",
      "source": "index.html"
    }
  }
}
```
