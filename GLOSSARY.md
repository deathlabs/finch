**Component**  
An OSCAL-based representation of a [hosting platform](GLOSSARY.md) element and the implemented [controls](GLOSSARY.md) it supports, as documented in the [hosting platform’s](GLOSSARY.md) [system security plan](GLOSSARY.md). For example, a Web Application Firewall is a [component](GLOSSARY.md) that can support implemented [controls](GLOSSARY.md) from the System and Communications Protection (SC) family.

**Context**  
One or more [components](GLOSSARY.md) within an [environment](GLOSSARY.md) on a [hosting platform](GLOSSARY.md).

**Control**  
The means of managing [risk](GLOSSARY.md) (e.g., policies, procedures, and guidelines) to a [hosting platform](GLOSSARY.md) and the apps on it. A [control](GLOSSARY.md) can be administrative, technical, or physical in nature. A [tenant](GLOSSARY.md) inherits a [hosting platform’s](GLOSSARY.md) [controls](GLOSSARY.md) when the [tenant’s](GLOSSARY.md) app is deployed on the [hosting platform](GLOSSARY.md).

**Environment**  
The compute, storage, and networking resources of a [hosting platform](GLOSSARY.md) that have a unique set of IP address space, cryptographic material, and [risk](GLOSSARY.md) thresholds. Examples include Development, Test, and Production.

**Finding**  
A result reported by a [scanner](GLOSSARY.md) when it detects a violation of a rule. A [finding](GLOSSARY.md) can indicate a [vulnerability](GLOSSARY.md), an error in how an app performs a task, or a formatting inconsistency that could make it difficult to update an app.

**Hosting platform**  
The compute, storage, and networking resources required to host one or more apps. A [hosting platform](GLOSSARY.md) is an Information System. Each [hosting platform](GLOSSARY.md) has one [system security plan](GLOSSARY.md).

**Mappings file**  
A file that records mappings between [scanner](GLOSSARY.md) rules, [components](GLOSSARY.md), and implemented [controls](GLOSSARY.md). Each mapping includes a reason and an expiry date. The [security team](GLOSSARY.md) reviews proposed mappings. Finch uses the file to generate [suppressions](GLOSSARY.md).

**Mappings file version**  
A tagged version of a [mappings file](GLOSSARY.md) for one [environment](GLOSSARY.md) of one [hosting platform](GLOSSARY.md). [Components](GLOSSARY.md) and [controls](GLOSSARY.md) can differ between [environments](GLOSSARY.md).

**Mitigation**  
The result when an implemented [control](GLOSSARY.md) reduces the likelihood or impact of harm from a [threat](GLOSSARY.md) or [vulnerability](GLOSSARY.md).

**Platform team**  
The people responsible for operating a [hosting platform](GLOSSARY.md).

**Residual risk**  
The [risk](GLOSSARY.md) that remains after a [control](GLOSSARY.md) has been implemented.

**Risk**  
The potential for harm, measured by its likelihood and impact.

**Risk acceptance**  
The decision to accept [risk](GLOSSARY.md) without taking further action to reduce it.

**Risk avoidance**  
The decision to eliminate [risk](GLOSSARY.md) by removing its source. For example, a [tenant](GLOSSARY.md) can avoid the [risk](GLOSSARY.md) associated with a vulnerable feature by removing the feature.

**Risk reduction**  
The decision to reduce the likelihood or impact of [risk](GLOSSARY.md) by implementing a [control](GLOSSARY.md). For example, a [tenant](GLOSSARY.md) can reduce the [risk](GLOSSARY.md) of unauthorized access by requiring multi-factor authentication.

**Risk transference**  
The decision to transfer the impact of [risk](GLOSSARY.md) to another entity. For example, a [tenant](GLOSSARY.md) can transfer some of the [risk](GLOSSARY.md) introduced by database server maintenance to a Cloud Service Provider by using a Platform as a Service (PaaS) offering that includes server patching.

**Scanner**  
A tool that applies rules to a [tenant’s](GLOSSARY.md) app and reports [findings](GLOSSARY.md). A [scanner](GLOSSARY.md) can use [context](GLOSSARY.md) to determine which [findings](GLOSSARY.md) a [hosting platform’s](GLOSSARY.md) [controls](GLOSSARY.md) address.

**Security team**  
The people responsible for protecting a [hosting platform](GLOSSARY.md) and deciding whether an app may be deployed to it. The [security team](GLOSSARY.md) reviews the app’s Body of Evidence to inform that decision.

**Suppression**  
An instruction that tells a [scanner](GLOSSARY.md) to ignore a [finding](GLOSSARY.md) because an implemented [control](GLOSSARY.md) mitigates the associated [risk](GLOSSARY.md).

**System security plan**  
The document that defines a [hosting platform’s](GLOSSARY.md) security requirements and describes how the [platform team](GLOSSARY.md) meets them.

**Tenant**  
The people responsible for a specific app deployed on a [hosting platform](GLOSSARY.md).

**Threat**  
A circumstance or event that can harm a [hosting platform](GLOSSARY.md), the apps deployed on it, or the data they contain. A [threat](GLOSSARY.md) can cause harm by exploiting a [vulnerability](GLOSSARY.md).

**Vulnerability**  
A weakness, such as a design flaw, misconfiguration, or exposure, that a [threat](GLOSSARY.md) can exploit to cause harm to a [hosting platform](GLOSSARY.md), its apps, or their data. A [scanner](GLOSSARY.md) can report a [finding](GLOSSARY.md) that identifies a [vulnerability](GLOSSARY.md).
