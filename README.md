# React Cli

A cli to create React Components and Hooks.

Instructions:

- Download the binary.
- To Create A React Component

```bash
    ./react -n <Component Name>
```

- To Create React Hook

```bash
    ./react -n <hook name> -H
```

#### Available Flags

```
-h, --help           Print help information
-H  --hook           Create a hook file
-m, --module         Create a css module instead of global css
-n, --name <NAME>    The name of the component you want to create
    --ts             Same as --typescript
    --typescript     Create a typescript component
-V, --version        Print version information
```

## End Notes

Thanks to [0xBirdie](https://github.com/itsmebirdie) for testing and releasing binaries for Windows and Mac machines.

Thanks to [Markos-TH09](https://github.com/markos-th09) for compiling binaries for different architecures,refactor the code and setup github actions.