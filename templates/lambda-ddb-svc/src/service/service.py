import os
from typing import Optional
import uuid

from src.repository.dynamo_db_repository import DynamoDbRepository


class {{service_name}}Service:

    def __init__(self, dynamodb_repository: Optional[DynamoDbRepository] = None):
        self.dynamodb_repository = dynamodb_repository or DynamoDbRepository()

    def get_all_{{service_name}}(self):
        return self.dynamodb_repository.get_all_item()

    def get_{{service_name}}(self, id: str):
        return self.dynamodb_repository.get_item(id)

    def save_{{service_name}}(self, name: str):
        self.dynamodb_repository.put_item(
            item={
                'id': str(uuid.uuid1()),
                'name': name
            }
        )

    def update_{{service_name}}(self, id: str, name: str):
        self.dynamodb_repository.put_item(
            item={
                'id': id,
                'name': name
            }
        )

    def delete_{{service_name}}(self, id: str):
        self.dynamodb_repository.delete_item(id=id)