#
# docker factory
#
.PHONY = build run clean

build: clean
	@git clone https://github.com/Software-Knife-and-Tool/.git
	@docker build -f Dockerfile.build .
	@docker build -t ubuntu-mu -f Dockerfile.ubuntu .
	@rm -rf mu

run:
	docker run -it ubuntu-mu /opt/mu/bin/mu

clean:
	@rm -rf mu
