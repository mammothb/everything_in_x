def handler(event, context):
    message = "Called second_func_2"
    print(message)
    return {"statusCode": 200, "body": message}
