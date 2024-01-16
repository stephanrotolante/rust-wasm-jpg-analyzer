package main

import (
	"fmt"
	"html/template"
	"net/http"
)

type Cat struct {
	Name     string
	Cuteness int
}

func (cat Cat) Speak() {
	fmt.Println("Meow")
}

type Cats []Cat

type Data struct {
	CatList Cats
}

func BaseHandler(writer http.ResponseWriter, request *http.Request) {

	templateFile := template.Must(template.ParseFiles("html/index.html"))

	cats := Cats{
		Cat{
			Name:     "Indy",
			Cuteness: 100,
		},
		Cat{
			Name:     "GramPam",
			Cuteness: 9000,
		},
	}

	templateFile.Execute(writer, Data{
		CatList: cats,
	})

}
