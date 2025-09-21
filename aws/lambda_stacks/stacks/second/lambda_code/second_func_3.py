def handler(event, context):
    message = "Called second_func_3"
    print(message)
    return {"statusCode": 200, "body": message}
