# Lightning-KV

Lightning-KV is a lightweight key–value store server written in Rust. Built with the high-performance [Actix-web](https://actix.rs/) framework, it exposes a simple RESTful API for basic CRUD (Create, Read, Update, Delete) operations.

## Features

- **Simple API:** Provides endpoints to create, read, update, and delete key–value pairs.
- **High Performance:** Utilizes the Actix-web framework for fast and efficient HTTP handling.
- **Lightweight:** Minimal dependencies and straightforward design.
- **Rust-Powered:** Built in Rust for memory safety and performance.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable version recommended)
- Cargo (comes with Rust)

## Installation

### Building Lightning-KV

1. **Clone the repository:**

    ```bash
   git clone https://github.com/0x002500/Lightning-KV.git
   cd Lightning-KV
   ```


2. **Build the project:**

    ```bash
   cargo build --release
   ```


3. **Run the server:**

    ```bash
   cargo run --release
   ```


    The server will start listening on `127.0.0.1:8080` as indicated by the startup message:  

    ```
   Server started at 127.0.0.1:8080
   ```


### Download from GitHub Actions

Alternatively, you can download the pre-built server binary from the GitHub Actions artifacts:

1. **Navigate to the repository's Actions tab:**

   - Go to the [Lightning-KV repository](https://github.com/0x002500/Lightning-KV).
   - Click on the **Actions** tab to view the list of workflows.

2. **Select the latest workflow run:**

   - Click on the most recent workflow run to view its details.

3. **Download the artifact:**

   - In the **Artifacts** section, find the artifact named `Lightning-KV` and click to download it.

4. **Run the server:**

    After downloading, extract the binary and run it directly:  

    ```bash
   ./Lightning-KV
   ```


    The server will start listening on `127.0.0.1:8080`.  

## API Endpoints

Lightning-KV provides the following REST endpoints:

### Create

- **Endpoint:** `POST /create`
- **Request Data:** JSON payload containing:
  - `key` (string): The unique identifier for the value.
  - `value` (string): The data to be stored.
- **Example Request:**

  
```bash
  curl -X POST -H "Content-Type: application/json" \
       -d '{"key": "username", "value": "john_doe"}' \
       http://127.0.0.1:8080/create
  ```


- **Description:** Creates a new key–value pair in the store. If the key already exists, the behavior might depend on your implementation (e.g., error out or overwrite).

### Read

- **Endpoint:** `GET /read`
- **Request Data:** Query parameter:
  - `key` (string): The key whose value is to be retrieved.
- **Example Request:**

  
```bash
  curl "http://127.0.0.1:8080/read?key=username"
  ```


- **Description:** Retrieves the value associated with the specified key. Returns the stored value in JSON format if the key exists.

### Update

- **Endpoint:** `POST /update`
- **Request Data:** JSON payload containing:
  - `key` (string): The unique identifier for the value.
  - `value` (string): The new data to update the existing key.
- **Example Request:**

  
```bash
  curl -X POST -H "Content-Type: application/json" \
       -d '{"key": "username", "value": "jane_doe"}' \
       http://127.0.0.1:8080/update
  ```


- **Description:** Updates the value for an existing key. If the key does not exist, you may want the server to return an error or create the key, depending on your desired behavior.

### Delete

- **Endpoint:** `GET /delete`
- **Request Data:** Query parameter:
  - `key` (string): The key to be deleted.
- **Example Request:**

  
```bash
  curl "http://127.0.0.1:8080/delete?key=username"
  ```


- **Description:** Deletes the key–value pair from the store. If the key does not exist, the server should handle the case appropriately (e.g., return an error message).

## Project Structure

- **src/**: Contains the source code.
  - **main.rs**: Sets up the Actix-web server and defines the API routes.
  - **services/**: Contains modules for each CRUD operation (`create`, `read`, `update`, `delete`).
- **Cargo.toml**: Contains project metadata and dependencies.

## Contributing

Contributions are welcome! If you have ideas for improvements or find any issues, please open an issue or submit a pull request.

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/my-feature`).
3. Commit your changes (`git commit -am 'Add new feature'`).
4. Push to your branch (`git push origin feature/my-feature`).
5. Open a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.
