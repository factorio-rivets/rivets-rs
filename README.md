# Rivets 🔩

Welcome to the Rivets, a Factorio modding written in Rust! Rivets injects code into the Factorio binary via DLL injection, providing a powerful toolset for modding and enhancing the game.

## Features

- **Procedural Macros**: Utilize idiomatic proc macros to overwrite or detour compiled Factorio functions.

Here is an example of how to detour the `main` function:
```rust
#[detour(main)]
fn main(argc: c_int, argv: *const c_char, envp: *const c_char) {
    info!("Detoured into main! 🦀🔩");
}
```
This will intercept the `main` function call and execute your custom logic, allowing you to modify the behavior of the game.

- **Seamless interop with Factorio Lua**: Mods written with Rivets can easily subscribe to lua events, and call `remote` API's to share data with Factorio mods written in Lua.

```rust
#[on_event(defines::events::on_player_died)]
fn on_player_died(player_index: u32, cause: LuaEntity) {
    let player: LuaPlayer = game.get_player(player_index);
    player.print("You just lost the game 💀");
}
```

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
See our open [ISSUES](https://github.com/notnotmelon/rivets/issues) if you are interested in becoming a contributor.

## License

This project is licensed under a custom license. See the [LICENSE](https://github.com/notnotmelon/rivets?tab=License-1-ov-file) file for details.
Any mods released using Rivets must have freely avalible source code.

```
1. Any modifications, adaptations, or derivative works (collectively "Mods")
   developed using the Software must be released under an open-source license
   as defined by the Open Source Initiative (OSI) (https://opensource.org/osd).
```

## Contact

For any questions or support, please open an issue on GitHub or use GitHub discussions.
We also have a community on [Discord](https://discord.gg/SBHM3h5Utj).

---

_This project is not affiliated with or endorsed by the developers of Factorio._

---

Happy modding!
