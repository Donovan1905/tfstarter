# Common
variable "region" {
  default = "eu-west-3"
}
variable "project_name" {
  default = "{{project_name}}"
}

variable "project_type" {
  default = "{{project_type}}"
}
variable "namespace" {
  default = "{{namespace}}"
}
variable "deploy_account" {}
variable "aws_deploy_account_number" {}
