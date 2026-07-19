def handler(event, context):
    message = "Called first_func_2"
    print(message)
    return {"statusCode": 200, "body": message}
