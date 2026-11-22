**Component**
An OSCAL-based representation of a Hosting Platform element and the implemented Controls it supports, as documented in the Hosting Platform’s System Security Plan. For example, a Web Application Firewall is a Component that can support implemented Controls from the System and Communications Protection (SC) family.

**Context**
One or more Components within an Environment on a Hosting Platform.

**Control**
The means of managing Risk (e.g., policies, procedures, and guidelines) to a Hosting Platform and the apps on it. A Control can be administrative, technical, or physical in nature. A Tenant inherits a Hosting Platform’s Controls when the Tenant’s app is deployed on the Hosting Platform.

**Environment**
The compute, storage, and networking resources of a Hosting Platform that have a unique set of IP address space, cryptographic material, and Risk Thresholds. Examples include Development, Test, and Production. 

**Finding**
A result reported by a Scanner when it detects a violation of a rule. A Finding can indicate a Vulnerability, an error in how an app performs a task, or a formatting inconsistency that could make it difficult to update an app.

**Hosting Platform**
The compute, storage, and networking resources required to host one or more apps. A Hosting Platform is an Information System. Each Hosting Platform has one System Security Plan.

**Mitigation**
The result when an implemented Control reduces the likelihood or impact of harm from a Threat or Vulnerability.

**Platform Team**
The people responsible for operating a Hosting Platform.

**Residual Risk**
The Risk that remains after a Control has been implemented.

**Risk**
The potential for harm, measured by its likelihood and impact.

**Risk Acceptance**
The decision to accept Risk without taking further action to reduce it.

**Risk Avoidance**
The decision to eliminate Risk by removing its source. For example, a Tenant can avoid the Risk associated with a vulnerable feature by removing the feature.

**Risk Reduction**
The decision to reduce the likelihood or impact of Risk by implementing a Control. For example, a Tenant can reduce the Risk of unauthorized access by requiring multi-factor authentication.

**Risk Transference**
The decision to transfer the impact of Risk to another entity. For example, a Tenant can transfer some of the Risk introduced by database server maintenance to a Cloud Service Provider by using a Platform as a Service (PaaS) offering that includes server patching.

**Scanner**
A tool that applies rules to a Tenant’s app and reports Findings. A Scanner can use Context to determine which Findings a Hosting Platform’s Controls address.

**Security Team**
The people responsible for protecting a Hosting Platform and deciding whether an app may be deployed to it. The Security Team reviews the app’s Body of Evidence to inform that decision.

**Suppression**
An instruction that tells a Scanner to ignore a Finding because an implemented Control already mitigates the associated Risk.

**System Security Plan**
The document that defines a Hosting Platform’s security requirements and describes how the Platform Team meets them.

**Tenant**
The people responsible for a specific app deployed on a Hosting Platform. 

**Threat**
A circumstance or event that can harm a Hosting Platform, the apps deployed on it, or the data they contain. A Threat can cause harm by exploiting a Vulnerability.

**Vulnerability**
A weakness, such as a design flaw, misconfiguration, or exposure, that a Threat can exploit to cause harm to a Hosting Platform, its apps, or their data. A Scanner can report a Finding that identifies a Vulnerability.
