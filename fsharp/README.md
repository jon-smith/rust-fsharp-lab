# F# Project

The aim of this project is to explore F# features and .NET integration.

### Complete ✅

- F# project setup with .NET 9
- Build and run scripts

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
