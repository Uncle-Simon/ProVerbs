# ProVerbs 

ProVerbs is a simple, interactive desktop application that helps users memorize verses from the book of Proverbs. It's built using Tauri, a framework for building lightweight, secure desktop applications with web technologies. The backend of the application is implemented in Rust.

## Features

- Interactive questions to test and improve your skills
- Provides a percentage score on how well you remembered.
- User-friendly interface

## Getting Started

To get started with ProVerbs, follow these steps:

1. Navigate to the [Releases](https://github.com/Uncle-Simon/ProVerbs/releases) page of the ProVerbs repository.
2. Download the latest release for your respective platform (Linux, Windows, or MacOS).
3. After downloading, extract the file and run the application.

Enjoy improving your Scriptural knowledge with ProVerbs!

## Building from Source

If you want to build the application from source, follow these steps:

### Requirements

Before you can build ProVerbs from source, you need to have the following software installed on your machine:

- [Node.js and npm](https://nodejs.org/en/download/)
- [Rust](https://www.rust-lang.org/tools/install)

### Building

1. Clone the repository to your local machine:

```bash
git clone https://github.com/Uncle-Simon/ProVerbs.git
```

2. Navigate into the project directory:

```bash
cd ProVerbs
```

3. Install the necessary dependencies:

```bash
npm install
```

4. Build the Tauri application:

```bash
npm run tauri build
```

After following these steps, you should have a built version of the NonCAR application on your local machine.

## Contributing

Any contributions you make are greatly appreciated.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## Acknowledgements

- Special thanks to Arul John for their work in formatting the Bible into JSON. The JSON-formatted Bible from [Arul John's Bible-kjv project](https://github.com/aruljohn/Bible-kjv) was utilized in this project under the terms of the MIT License.
- [Tauri](https://tauri.app/)
- [Node.js](https://nodejs.org/en/)
- [Rust](https://www.rust-lang.org/)
