scp Cargo.toml ralle@161.97.67.44:~/chatserver/_Cargo.toml
scp -r src ralle@161.97.67.44:~/chatserver/_src
ssh ralle@$contabo -t "./deploy_chatserver.sh"
