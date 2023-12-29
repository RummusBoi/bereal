scp api.py ralle@161.97.67.44:~/api2.py
scp -r helper_functions ralle@161.97.67.44:~/helper_functions2
ssh ralle@$contabo -t "./deploy_api2.sh"


