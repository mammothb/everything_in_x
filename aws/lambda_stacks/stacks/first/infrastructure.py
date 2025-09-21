from typing import Any

from aws_cdk import Stack, aws_lambda
from constructs import Construct


class FirstStack(Stack):
    def __init__(self, scope: Construct, construct_id: str, **kwargs: Any) -> None:
        super().__init__(scope, construct_id, **kwargs)

        _ = aws_lambda.Function(
            scope=self,
            id="first_stack-lambda_1",
            runtime=aws_lambda.Runtime.PYTHON_3_11,
            handler="first_func_1.handler",
            code=aws_lambda.Code.from_asset("stacks/first/lambda_code"),
        )

        _ = aws_lambda.Function(
            scope=self,
            id="first_stack-lambda_2",
            runtime=aws_lambda.Runtime.PYTHON_3_11,
            handler="first_func_2.handler",
            code=aws_lambda.Code.from_asset("stacks/first/lambda_code"),
        )
