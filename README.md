# Aokana Tools 苍彼工具包

[![Build](https://github.com/Flyfish233/aokana-tools-rust/actions/workflows/build.yml/badge.svg)](https://github.com/Flyfish233/aokana-tools-rust/actions/workflows/build.yml)

> I will be the first to fly beyond the sky-

**Aokana Tools** is a suite designed for the visual
novel [Aokana -Four Rhythms Across the Blue-](https://vndb.org/v12849) (蒼の彼方のフォーリズム), aimed at providing a
set of tools for extracting game assets and combining segmented CG images, with multithreading capabilities and operates
independently of external dependencies
such as `ffmpeg` or `ImageMagick`, ensuring an easy-to-use and light-weight experience.

With the goal of achieving high-performance extraction and elegantly simple code, these tools are developed in Rust.

## Toolkit

- **Unpacker (WIP)**

  The Unpacker facilitates the decryption and extraction of all game assets from .dat files, tailored for the
  Steam release of Aokana.

    - `Unpacker` specializes in unpack .dat files encrypted with the original method used in [Aokana -Four Rhythms
      Across the Blue-](https://store.steampowered.com/app/1044620/_/)
      and [Aokana EXTRA1](https://store.steampowered.com/app/1340130/_EXTRA1/).
    - `UnpackerEx` is updated to unpack .dat files with the newer encryption method found in [Aokana
      EXTRA2](https://store.steampowered.com/app/2206340/_EXTRA2/).

- **Merger**

  Aokana features character sprites that are split into several files. The Merger is here to put these pieces back
  together, reconstructing the full CG images.

  Please note: As the Decryptor is still being WIP, you will need to decrypt your files in advance using the
  Kotlin/JVM [aokana-tools-kt](https://github.com/Flyfish233/aokana-tools-kt) Unpacker.

    - **How to Use**

  To merge images, you can place the `merge` tool either in the parent directory or alongside your extracted asset
  folder, then execute it. CG images will be output to the `merged` directory.

  Make sure you've extracted `system.dat`, which contains `vcglist.csv`, as well as all the images that need to combine.

## 概述

> 能比任何人更快，到达天空的彼方——

**苍彼工具包**用来解包 Steam 版本 Galgame《蒼の彼方のフォーリズム》（苍之彼方的四重奏）加密的 .dat 游戏资源文件，并将分离的图像合并成正常
CG.

本企划旨在提供易于使用的多线程工具来以最快的速度解包和合并图像，并且不使用任何外部依赖，如 `ffmpeg` 或 `ImageMagick`
来处理图像以保证轻量和兼容。

本仓库使用 Rust 编写，确保最佳性能和精简。

## 工具集

- **解包器（开发中）**

  解包器用于解密和提取`.dat`文件中的所有游戏资源，专为《蒼の彼方のフォーリズム》(苍之彼方的四重奏) Steam 版本设计。

    - `Unpacker` 用于解包 [苍之彼方的四重奏](https://store.steampowered.com/app/1044620/_/)
      和 [苍之彼方的四重奏 EXTRA1](https://store.steampowered.com/app/1340130/_EXTRA1/) 中使用的加密方法的资源文件。
    - `UnpackerEx` 用于解包 [苍之彼方的四重奏 EXTRA2](https://store.steampowered.com/app/2206340/_EXTRA2/)
      中使用的新加密方法的资源文件。

- **合并**

  苍之彼方的四重奏中的 CG 被拆分为多个文件。合并工具用于这些碎片重新组合，重建完整的 CG 图像。

  由于解包器仍在开发中，您需要预先使用 Kotlin/JVM 版本的 [aokana-tools-kt](https://github.com/Flyfish233/aokana-tools-kt)
  解包器对文件进行解密。

    - **使用方法**

      把 `merge` 放在解包后的文件目录或父目录，运行它即可。CG 将输出于 `merged` 目录。

      确保您已提取了包含`vcglist.csv`的`system.dat`文件，以及需要合并的所有图像。