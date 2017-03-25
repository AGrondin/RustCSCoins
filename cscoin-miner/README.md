# Docker

### Creating our Docker image
- remove your `target/` directory and any gzipped archives
```
cd cscoin-miner
rm -rf target/
rm cscoins-concordia.tar.gz
```
- pack our source code
```
tar -czvf cscoins-concordia.tar.gz .
```
- source our `env.rc` (:warning: Make sure the cert path is pointing to our repository :warning:)
```
source config/env.rc
```
- Create our Docker image
```
docker --config config build -t "cscoins-concordia-miner-test" .
```




