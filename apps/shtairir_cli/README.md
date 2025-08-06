# Shtairir CLI

Command-line interface for the Shtairir unified scripting language.

## Installation

```bash
cargo install --path apps/shtairir_cli
```

## Usage

### Execute a script file

```bash
shtairir --script examples/hello_world.sht
```

### Execute an inline script

```bash
shtairir --execute 'bevy:create_entity()'
```

### Output formats

```bash
# Text format (default)
shtairir --script examples/hello_world.sht --format text

# JSON format
shtairir --script examples/hello_world.sht --format json
```

## Examples

### Hello World

Create a script file `hello.sht`:
```
bevy:create_entity()
redis:set("greeting", "Hello, World!")
```

Execute it:
```bash
shtairir --script hello.sht
```

### Complex Workflow

Create a script file `workflow.sht`:
```
// Create a 3D scene
bevy:create_entity()
bevy:add_component("camera", "Camera")
bevy:add_component("light", "Light")

// Process media
ffmpeg:convert("input.mp4", "output.webm")
ffmpeg:extract_audio("input.mp4", "audio.opus")

// Store metadata
redis:set("video:input", "input.mp4")
redis:set("video:output", "output.webm")
```

Execute it:
```bash
shtairir --script workflow.sht
```

## Options

```
-s, --script <SCRIPT>      Script file to execute
-e, --execute <EXECUTE>    Inline script to execute
-f, --format <FORMAT>      Output format (json, text) [default: text]
-h, --help                 Print help
-V, --version              Print version
```

## License

This project is licensed under the CPC License - see the LICENSE file for details.