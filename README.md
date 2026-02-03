# JSON Log Parser

A CLI tool for parsing and formatting Docker Compose log streams with colored output.

## Example

Input:
```
web-1 | {"level":"info","message":"Server started","timestamp":"2024-01-30T10:00:00Z"}
```

Output (with colors):
```
[web-1] 2024-01-30T10:00:00Z INFO: Server started
```

## Use as a K9S plugin

### Automatic Install:
```
curl -fsSL https://github.com/yairvogel/json-log-parser/raw/refs/heads/master/install.sh | bash
```

- download a binary from the releases or build from source. Place your binary in your PATH.
- download kubectl-parse script, make it executable and in your PATH as well
- run `(read -r _ PLUGIN_PATH < <(k9s info | grep Plugins); echo $PLUGIN_PATH)`
- copy the contents of k9s_plugin_def.yaml in the file you got from the previous command
- on a deployment or a pod view, press <shift-h> to view formatted logs

