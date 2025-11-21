# HotDog 🌭

A fullstack Dioxus application for browsing and saving random dog images from the Dog CEO API. Built with Rust, featuring server functions for backend logic, and styled with Tailwind CSS.

## Project Structure

```text
hot_dog/
├─ assets/             # Static assets (CSS, images, etc.)
├─ src/
│  ├─ main.rs          # Application entry point (client & server)
│  ├─ components/      # UI components
│  │  ├─ mod.rs        # Component module exports
│  │  ├─ app.rs        # Main App component
│  │  ├─ title.rs      # Title component and state
│  │  └─ dog_view.rs   # Dog image viewer component
│  ├─ apis/            # Server functions and API layer
│  │  ├─ mod.rs        # API module exports
│  │  ├─ save_dog.rs   # Server function to persist dog images
│  │  └─ server_utils.rs # Server-only utilities and helpers
├─ Cargo.toml          # Project dependencies and configuration
├─ Dioxus.toml         # Dioxus configuration
└─ tailwind.css        # Tailwind CSS configuration
```

## Features

- 🐕 Fetches random dog images from [Dog CEO API](https://dog.ceo/api/breeds/image/random)
- 💾 Server functions for persisting favorite dog images
- 🔒 Server-only code with proper security boundaries
- 🎨 Styled with Tailwind CSS
- ⚡ Built with Dioxus 0.7 fullstack framework
- 📦 Modular component architecture
- 🌐 Runs as web app, desktop app, or fullstack server

## Development

### Prerequisites

- Rust (latest stable version)
- Dioxus CLI (`cargo install dioxus-cli`)

### Automatic Tailwind (Dioxus 0.7+)

As of Dioxus 0.7, there's no need to manually install Tailwind. Simply run `dx serve` and you're good to go!

Automatic Tailwind is enabled by the `tailwind.css` file in your project root. To customize Tailwind configuration, edit `Dioxus.toml`:

```toml
[application]
tailwind_input = "tailwind.css"
tailwind_output = "assets/out.css"
```

### Manual Tailwind Installation (Optional)

For advanced Tailwind features like plugins or custom configurations:

1. Install npm: <https://docs.npmjs.com/downloading-and-installing-node-js-and-npm>
2. Install Tailwind CSS CLI: <https://tailwindcss.com/docs/installation/tailwind-cli>
3. Run the Tailwind compiler:

```bash
npx @tailwindcss/cli -i ./tailwind.css -o ./assets/tailwind.css --watch
```

### Running the App

#### Client-side Mode (Desktop/Web)

Start the development server with the default platform:

```bash
dx serve
```

To run on a specific platform:

```bash
dx serve --platform desktop  # Desktop app
dx serve --platform web       # Web app (client-side only)
```

#### Fullstack Mode (Server + Client)

Run the app in fullstack mode with server functions enabled:

```bash
dx serve --features server
```

This starts an Axum server that:

- Serves the frontend application
- Handles server function requests at `/api/*` endpoints
- Provides backend logic for saving dog images

## Architecture

### Fullstack Overview

HotDog uses Dioxus 0.7's fullstack capabilities to provide a seamless client-server architecture:

- **Client code**: Components render in the browser/desktop and call server functions
- **Server functions**: Backend logic marked with `#[post("/api/...")]` that only runs on the server
- **Server-only code**: Utilities in `server_utils.rs` protected by `#[cfg(feature = "server")]`

### Server Functions

#### `save_dog` ([apis/save_dog.rs](src/apis/save_dog.rs))

Server function that persists dog image URLs to storage:

- **Endpoint**: `POST /api/save_dog`
- **Input**: Dog image URL string
- **Output**: Success or error
- **Security**: Validates URLs before saving, server-only logging

Example usage from client code:

```rust
use crate::apis::save_dog;

// Call server function from client
save_dog(image_url).await?;
```

## Component Overview

### App ([components/app.rs](src/components/app.rs))

The main application component that sets up the context provider and renders the Title and DogView components.

### Title ([components/title.rs](src/components/title.rs))

Displays the application title with emoji. Uses context to access the title state.

### DogView ([components/dog_view.rs](src/components/dog_view.rs))

The main feature component that:

- Displays a dog image
- Provides "skip" and "save!" buttons
- Fetches new random dog images from the Dog CEO API
- Calls the `save_dog` server function when saving favorites

## Building for Production

### Client-only Build

Build optimized binaries for desktop or web:

```bash
dx build --release --platform desktop
dx build --release --platform web
```

### Fullstack Build

Build with server capabilities:

```bash
dx build --release --features server
```

## Deployment

For fullstack deployment, the built binary includes both the Axum web server and the compiled frontend. Simply run:

```bash
./target/release/hot_dog
```

The server will start and serve the application on the configured port (default: 8080).
