module "recipe_service_lambda" {
  source = "./modules/lambda"

  environment    = var.deploy_account
  function_name  = "${var.namespace}-${var.project_name}-lambda"
  handler_name   = "src.handler.service_handler.lambda_handler"
  lambda_package = "../${path.root}/lambda.zip"
  s3_bucket_id   = aws_s3_bucket.builds.id
  lambda_environment_var = {
    "TABLE_NAME" = local.service_table_name
  }

  lambda_role_arn = aws_iam_role.iam_for_lambda_dynamodb.arn
}
