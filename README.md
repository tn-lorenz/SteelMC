[![Rust][rust-shd]][rust-url]
[![License][license-shd]][license-url]
[![DeepWiki][dw-shd]][dw-url]
[![Tests][test-shd]][test-url]
[![Lint][lint-shd]][lint-url]
[![Build][build-shd]][build-url]
[![SteelMC][dc-shd]][dc-url]


<div align="center">

# Steel

![Logo](https://i.imgur.com/lFQ6jH2.png)

Steel is a lightweight Rust implementation of the Minecraft server.
It focuses on clean code, performance, extensibility, and ease of use.

---

![Demo](https://github.com/user-attachments/assets/ee656153-0660-4626-8295-37d3c96d8fd9)


</div>

---

## 🔗 Links
<div align="center">

[Discord](https://discord.gg/MwChEHnAbh) | [GitCraft](https://github.com/WinPlay02/GitCraft)
</div>

---

## ⚙ How to Contribute

1. Identify a feature you'd like to add or an issue to work on.
   You should always create a post in the channel [feature-discussion](https://canary.discord.com/channels/1428487339759370322/1429074039015473272) when considering adding a major feature.
2. Decompile Minecraft 26.1 by running the provided script:
   ```bash
   ./update-minecraft-src.sh
   ```
   This will clone GitCraft and generate the decompiled source in `minecraft-src/`.
3. Fork the `master` branch of this repository.
4. Examine the vanilla implementation and translate it into idiomatic Rust as cleanly and efficiently as possible.
5. Commit your changes to your fork and open a pull request.

> [!NOTE]
> It is highly recommended to join the [Discord server](https://discord.gg/MwChEHnAbh) and reach out to [4lve](https://github.com/4lve) if you have code-related questions or encounter any ambiguities.

> [!IMPORTANT]
> This project is still in a very early stage of development.

### Precommit Hook
This repository uses [prek](https://prek.j178.dev/) to ensure that all commits follow the style guide and makes sure the cicd will pass.
To install the hook, some things needed to be installed first:
```bash
cargo install prek typos-cli --locked
```

Then you can run `prek install` to install the hook and it is configured to run automatically before every commit.
It will fix some things already for you, but the commit will still fail and please check the changes.

[rust-url]: https://www.youtube.com/watch?v=cE0wfjsybIQ&t=73s
[rust-shd]: https://img.shields.io/badge/rust-%23000000.svg?style=plastic&logo=rust&logoColor=white

[license-url]: https://github.com/4lve/SteelMC/blob/master/LICENSE
[license-shd]: https://img.shields.io/github/license/4lve/SteelMC?label=License&labelColor=black&color=blue&logo=gpl

[dc-url]: https://discord.gg/MwChEHnAbh
[dc-shd]: https://dcbadge.limes.pink/api/server/MwChEHnAbh?style=social

[dw-url]: https://deepwiki.com/4lve/SteelMC
[dw-shd]: https://deepwiki.com/badge.svg

[test-shd]: https://img.shields.io/github/actions/workflow/status/4lve/SteelMC/test.yml?branch=main&logo=github&label=Test&labelColor=black
[test-url]: https://github.com/4lve/SteelMC/actions/workflows/test.yml

[lint-shd]: https://img.shields.io/github/actions/workflow/status/4lve/SteelMC/lint.yml?branch=main&logo=github&label=Lint&labelColor=black
[lint-url]: https://github.com/4lve/SteelMC/actions/workflows/lint.yml

[build-shd]: https://img.shields.io/github/actions/workflow/status/4lve/SteelMC/release.yml?branch=main&logo=github&label=Build&labelColor=black
[build-url]: https://github.com/4lve/SteelMC/actions/.github/workflows/release.yml

<!-- light mode
[test-shd]: https://github.com/4lve/SteelMC/actions/workflows/test.yml/badge.svg
[lint-shd]: https://github.com/4lve/SteelMC/actions/workflows/lint.yml/badge.svg
[build-shd]: https://github.com/4lve/SteelMC/actions/workflows/release.yml/badge.svg
-->
