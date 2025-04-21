# Docs
This page covers documentation about quickserving.json configuration file.

## Port
The port attribute sets the port the server will try to listen on.
Example:
```
{
  "port": 5001
}
```

## Routes
The routes attribute is used to defined set of routes that will be looked up to find out how to serve the resource requested by the client.
For details about all the attributes inside every route, check out [routes.md](routes.md)
Example
```
{
  "routes": { ... }
}
```
