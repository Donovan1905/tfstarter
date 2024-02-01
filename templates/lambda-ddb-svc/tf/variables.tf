# Common
variable "region" {
  default = "{{region}}"
}
variable "project_name" {
  default = "{{service_name}}-service"
}

variable "project_type" {
  default = "backend"
}
variable "namespace" {
  default = "{{namespace}}"
}
variable "deploy_account" {}
variable "aws_deploy_account_number" {}
