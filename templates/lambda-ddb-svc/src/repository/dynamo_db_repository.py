import os

import boto3


class DynamoDbRepository:
    def __init__(
            self
    ):
        dynamodb = boto3.resource('dynamodb')
        self.table = dynamodb.Table(os.environ['TABLE_NAME'])

    def get_all_item(self) -> dict:
        items = self.table.scan(Limit=100)
        return items['Items']

    def get_item(self, id: str) -> dict:
        item = self.table.get_item(
            Key={
                'id': id
            }
        )
        return item

    def put_item(self, item: dict) -> None:
        self.table.put_item(
            Item=item
        )
        return None

    def delete_item(self, id: str) -> None:
        self.table.delete_item(
            Key={
                'id': id
            }
        )
        return None