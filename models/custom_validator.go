package models

import (
	"github.com/go-playground/validator/v10"
	"github.com/labstack/echo/v5"
)

type CustomValidator struct {
	Validator *validator.Validate
}

func (cv *CustomValidator) Validate(i any) error {
	var err = cv.Validator.Struct(i)
	if err != nil {
		return echo.ErrBadRequest.Wrap(err)
	}
	return nil
}
