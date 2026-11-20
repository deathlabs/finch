package models

type ComponentDefinition struct {
	UUID                                  string      `yaml:"uuid"`
	Title                                 string      `yaml:"title"`
	Type                                  string      `yaml:"type"`
	Description                           string      `yaml:"description,omitempty"`
	Purpose                               string      `yaml:"purpose"`
	ControlsImplementedByThisComponent    []string    `yaml:"controls_implemented_by_this_component"`
	SecurityRulesMitigatedByThisComponent []ToolRules `yaml:"security_rules_mitigated_by_this_component"`
}
