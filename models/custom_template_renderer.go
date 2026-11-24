package models

import (
	"html/template"
	"io"

	"github.com/labstack/echo/v5"
)

type CustomTemplateRenderer struct {
	Templates *template.Template
}

func (ctr *CustomTemplateRenderer) Render(context *echo.Context, w io.Writer, name string, data any) error {
	return ctr.Templates.ExecuteTemplate(w, name, data)
}
