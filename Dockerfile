# Use the official Rust image as the base image
FROM rust:1.72.1

# Set the working directory to /src
WORKDIR /usr/src/app

# Copy the Rust application code to the container
COPY . .

# Compile the Rust application
RUN cargo install --path .

# Set the entry point to run the compiled application
CMD ["rustapp"]
