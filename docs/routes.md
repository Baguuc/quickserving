## Docs
This page covers documentation about route configuration attributes.

# Route types
Routes are split into following types, each one is described below.

#### File route
The file route is used for serving a specific file to the client.
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
#### Text route
Text routes are just serving provided text.
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
Every route can define http-related rules (like returned headers, allowed methods for it etc) thru request and response options.
The request config defines the requirements that the recieved request should follow to get the 200 code from the route it's requesting,
while the response config defines how the response should be constructed.
More on HTTP definitions below. 

## Request
All of the request configuration options are described below.

#### Methods
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

## Response
All of the response configuration options are described below.

#### Headers
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
