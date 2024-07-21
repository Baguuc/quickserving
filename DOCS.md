## Quickserving-core docs

- quickserving-core::config::Config (struct)
  It represents the configuration options for the files serving.

  - port (field) - represents the port that will be used to launch the HTTP server on.
  - directory (field) - represents the root directory that contains the files to serve.

- quickserving-core::request::Request (struct)
  It represents a HTTP request.

  - from_string (function) - parses requests data from string
  - to_string (function) - returns string created from data of the request
  - Other fields are HTTP request headers

- quickserving-core::response::Response (struct)
  It represents a HTTP response.

  - from_string (function) - parses response data from string
  - to_string (function) - returns string created from data of the response
  - Other fields are HTTP response headers

[here is a list of HTTP headers](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers)

- quickserving-core::server::listen (function) <br>
  starts the HTTP server. <br>
  parameters:
  - config: quickserving-core::config::Config
