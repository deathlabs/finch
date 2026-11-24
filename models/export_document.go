package models

// ExportMetadata records which System Security Plan a mappings file was created from.
type ExportMetadata struct {
	SystemSecurityPlanUUID         string `yaml:"system_security_plan_uuid,omitempty" json:"system_security_plan_uuid,omitempty"`
	SystemSecurityPlanTitle        string `yaml:"system_security_plan_title,omitempty" json:"system_security_plan_title,omitempty"`
	SystemSecurityPlanVersion      string `yaml:"system_security_plan_version,omitempty" json:"system_security_plan_version,omitempty"`
	SystemSecurityPlanLastModified string `yaml:"system_security_plan_last_modified,omitempty" json:"system_security_plan_last_modified,omitempty"`
	GeneratedAt                    string `yaml:"generated_at,omitempty" json:"generated_at,omitempty"`
}

type ExportDocument struct {
	Metadata   *ExportMetadata       `yaml:"metadata,omitempty"`
	Components []ComponentDefinition `yaml:"components"`
}
