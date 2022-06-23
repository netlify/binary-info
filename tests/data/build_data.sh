#!/bin/sh

# GO
# Linux
GOOS="linux" GOARCH="386" go build -ldflags "-s -w" -o ./linux/go-x86 ./src/go/hello-world.go
GOOS="linux" GOARCH="amd64" go build -ldflags "-s -w" -o ./linux/go-amd64 ./src/go/hello-world.go
GOOS="linux" GOARCH="arm" go build -ldflags "-s -w" -o ./linux/go-arm ./src/go/hello-world.go
GOOS="linux" GOARCH="arm64" go build -ldflags "-s -w" -o ./linux/go-arm64 ./src/go/hello-world.go
# Darwin
GOOS="darwin" GOARCH="amd64" go build -ldflags "-s -w" -o ./darwin/go-amd64 ./src/go/hello-world.go
GOOS="darwin" GOARCH="arm64" go build -ldflags "-s -w" -o ./darwin/go-arm64 ./src/go/hello-world.go
# Windows
GOOS="windows" GOARCH="amd64" go build -ldflags "-s -w" -o ./windows/go-amd64.exe ./src/go/hello-world.go
GOOS="windows" GOARCH="386" go build -ldflags "-s -w" -o ./windows/go-x86.exe ./src/go/hello-world.go


# For rust there is more setup needed, install a linker for the platform etc,
# so we are not doing it in this script. could use docker in theory
