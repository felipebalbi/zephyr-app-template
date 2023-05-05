# Zephyr Application Template

## Overview

A simple template with `devcontainers` support mimicking the standard
"Hello World" sample.

## Building

Upon cloning the repository, open the folder in VS Code, install Dev
Containers extension. Once the extension is installed, open a terminal
and initialize the workspace:

```sh
$ west init
$ west update
```

Finally, build the application:

```sh
$ west build -b qemu_cortex_m3
```
