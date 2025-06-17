# A toy `docker` command

The example `bash` script `docker.sh` reads the app spec from `spec.toml`.

Note that the script runs `claptrap` in a container, so you need to have Docker installed and running.

## Usage examples

Show help:

```shell
./docker.sh --help
A self-sufficient runtime for containers

Usage: docker [COMMAND]

Commands:
  run   Create and run a new container from an image
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Show help for the `run` subcommand:

```shell
./docker.sh run --help
Create and run a new container from an image

Usage: run [OPTIONS] <IMAGE>

Arguments:
  <IMAGE>  The image to run

Options:
  -i, --interactive    Keep STDIN open even if not attached
  -t, --tty            Allocate a pseudo-TTY
  -v, --volume <list>  Bind mount a volume
  -h, --help           Print help
```

Run a container interactively with a volumes mounted:

```shell
./docker.sh run -it -v ${PWD}:/data -v /root:/root fujiapple/claptrap:latest
claptrap_run_image: fujiapple/claptrap:latest
claptrap_run_interactive: true
claptrap_run_tty: true
claptrap_run_volume: (/home/fuji:/data /root:/root)
```
