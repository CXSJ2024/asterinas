#!env zsh

#docker run -it --privileged --network=host --device=/dev/kvm -v .:/root/asterinas asterinas/asterinas:custom
docker start romantic_galois
docker exec -it romantic_galois /bin/zsh
