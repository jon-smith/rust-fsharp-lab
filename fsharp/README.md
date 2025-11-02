# F# Project

The aim of this project is to explore F#, the Oxpecker library and OpenAPI type providers.

### Complete ✅

- F# project setup with .NET 9
- Build and run scripts
- Basic webserver with hello world at the root

### Todo 🔮

- Implement web server
- Add unit and integration tests
- Add OpenAPI type provider from the rust OpenAPI docs
- Add GraphQL endpoints and/or MCP server

## Setup

1. Ensure you have the [.NET 9 SDK](https://dotnet.microsoft.com/download/dotnet/9.0) installed.

2. Restore project dependencies and tools:

```sh
dotnet restore
dotnet tool restore
```

## Build/Run

```sh
dotnet build
dotnet fantomas . # format
dotnet fsharplint lint . # run linter
dotnet run
```

Now you can navigate to [localhost:5000](localhost:5000) to see the message from the server
