def handler(event, context):
    message = "Called second_func_1"
    print(message)
    return {"statusCode": 200, "body": message}
