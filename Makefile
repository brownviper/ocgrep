build-dev:
	docker build -f Dockerfile.dev -t ocgrep-dev .

test: build-dev
	docker run --rm ocgrep-dev

build:
	docker build -t ocgrep .

# Runs the CLI tool against your local files
# tell Docker to pretend it's a real terminal by passing the -t (tty) and -i (interactive) flags
# Usage: make run ARGS="-n duct some_file.txt"
run: build
	docker run --rm -it -v "$(PWD):/workspace" ocgrep $(ARGS)

# Cleans up the built Docker images
clean:
	docker rmi ocgrep-dev ocgrep || true

release:
	cargo build --release
	
wipe:
	docker stop $$(docker ps -q) 2>/dev/null || true
	docker container prune
	docker image prune -a -f
	docker volume prune -a -f
	docker builder prune
	docker system df
