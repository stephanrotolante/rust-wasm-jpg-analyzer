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

	http.HandleFunc("/", BaseHandler)

	log.Fatal(http.ListenAndServe(":4005", nil))
}
