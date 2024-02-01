from typing import Optional

from src.service.service import {{service_name}}Service


def lambda_handler(event, _context):
    return {{service_name}}ServiceHandler().handle(event, _context)


class {{service_name}}ServiceHandler:

    def __init__(
            self,
            {{service_name}}_service: Optional[{{service_name}}Service] = None
    ):
        self.{{service_name}}_service = {{service_name}}_service or {{service_name}}Service()

    def handle(self, event, context):
        match event["context"]["http-method"]:
            case "GET":
                if "id" in event:
                    return self.get_{{service_name}}_handler(event)
                else:
                    return self.get_all_{{service_name}}_handler()
            case "POST":
                return self.new_{{service_name}}_handler(event)
            case "PUT":
                return self.update_{{service_name}}_handler(event)
            case "DELETE":
                return self.delete_{{service_name}}_handler(event)

    def get_all_{{service_name}}_handler(self):
        resp = self.{{service_name}}_service.get_all_{{service_name}}()

        return {
            'statusCode': 200,
            'headers': {
                'Content-Type': 'application/json',
                'Access-Control-Allow-Origin': [
                    '*'
                ]
            },
            'body': resp,
            "isBase64Encoded": False
        }

    def get_{{service_name}}_handler(self, event):
        resp = self.{{service_name}}_service.get_{{service_name}}(event["id"])

        return {
            'statusCode': 200,
            'headers': {
                'Content-Type': 'application/json',
                'Access-Control-Allow-Origin': [
                    '*'
                ]
            },
            'body': resp,
            "isBase64Encoded": False
        }

    def new_{{service_name}}_handler(self, event):
        expected_payload = [
            "name"
        ]
        check_parameters(expected_payload, event)
        resp = self.{{service_name}}_service.save_{{service_name}}(event["name"])

        return {
            'statusCode': 200,
            'headers': {
                'Content-Type': 'application/json',
                'Access-Control-Allow-Origin': [
                    '*'
                ]
            },
            'body': resp,
            "isBase64Encoded": False
        }

    def update_{{service_name}}_handler(self, event):
        expected_payload = [
            "id",
            "name"
        ]
        check_parameters(expected_payload, event)
        resp = self.{{service_name}}_service.update_{{service_name}}(event["id"],
                                                 event["name"])

        return {
            'statusCode': 200,
            'headers': {
                'Content-Type': 'application/json',
                'Access-Control-Allow-Origin': [
                    '*'
                ]
            },
            'body': resp,
            "isBase64Encoded": False
        }

    def delete_{{service_name}}_handler(self, event):
        expected_payload = [
            "id"
        ]
        check_parameters(expected_payload, event)
        resp = self.{{service_name}}_service.delete_{{service_name}}(event["id"])

        return {
            'statusCode': 200,
            'headers': {
                'Content-Type': 'application/json',
                'Access-Control-Allow-Origin': [
                    '*'
                ]
            },
            'body': resp,
            "isBase64Encoded": False
        }


def check_parameters(expected_parameters: list, event: dict):
    for key in expected_parameters:
        if not (key in event):
            message = {"message": f"Bad Request: missing {key} key in event payload"}
            print(f"Key {key} is missing")
            return {
                'statusCode': 400,
                'headers': {
                    'Content-Type': 'application/json',
                    'Access-Control-Allow-Origin': [
                        '*'
                    ]
                },
                'body': message,
                "isBase64Encoded": False
            }
