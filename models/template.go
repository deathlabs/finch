package models

type MappingTemplate struct {
	Name     string   `yaml:"name"`
	Version  string   `yaml:"version"`
	Controls []string `yaml:"controls"`
}
