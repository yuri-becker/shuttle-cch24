[![Unlicense license](https://img.shields.io/github/license/yuri-becker/shuttle-cch24?style=for-the-badge&logo=unlicense&logoColor=white
)](https://github.com/yuri-becker/shuttle-cch24/blob/main/LICENSE)

<br />
<div align="center">

  <h1 align="center"><strong>shuttle-cch24</strong></h1>

  <p align="center">
    My solutions for the <a href="https://www.shuttle.dev/cch"> Shuttle Christmas Code Hunt 2024</a>.
  </p>
</div>

## Content

Some code may serve as examples for how to do something specific in Rust. For future reference, I summarized the
challenges. Every challenge uses [rocket](https://crates.io/crates/rocket) and [serde](https://crates.io/crates/serde).

| File                                                                                    | Topics                                             | Crates                                                                                                       |
|-----------------------------------------------------------------------------------------|----------------------------------------------------|--------------------------------------------------------------------------------------------------------------|
| [day2.rs](https://github.com/yuri-becker/shuttle-cch24/blob/main/src/day2.rs)           | IP parsing and overflowing additions               |                                                                                                              |
| [day5.rs](https://github.com/yuri-becker/shuttle-cch24/blob/main/src/day5.rs)           | Cargo Manifest parsing from different file formats | [cargo_manifest](https://crates.io/crates/cargo_manifest), [serde_yaml](https://crates.io/crates/serde_yaml) |
| [day9.rs](https://github.com/yuri-becker/shuttle-cch24/blob/main/src/day9.rs)           | Leaky Bucket rate limiting                         | [leaky_bucket](https://crates.io/crates/leaky_bucket)                                                        |
| [day12/mod.rs](https://github.com/yuri-becker/shuttle-cch24/blob/main/src/day12/mod.rs) | Connect 4, seeded random                           | [rand](https://crates.io/crates/rand)                                                                        |

## Usage

```sh
 git clone git@github.com:yuri-becker/shuttle-cch24.git
 cd shuttle-cch24
 cargo install cargo-shuttle
 shuttle run 
 ```
