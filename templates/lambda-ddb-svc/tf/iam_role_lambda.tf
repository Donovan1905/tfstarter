data "aws_caller_identity" "current" {}

data "aws_iam_policy_document" "assume_role_policy_lambda" {
  statement {
    sid    = ""
    effect = "Allow"
    principals {
      identifiers = ["lambda.amazonaws.com"]
      type        = "Service"
    }
    actions = ["sts:AssumeRole"]
  }
}

data "aws_iam_policy_document" "policy_dynamodb" {
  statement {
    sid    = "DynamoDBReadWritePolicy"
    effect = "Allow"
    actions = [
        "dynamodb:GetItem",
        "dynamodb:Scan",
        "dynamodb:Query",
        "dynamodb:PutItem",
        "dynamodb:UpdateItem",
        "dynamodb:DeleteItem",
        "dynamodb:BatchWriteItem",
        "dynamodb:BatchGetItem"
    ]
    resources = [
      aws_dynamodb_table.service_table.arn
    ]
  }

  statement {
    sid    = "LambdaLoggingPolicy"
    effect = "Allow"
    actions = [
      "logs:CreateLogStream",
      "logs:PutLogEvents"
    ]
    resources = [
      "arn:aws:logs:*:*:*"
    ]
  }
}

resource "aws_iam_role" "iam_for_lambda_dynamodb" {
  name               = "${var.namespace}-${var.project_name}-lambda-dynamodb"
  assume_role_policy = data.aws_iam_policy_document.assume_role_policy_lambda.json
}

resource "aws_iam_role_policy" "lambda_policy_dynamodb" {
  policy = data.aws_iam_policy_document.policy_dynamodb.json
  role   = aws_iam_role.iam_for_lambda_dynamodb.id
}
