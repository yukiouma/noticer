.PHONY: build
build:
	docker build --network=host -t noticer/$(module):v0.1.0 -f Dockerfile.$(module) .

.PHONY: remove
remove:
	docker rmi noticer/$(module):v0.1.0
