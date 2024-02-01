variable "function_name" {
  description = "Name of the lambda"
  type        = string
}

variable "environment" {
  type        = string
  description = "Environment we are currently deploying into"
}

variable "handler_name" {
  type        = string
  description = "Name of the lambda handler"
}

variable "timeout" {
  type        = string
  description = "Timeout for the lambda"
  default     = 30
}

variable "memory_size" {
  type        = string
  description = "Memory size for the lambda"
  default     = 1024
}

variable "lambda_package" {
  type        = string
  description = "Path where the lambda package is located"
}

variable "s3_bucket_id" {
  type        = string
  description = "S3 bucket id for storing lambda package"
}

variable "lambda_role_arn" {
  type        = string
  description = "ARN of the IAM role"
}

variable "lambda_environment_var" {
  type        = map(string)
  description = "List of environment variable to give the Lambda function"
}
