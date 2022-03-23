### React Cli

A cli to generate React Components.

## Instructions

Download the Binary

```bash
    ./react -n <Component Name> 
```
This will create a folder with Component name with css as default stylesheet.

To Change this behaviour you can use tags like
`-t`,`-c`,`-s`

- `-t` tag will create a Typescript Component
- `-c` will create a stylesheet module.
- `-s <name>` This will create stylesheet of defined name

For Example:

```
./react -n Header -t -c -s sass
```
This will create Header component with Sass modules
