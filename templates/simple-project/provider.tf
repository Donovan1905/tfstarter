provider "aws" {
  region = var.region
  default_tags {
    tags = {
      namespace    = var.namespace
      project_type = var.project_type
      project      = var.project_name
    }
  }
}

terraform {
  backend "s3" {
    region = "eu-west-3"
    bucket = "{{state_bucket_name}}"
  }
}
