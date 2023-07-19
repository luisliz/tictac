    # Use an official Dart runtime as a parent image
    FROM google/dart

    # Set the working directory in the container to /app
    WORKDIR /app

    # Copy the current directory contents into the container at /app
    COPY . /app

    # Install any needed packages specified in pubspec.yaml
    RUN pub get

    # Make port 8080 available to the world outside this container
    EXPOSE 8080

    # Run the binary program produced by `dart compile exe`
    CMD ["./your_binary_name"]
