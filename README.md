# Simple Web Server with Thread Pool

This project is a basic web server implemented in Rust. It includes a custom thread pool for handling multiple client connections concurrently. The server responds to simple HTTP GET requests.

## Features

- **Thread Pool**: A custom thread pool to manage worker threads for handling client requests.
- **Graceful Shutdown**: Ensures all worker threads are properly shut down when the server stops.
- **Static File Serving**: Serves static HTML files in response to HTTP GET requests.

### Request Handling

The server handles the following requests:

- `GET /`: Responds with the content of `hello.html`.
- `GET /sleep`: Waits for 5 seconds and then responds with the content of `hello.html`.
- `Get /time`: Shows the time in a JSON format and also prints the same to a JSON file.
- Any other request: Responds with `404.html`.
