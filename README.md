# HyperPM

HyperPM is a fast, Rust-based npm package manager designed to optimize and streamline the process of managing JavaScript packages.

## Features

- **Concurrent Package Installation**: Install multiple packages simultaneously, significantly reducing total installation time.
- **User-Friendly Interface**: Colored output and progress bars for a better user experience.
- **npm Compatibility**: Seamlessly works with existing npm projects.
- **Project Initialization**: Quickly create new npm projects.

## Installation

To install HyperPM, you need to have Rust and Cargo installed on your system. If you don't have them, you can install them from [https://rustup.rs/](https://rustup.rs/).

Once you have Rust and Cargo, you can install HyperPM by following these steps:

1. Clone the repository:
   ```
   git clone https://github.com/hyperpmrs/hyperpm.git
   ```

2. Navigate to the project directory:
   ```
   cd hyperpm
   ```

3. Build and install the project:
   ```
   cargo install --path .
   ```

## Usage

### Creating a New Project

To create a new npm project:

```
hyperpm new my-project
```

This will create a new directory called `my-project` and initialize an npm project inside it.

### Installing Packages

To install one or more packages:

```
hyperpm install package1 package2 package3
```

This will concurrently install the specified packages, displaying progress for each installation.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the Apache License, Version 2.0. See the [LICENSE](LICENSE) file for details.

```
Copyright 2024 HyperPM

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```