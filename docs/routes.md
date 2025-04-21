## Docs
This page covers documentation about route configuration attributes.

## Request data
Every request recieved by quickserving is tried to be matched against one of all the defined routes.
Each route can specify one or more responses for the same path, matching other request's data, for example methods.
Here is all the supported (at the time) data that the route is matching:
- HTTP Method (GET, POST etc)
Example:
```
{
  "port": 3000,
  "routes": {
    "/greet": [
      {
        "method": "POST",
        "response": {
          "type": "text",
          "text": "Hello, World!",
          "http": {
            "headers": {}
          }
        }
      },
      {
        "method": "DELETE",
        "response": {
          "type": "text",
          "text": "Goodbye, World!",
          "http": {
            "headers": {}
          }
        }
      }
    ]
  }
}
```
The above configuration will respond with "Hello, World!" text when requested with POST method, and "Goodbye World!" when requested with DELETE method, even tho they are on the same path (/greet).


## Response data
Each route have their own response.
Responses are split into multiple types.
The response type is determined by the "type" attribute in them".
Here are description of all response types:
+ text - route responds with text defined in the "text" attribute;
+ file - route tries to server the file existing at path from "source" attribute, returning 404 response when the file cannot be found;
Each response (even in the same path as shown before) can have different types.
Example:
```
{
  "port": 3000,
  "routes": {
    "/greet": [
      {
        "method": "POST",
        "response": {
          "type": "text",
          "text": "Hello, World!",
          "http": {
            "headers": {}
          }
        }
      },
      {
        "method": "DELETE",
        "response": {
          "type": "file",
          "source": "goodbye.txt",
          "http": {
            "headers": {}
          }
        }
      }
    ]
  }
}
```
This configuration will respond with "Hello" text when requested with POST method and "goodbye.txt" file when requested with the DELETE method.

## HTTP config
Each response can configure their own http metadata (headers etc).
To configure response's http metadata you can specify it in the "http" attribute.
Example:
```
{
  "port": 3000,
  "routes": {
    "/greet": [
      {
        "method": "GET",
        "response": {
          "type": "text",
          "text": "{ \"msg\": \"Hello, World!\" }",
          "http": {
            "headers": {
                "Content-Type": "application/json"
            }
          }
        }
      }
    ]
  }
}
