import aws_cdk

from stacks.first.infrastructure import FirstStack
from stacks.second.infrastructure import SecondStack
from stacks.third.infrastructure import ThirdStack

app = aws_cdk.App()

environment = aws_cdk.Environment(account="000000000000", region="us-east-1")

_ = FirstStack(scope=app, construct_id="MyStacks-First", env=environment)
_ = SecondStack(scope=app, construct_id="MyStacks-Second", env=environment)
_ = ThirdStack(scope=app, construct_id="MyStacks-Third", env=environment)

_ = app.synth()
