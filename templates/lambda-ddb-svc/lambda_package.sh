mkdir lambda_package
source venv/bin/activate
python3 -m pip install --platform manylinux2014_x86_64 --implementation cp --only-binary=:all: --upgrade --target venv/lib/python3.11/site-packages/ -r requirements.txt
cp -r venv/lib/python3.11/site-packages/* lambda_package/
cp -r src lambda_package/
cd lambda_package && zip -r ../lambda.zip * && cd ../