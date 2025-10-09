# Spotify Sync CLI

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Spotify](https://img.shields.io/badge/Spotify-1ED760?style=for-the-badge&logo=spotify&logoColor=white)](https://developer.spotify.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

A goofy command-line tool for seamlessly transferring your Spotify library between accounts. Transfer your liked songs, playlists, and albums with a single command, im also working on a second version as an APP version for windows, mac or linux. Not android i suppose, we will see? i guess?

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Spotify Setup](#spotify-setup)
- [Usage](#usage)
- [How It Works](#how-it-works)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Account Management**: Log in to multiple Spotify accounts and manage them easily
- **Comprehensive Transfer**: Copy liked songs, playlists, and albums
- **Interactive Progress**: Real-time progress bars for transfer operations
- **Secure Authentication**: Uses Spotify's OAuth 2.0 flow for secure access
- **Data Persistence**: Safely stores authentication tokens for future use

## Installation

### Prerequisites

- Rust (1.70.0 or higher)
- Cargo (comes with Rust)
- A Spotify account
- Spotify API credentials (see [Spotify Setup](#spotify-setup))

### Building from Source

```bash
# Clone the repository
git clone https://github.com/laxenta/spotify-sync-CLI
cd spotify-sync-CLI

# Build the project
cargo build --release

# The binary will be available at
./target/release/spotify-sync
```

## Spotify Setup

1. Visit the [Spotify Developer Dashboard](https://developer.spotify.com/dashboard)
2. Log in with your Spotify account
3. Click "Create App"
4. Fill in the application details:
   - App name: `spotify-sync` (or any name you prefer)
   - Redirect URI: `http://localhost:8888/callback`
   - Website: (optional)
   - Description: (optional)
5. After creating the app, you'll see your:
   - Client ID
   - Client Secret (click "Show Client Secret" to reveal)
6. Create a `.env` file in the project root:
   ```env
   SPOTIFY_CLIENT_ID=your_client_id_here
   SPOTIFY_CLIENT_SECRET=your_client_secret_here
   ```

## Usage

### Logging In to Accounts

```bash
# Log in to your source account (the one you want to copy from)
spotify-sync login source

# Log in to your target account (the one you want to copy to)
spotify-sync login target
```

### Listing Connected Accounts

```bash
spotify-sync list
```

### Previewing Transfer

```bash
# See what will be transferred from the source account
spotify-sync preview source
```

### Performing the Transfer

```bash
# Transfer everything from source to target account
spotify-sync transfer source target
```

## How It Works

The application follows this process:

1. **Authentication**
   - Uses Spotify's OAuth 2.0 flow
   - Opens your browser for login
   - Securely stores tokens in `~/.spotify-sync/tokens.json`

2. **Library Analysis**
   - Scans source account for:
     - Liked songs
     - Playlists
     - Albums

3. **Transfer Process**
   - Copies liked songs
   - Creates new playlists in target account
   - Preserves playlist metadata (names, descriptions, public/private status)
   - Copies all tracks to new playlists
   - Saves albums to target library

4. **Progress Tracking**
   - Shows real-time progress bars
   - Displays item counts and estimated time remaining
   - Provides clear success/failure feedback

## Development

### Project Structure

```
spotify-sync/
├── Cargo.toml           # Project dependencies
├── .env                 # API credentials
└── src/
    ├── main.rs          # CLI entry point
    ├── spotify/         # Spotify API interaction
    │   ├── auth.rs      # OAuth handling
    │   ├── client.rs    # API client
    │   └── models.rs    # Data structures
    ├── storage/         # Data persistence
    │   └── tokens.rs    # Token management
    └── transfer/        # Transfer logic
        └── sync.rs      # Core transfer code
        AND MORE, will update as project goes on
```

### Building for Development

```bash
cargo run -- [command]

cargo test

cargo fmt -- --check

cargo clippy
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
https://developer.spotify.com/dashboard