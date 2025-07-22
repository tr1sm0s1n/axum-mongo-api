CMD := 'cargo'
DOCKER := 'docker'

# Display recipes.
default:
    @just --list

# Install Rust using rustup.
install:
    @curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Run the application.
run:
    @{{ CMD }} run

# Format the Rust files.
fmt:
	@{{ CMD }} fmt

# Start the containers.
up:
	@{{ DOCKER }} compose up --build -d

# Stop the containers.
down:
	@{{ DOCKER }} compose down

# Enter the database.
enter:
	@{{ DOCKER }} exec -it axum-mongo mongosh -u root -p rootpw
