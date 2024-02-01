resource "aws_dynamodb_table" "service_table" {
  name             = local.service_table_name
  hash_key         = "id"
  billing_mode     = "PAY_PER_REQUEST"

  attribute {
    name = "id"
    type = "S"
  }

  deletion_protection_enabled = true
}