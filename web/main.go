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

	// Handle requests by serving static files
	// http.HandleFunc("/wasm", func(w http.ResponseWriter, r *http.Request) {
	// 	fileBytes, err := os.ReadFile("/Users/srotolante/Projects/rust-wasm-jpg-analyzer/wasm/pkg/rust_wasm_jpg_analyzer_bg.wasm")
	// 	if err != nil {
	// 		panic(err)
	// 	}
	// 	w.WriteHeader(http.StatusOK)
	// 	w.Header().Set("Content-Type", "application/octet-stream")
	// 	w.Write(fileBytes)
	// 	return
	// })

	http.HandleFunc("/home", BaseHandler)

	log.Fatal(http.ListenAndServe(":4005", nil))
}
