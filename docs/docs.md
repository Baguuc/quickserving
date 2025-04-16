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
