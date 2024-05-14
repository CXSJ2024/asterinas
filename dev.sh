#!env zsh

#docker run -it --privileged --network=host --device=/dev/kvm -v .:/root/asterinas asterinas/asterinas:custom
docker start elated_bardeen
docker exec -it elated_bardeen /bin/zsh
