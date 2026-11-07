defmodule GetPassword.MixProject do
  use Mix.Project

  def project do
    [
      app: :get_password,
      version: "0.9.1",
      elixir: "~> 1.19",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      # Hex Packaging
      package: package(),
      # Docs
      name: "GetPassword",
      source_url: "https://github.com/aggressive-labs/get_password",
      homepage_url: "https://github.com/aggressive-labs/get_password",
      docs: &docs/0
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      # Dependencies
      {:rustler, "~> 0.38.0", runtime: false},

      # Dev Dependencies
      {:ex_doc, "~> 0.40.1", only: :dev, runtime: false, warn_if_outdated: true}
    ]
  end

  defp package() do
    [
      description:
        "Provides functionality to actually, securely read a password from within `IEx` or a `Mix` task.",
      files: [
        "lib",
        "priv",
        ".formatter.exs",
        "mix.exs",
        "README*",
        "LICENSE*",
        "native",
        "Cargo.*"
      ],
      exclude_patterns: ["*.so"],
      licenses: ["Apache-2.0"],
      links: %{
        "GitHub" => "https://github.com/aggressive-labs/get_password"
      }
    ]
  end

  defp docs do
    [
      main: "GetPassword",
      logo: "assets/logo.png",
      extras: ["README.md"]
    ]
  end
end
