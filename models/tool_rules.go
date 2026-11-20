package models

type ToolRules struct {
	Tool  string   `yaml:"tool"`
	Rules []string `yaml:"rules"`
}
