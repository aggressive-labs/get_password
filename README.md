# GetPassword

[![CI](https://github.com/aggressive-labs/get_password/actions/workflows/ci.yml/badge.svg)](https://github.com/aggressive-labs/get_password/actions/workflows/ci.yml) [![Hex.pm](https://img.shields.io/hexpm/v/get_password.svg)](https://hex.pm/packages/get_password) [![HexDocs](https://img.shields.io/badge/docs-hexdocs-blue.svg)](https://hexdocs.pm/get_password)

Provides functionality to actually, securely read a password from within `IEx`
or a `Mix` task.

## Usage

```elixir
# Get password with default prompt, returning a result-tuple
{:ok, pw} = GetPassword.get_password()

# Get password with a custom prompt, returning a result-tuple
{:ok, pw} = GetPassword.get_password("Password: ")

# Get password with default prompt, returning a password or raising an error
pw = GetPassword.get_password!()

# Get password with a custom prompt, returning a password or raising an error
pw = GetPassword.get_password!("Password: ")
```

## Installation

The package can be installed by adding `get_password` to your list of
dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:get_password, "~> 0.8.0"}
  ]
end
```

The docs can be found at <https://hexdocs.pm/get_password>.

