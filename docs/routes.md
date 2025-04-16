## Docs
This page covers documentation about route configuration attributes.

# Overall
Routes are split into following types, each one is described below.

## File route
The file route is used for serving a specific file for a server to a client.
Example:
```js
{
  "routes": {
    // this will result in looking for {directory attribute value}/index.html
    // if the client requests the "/" path
    // for example ./index.html or ./somesetdir/index.html etc.
    "/": {
      "type": "file",
      "source": "index.html"
    }
  }
}
```
## Text route
Text routes are just serving provided text with text/plain mimetype.
Example:
```js
{
  "routes": {
    // this will serve the text "Hello, World" with text/plain mimetype
    // on /hello route
    "/hello": {
      "type": "text",
      "text": "Hello, World!"
    }
  }
}
```

# HTTP
Every route can define http-related rule (like returned headers, allowed methods for it etc) thru HTTP config.
More on HTTP definitions below. 

## Methods
Every route can define what HTTP methods it allows to pass into itself.
Example:
```js
{
  "routes": {
    "/": {
      "http": {
        "methods": [ "GET" ]
      }
    }
  }
}
```

## Headers
Every route can define HTTP headers that will be sent as part of the response.
The Content-Length header will be overidden in file routes.
Example:
```js
{
  "routes": {
    "/": {
      "http": {
        "headers": {
          "Content-Type": "application/json"
        }
      }
    }
  }
}
```
