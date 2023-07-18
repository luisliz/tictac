# Assuming you have Rust and Cargo installed
# Install diesel_cli for database operations
cargo install diesel_cli --no-default-features --features postgres

# Install Node.js and npm for frontend dependencies
# Assuming you have Node.js and npm installed
# Install Tauri CLI for building the desktop app
npm install -g @tauri-apps/cli

# Install Yew for the frontend
cargo install cargo-web

# Install frontend dependencies
npm install

# Run migrations
diesel migration run

# Start the backend
cargo run

# In a new terminal window, start the frontend
npm run dev

# In a new terminal window, start the Tauri app
tauri dev
