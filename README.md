# tfstarter

CLI Tools to start terraform project from templates - Powered by Rust

The tool is currently on development, if you think that a feature needs to be added, do no hesitate to create an issue.

## Install

Published [here](https://crates.io/crates/tfstarter)


```
cargo install tfstarter
```

## Usage

### Add a templates

Pre configured templates are WIP. For now you can add manualy your Terraform project in `~/.tfstarter/` (note that the directory name that you choose will define the key to use when using this template).

### List the templates

Use this command to list all the available templates : 

```
tfstarter get
```

### Use a template 

Go to the target directory of your project and generate the terraform project by using :

```
tfstarter generate -t <template_name>
```

The project files are copied to the current directory.