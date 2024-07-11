<div align="center">
    <a href="https://github.com/notnotmelon/rivets">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://forums.factorio.com/images/ext/20b0f93fa0933e4aa7df8592f124d153.png">
        <img alt="Rivets 🔩 - the Factorio mod loader" width="75%" style="max-width: 600px" src=".github/assets/logo-horizontal.png">
    </picture>
    </a>

[![Discord](https://img.shields.io/discord/1260754935952314418?color=lightblue&label=Community%20Chat&logo=Discord&logoColor=aqua)](https://discord.gg/xRYEZYz5WR)
![Downloads](https://img.shields.io/crates/d/rivets)
[![](https://img.shields.io/badge/License-Rivets_2024-green)](https://github.com/notnotmelon/rivets/blob/master/LICENSE.md)

</div>

# Rivets 🔩

Welcome to Rivets, a Factorio mod loader written in Rust! Rivets injects code into the Factorio binary via DLL injection, providing a powerful toolset for modding and enhancing the game.

Mods written in Rivets have access to functionality not possible within the traditional Lua API.
- Directly modifiy functionality in the compiled Factorio executable.
- Add new prototypes such as [LinkedRail](https://mods.factorio.com/mod/linked-rail).
- Access to a huge library of crates via the Rust [package manager](https://crates.io/).
- Go blazingly fast with Rust memory safety guarantees and multithreading.
- A superset of all the functionality possible within the vanilla Lua scripting language.

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
    let player: LuaPlayer = game::get_player(player_index);
    player.print("You just lost the game 💀");
}
```

- **Leverage existing modding infrastructure**: Rivets' design goal is to not reinvent the wheel. Mods written in Rust can be hosted on the offical Factorio modding website. Rivets is also designed to easily allow both Lua and Rust in the same Factorio mod.

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
See our open [issues](https://github.com/notnotmelon/rivets/issues) if you are interested in becoming a contributor.

## Credits

A huge thank-you to the following crates, without which this project would not be possible.
- [Retour](https://crates.io/crates/retour) - Allows directly modifying Factorio's ASM to detour functions.
- [DLL Syringe](https://crates.io/crates/dll-syringe) - Injects .DLL files into the Factorio executable.
- [PDB](https://crates.io/crates/pdb) - Used to parse the .PDB file format and allows debug symbols to be read on Windows.
- [Gimli](https://crates.io/crates/gimli) - Used to parse the .DWARF file format and allows debug symbols to be read on *nix systems.
- [Bindgen](https://crates.io/crates/bindgen) - Automatically generates Rust bindings for Factorio datatypes defined in the .pdb

## License

This project is licensed under a custom license. See the [license](https://github.com/notnotmelon/rivets?tab=License-1-ov-file) file for details.
Any mods released using Rivets must have freely available source code.

```
1. Any modifications, adaptations, or derivative works (collectively "Mods")
   developed using the Software must be released under an open-source license
   as defined by the Open Source Initiative (OSI) (https://opensource.org/osd).
```

## Contact

For any questions or support, please open an issue on GitHub or use GitHub discussions.

---

_This project is not affiliated with or endorsed by the developers of Factorio._

---

Happy modding!
