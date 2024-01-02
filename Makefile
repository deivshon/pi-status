BACK_DIR="./back"
FRONT_DIR="./front/pi-status-front/"

default:
	make -C $(BACK_DIR) default
	make -C $(FRONT_DIR)

arm64:
	make -C $(BACK_DIR) arm64
	make -C $(FRONT_DIR)

docker-arm64:
	docker buildx build --platform linux/arm64 -t pi-status . --build-arg PLATFORM=arm64 --build-arg CARGO_TARGET=aarch64-unknown-linux-gnu

docker-armv7:
	docker buildx build --platform linux/arm/v7 -t pi-status . --build-arg PLATFORM=linux/arm/v7 --build-arg CARGO_TARGET=armv7-unknown-linux-gnueabihf

docker-amd64:
	docker buildx build --platform linux/amd64 -t pi-status . --build-arg PLATFORM=x86_64 --build-arg CARGO_TARGET=x86_64-unknown-linux-gnu

clean:
	make -C $(BACK_DIR) clean
	make -C $(FRONT_DIR) clean
