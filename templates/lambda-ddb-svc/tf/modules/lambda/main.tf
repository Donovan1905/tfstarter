module "lambda" {
  source  = "terraform-aws-modules/lambda/aws"
  version = "5.2.0"

  function_name         = var.function_name
  runtime               = "python3.11"
  handler               = var.handler_name
  lambda_role           = var.lambda_role_arn
  create_role           = false
  environment_variables = var.lambda_environment_var

  memory_size = 1028
  timeout     = var.timeout

  create_package = false
  s3_existing_package = {
    bucket = var.s3_bucket_id
    key    = aws_s3_object.lambda_package.id
  }
}

resource "aws_s3_object" "lambda_package" {
  bucket = var.s3_bucket_id
  key    = "${filemd5(var.lambda_package)}.zip"
  source = var.lambda_package
}
