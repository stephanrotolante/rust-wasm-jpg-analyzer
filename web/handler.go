package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
	"stephanrotolante/rust-wasm-jpg-analyzer/web/views"
)

func BaseHandler(w http.ResponseWriter, r *http.Request) {

	fileUploadScript, err := os.ReadFile("scripts/fileUpload.js")
	if err != nil {
		log.Fatalf("unable to read file: %v", err)
		views.Error().Render(r.Context(), w)
		return
	}

	wasmTestScript, err := os.ReadFile("scripts/wasmTest.js")
	if err != nil {
		log.Fatalf("unable to read file: %v", err)
		views.Error().Render(r.Context(), w)
		return
	}

	views.Index([]string{
		fmt.Sprintf("<script type=\"text/javascript\">%s</script>", string(fileUploadScript)),
		fmt.Sprintf("<script type=\"text/javascript\">%s</script>", string(wasmTestScript)),
	}).Render(r.Context(), w)
}
