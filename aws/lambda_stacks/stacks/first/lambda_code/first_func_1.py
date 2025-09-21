def handler(event, context):
    message = "Called first_func_1"
    print(message)
    return {"statusCode": 200, "body": message}
