# HotDog 🌭

A fun Dioxus application for browsing random dog images from the Dog CEO API. Built with Rust and styled with Tailwind CSS.

## Project Structure

```text
hot_dog/
├─ assets/             # Static assets (CSS, images, etc.)
├─ src/
│  ├─ main.rs          # Application entry point
│  ├─ components/      # UI components
│  │  ├─ mod.rs        # Component module exports
│  │  ├─ app.rs        # Main App component
│  │  ├─ title.rs      # Title component and state
│  │  └─ dog_view.rs   # Dog image viewer component
├─ Cargo.toml          # Project dependencies and configuration
├─ Dioxus.toml         # Dioxus configuration
└─ tailwind.css        # Tailwind CSS configuration
```

## Features

- 🐕 Fetches random dog images from [Dog CEO API](https://dog.ceo/api/breeds/image/random)
- 🎨 Styled with Tailwind CSS
- ⚡ Built with Dioxus for fast, reactive UI
- 📦 Modular component architecture

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

Start the development server with the default platform:

```bash
dx serve
```

To run on a specific platform:

```bash
dx serve --platform desktop  # Desktop app
dx serve --platform web       # Web app
```

## Component Overview

### App (`components/app.rs`)

The main application component that sets up the context provider and renders the Title and DogView components.

### Title (`components/title.rs`)

Displays the application title with emoji. Uses context to access the title state.

### DogView (`components/dog_view.rs`)

The main feature component that:

- Displays a dog image
- Provides "skip" and "save!" buttons
- Fetches new random dog images from the Dog CEO API

## Building for Production

Build optimized binaries for your target platform:

```bash
dx build --release
```
