#TODO Switch to bash
set shell := ["pwsh.exe", "-C"]

name := "mcimg"
version := "1.0.0"

zip := name + "-v" + version + ".zip"

build:
  echo "Building {{name}} version {{version}}"

  # Clean ./dist directory, create empty ./dist/temp directory
  if (Test-Path dist/temp) { rm -Recurse -Force dist/temp }
  if (Test-Path dist/{{zip}}) { rm -Force dist/{{zip}} }
  mkdir dist/temp >> nul;
	
  # Build rust package as release
  cargo build --release;

  # Move package binary, copy assets
  move target/release/mcimg.exe dist/temp/{{name}}.exe;
  copy textures dist/temp/textures;

  # Zip package, delete ./dist/temp
  Compress-Archive -LiteralPath dist/temp/mcimg.exe, dist/temp/textures -DestinationPath dist/{{zip}}
  rm -Recurse -Force dist/temp

  echo "Complete build of {{zip}}"
