# Glossary

## Component

An OSCAL-based representation of a [hosting platform](#hosting-platform) element and the implemented [controls](#control) it supports, as documented in the [hosting platform’s](#hosting-platform) [system security plan](#system-security-plan). For example, a Web Application Firewall is a [component](#component) that can support implemented [controls](#control) from the System and Communications Protection (SC) family.

## Context

One or more [components](#component) within an [environment](#environment) on a [hosting platform](#hosting-platform).

## Control

The means of managing [risk](#risk) (e.g., policies, procedures, and guidelines) to a [hosting platform](#hosting-platform) and the apps on it. A [control](#control) can be administrative, technical, or physical in nature. A [tenant](#tenant) inherits a [hosting platform’s](#hosting-platform) [controls](#control) when the [tenant’s](#tenant) app is deployed on the [hosting platform](#hosting-platform).

## Environment

The compute, storage, and networking resources of a [hosting platform](#hosting-platform) that have a unique set of IP address space, cryptographic material, and [risk](#risk) thresholds. Examples include Development, Test, and Production.

## Expiry

The last date on which a [mapping](#mapping) is valid.

## Finding

A result reported by a [scanner](#scanner) when it detects a violation of a rule. A [finding](#finding) can indicate a [vulnerability](#vulnerability), an error in how an app performs a task, or a formatting inconsistency that could make it difficult to update an app.

## Hosting platform

The compute, storage, and networking resources required to host one or more apps. A [hosting platform](#hosting-platform) is an Information System. Each [hosting platform](#hosting-platform) has one [system security plan](#system-security-plan).

## Justification

An explanation of why an implemented [control](#control) mitigates the [risk](#risk) associated with a [scanner](#scanner) rule.

## Mapping

A relationship between a [scanner](#scanner) rule, a [component](#component), and an implemented [control](#control). It includes a [justification](#justification) and an [expiry](#expiry).

## Mappings file

A file that records [mappings](#mapping). The [security team](#security-team) reviews proposed [mappings](#mapping). Finch uses the file to generate [suppressions](#suppression).

## Mappings file version

A tagged version of a [mappings file](#mappings-file) for one [environment](#environment) of one [hosting platform](#hosting-platform). [Components](#component) and [controls](#control) can differ between [environments](#environment).

## Mitigation

The result when an implemented [control](#control) reduces the likelihood or impact of harm from a [threat](#threat) or [vulnerability](#vulnerability).

## Platform team

The people responsible for implementing and operating a [hosting platform](#hosting-platform) and helping apps integrate with it.

## Residual risk

The [risk](#risk) that remains after a [control](#control) has been implemented.

## Risk

The potential for harm, measured by its likelihood and impact.

## Risk acceptance

The decision to accept [risk](#risk) without taking further action to reduce it.

## Risk avoidance

The decision to eliminate [risk](#risk) by removing its source. For example, a [tenant](#tenant) can avoid the [risk](#risk) associated with a vulnerable feature by removing the feature.

## Risk reduction

The decision to reduce the likelihood or impact of [risk](#risk) by implementing a [control](#control). For example, a [tenant](#tenant) can reduce the [risk](#risk) of unauthorized access by requiring multi-factor authentication.

## Risk transference

The decision to transfer the impact of [risk](#risk) to another entity. For example, a [tenant](#tenant) can transfer some of the [risk](#risk) introduced by database server maintenance to a Cloud Service Provider by using a Platform as a Service (PaaS) offering that includes server patching.

## Scanner

A tool that applies rules to a [tenant’s](#tenant) app and reports [findings](#finding). A [scanner](#scanner) can use [context](#context) to determine which [findings](#finding) a [hosting platform’s](#hosting-platform) [controls](#control) address.

## Security team

The people responsible for protecting a [hosting platform](#hosting-platform) and deciding whether an app may be deployed to it. 

## Suppression

An instruction that tells a [scanner](#scanner) to ignore a [finding](#finding) because an implemented [control](#control) mitigates the associated [risk](#risk).

## System security plan

The document that defines a [hosting platform’s](#hosting-platform) security requirements and describes how the [platform team](#platform-team) meets them.

## Tenant

The people responsible for a specific app deployed on a [hosting platform](#hosting-platform).

## Threat

A circumstance or event that can harm a [hosting platform](#hosting-platform), the apps deployed on it, or the data they contain. A [threat](#threat) can cause harm by exploiting a [vulnerability](#vulnerability).

## User

Anyone who uses Finch (e.g., a [tenant](#tenant) or [security team](#security-team)).

## Vulnerability

A weakness, such as a design flaw, misconfiguration, or exposure, that a [threat](#threat) can exploit to cause harm to a [hosting platform](#hosting-platform), its apps, or their data. A [scanner](#scanner) can report a [finding](#finding) that identifies a [vulnerability](#vulnerability).
