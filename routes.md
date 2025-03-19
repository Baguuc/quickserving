## Docs
This page covers documentation about route configuration attributes.

## File (Required)
The file attribute is used to define the file that quickserving will try to serve when client
requests path it is bound to.
Example:
```json
{
  "routes": {
    // this will result in looking for {directory attribute value}/index.html
    // if the client requests the "/" path
    // for example ./index.html or ./somesetdir/index.html etc.
    "/": "index.html"
  }
}
```
