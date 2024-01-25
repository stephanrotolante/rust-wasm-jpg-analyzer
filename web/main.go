package main

import (
	"fmt"
	"log"
	"net/http"
)

func init() {
	fmt.Println("Init...")
}

func main() {
	fmt.Println("Invoking main...")

	staticDir := "/Users/srotolante/Projects/rust-wasm-jpg-analyzer/wasm/pkg/"

	// Create a file server handler for the specified directory
	fs := http.FileServer(http.Dir(staticDir))

	// Handle requests by serving static files
	http.Handle("/", fs)

	http.HandleFunc("/home", BaseHandler)

	log.Fatal(http.ListenAndServe(":4005", nil))
}
