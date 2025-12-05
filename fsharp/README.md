# F# Project

The aim of this project is to explore F#, the Oxpecker library and OpenAPI type providers.

### Complete ✅

- F# project setup with .NET 10
- Build and run scripts
- Basic webserver with hello world at the root

### Todo 🔮

- Add unit and integration tests
- Add OpenAPI type provider from the rust OpenAPI docs
- Add GraphQL endpoints and/or MCP server

## Setup

1. Ensure you have the [.NET 10 SDK](https://dotnet.microsoft.com/download/dotnet/10.0) installed.

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

## Cross-platform lock files

When updating dependencies, lock files must be created for every platform:

```sh
dotnet restore -p:MyRuntimeIdentifier=linux-x64 --force-evaluate
dotnet restore -p:MyRuntimeIdentifier=win-x64 --force-evaluate
dotnet restore -p:MyRuntimeIdentifier=osx-x64 --force-evaluate
```

(more details here: https://github.com/NuGet/Home/issues/9195#issuecomment-1547896705)
