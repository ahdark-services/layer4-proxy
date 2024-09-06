# Layer 4 Proxy

A simple layer 4 proxy that forwards TCP & UDP connections to a remote server.

## Usage

Create a `config.toml` file in the root directory of the project with the following content:

```toml
[[forward]]
listen_host = "0.0.0.0"
listen_port = 8080
remote_host = "0.0.0.0"
remote_port = 8081

[[forward]]
listen_host = "0.0.0.0"
listen_port = 8082
remote_host = "1.1.1.1"
remote_port = 80
```

Run the proxy:

```sh
cargo build --release
chmod +x target/release/layer-4-proxy
./target/release/layer-4-proxy
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
