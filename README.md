# Rivets

Welcome to the Rivets, a Factorio DLL Injector written in rust! Rivets injects code into the Factorio binary via DLL injection, providing a powerful toolset for modding and enhancing the game.
Currently, our injector supports Windows, but we welcome contributions for Unix support.

## Features

- **DLL Injection**: Seamlessly inject your custom code into the Factorio binary.
- **Procedural Macros**: Utilize elegant proc macros to overwrite or detour functions with ease.

## Installation

1. **Clone the repository**:
    ```sh
    git clone https://github.com/notnotmelon/rivets.git
    cd rivets
    ```

2. **Build the project**:
    ```sh
    cargo build --release
    ```

3. **Inject the DLL**:
    Follow the instructions in the [Usage](#usage) section to inject your custom DLL into Factorio.

## Usage

### Detour Example

With the provided procedural macros, you can easily detour functions. Here is an example of how to detour the `main` function:

```rust
#[detour(main)]
fn main_detour(_argc: c_int, _argv: *const c_char, _envp: *const c_char) {
    info!("Detoured into main!");
}
```

This will intercept the `main` function call and execute your custom logic, allowing you to modify the behavior of the game seamlessly.

## Contributing

We are actively seeking contributors to help expand this project, particularly for adding Unix support. If you're interested in contributing, please fork the repository and submit a pull request. 

### Steps to Contribute:

1. **Fork the repository**
2. **Create a new branch** (`git checkout -b feature-branch`)
3. **Make your changes**
4. **Commit your changes** (`git commit -m 'Add some feature'`)
5. **Push to the branch** (`git push origin feature-branch`)
6. **Open a pull request**

We welcome contributions of all kinds, including bug fixes, new features, documentation improvements, and more.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For any questions or support, please open an issue on GitHub or contact us directly.

Happy modding!

---

_This project is not affiliated with or endorsed by the developers of Factorio._

---
