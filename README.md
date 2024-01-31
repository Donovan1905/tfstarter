# ⚙️ tfstarter - Bootstrap Terraform projects based on multiple templates 🚀

*Powered by Rust*

The tool is currently on development, if you think that a feature/template needs to be added, do not hesitate to create an issue.

## Install

Published [here](https://crates.io/crates/tfstarter)


```
cargo install tfstarter
```

## Usage

### Add custom templates

You can add manually your own Terraform projects in `~/.tfstarter/` (note that the directory name that you choose will define the key to use when using this template).

If you want to add placeholder to your templates use the following format : 
```
{{placeholder_name}}
```
It will be automatically recognized, and you will be prompted for the value when using the template.


### List the templates

Use this command to list all the available templates : 

```
tfstarter get
```

### Use a template 

Go to the target directory of your project and generate the terraform project by using :

This command will simply copy the files of the template directory to your current directory. You will be prompted for the value of each tag present in the template files.

```
tfstarter new -t <template_name>
```

### Update the default templates list

To update the default templates list that is provided remotely, use :

```
tfstarter update
```