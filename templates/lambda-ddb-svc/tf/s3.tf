resource "aws_s3_bucket" "builds" {
  bucket = "${var.namespace}-lambdas-package-${var.deploy_account}"
}