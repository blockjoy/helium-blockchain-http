IMAGE_NAME=helium-blockchain-http

docker-build:
	@docker build . -t ${IMAGE_NAME}