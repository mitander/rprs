.PHONY: run-servers
run-servers:
	docker run --rm -d -p 9001:80 --name server1 kennethreitz/httpbin
	docker run --rm -d -p 9002:80 --name server2 kennethreitz/httpbin
	docker run --rm -d -p 9003:80 --name server3 kennethreitz/httpbin

.PHONY: run-proxy
run-proxy:
	cargo run src/main.rs

.PHONY: stop-servers
stop-servers:
	docker stop server1
	docker stop server2
	docker stop server3
