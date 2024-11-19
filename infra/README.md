Notes:

- cannot use `aws sam` because the API gateway must be defined in the same template as the lambda function
- `cargo lambda` doesn't seem to cleanup after itself and it uses `aws sam` under the hood

Steps:

1. createa s3 bucket
3. upload lambda code to s3
2. deploy cloudformation
