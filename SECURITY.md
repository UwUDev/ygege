# Security Policy

## Supported Versions

Only the most recent stable release of Ygégé is guaranteed to have security patches applied. **Running older stable versions may leave you vulnerable to security risks**; always run the latest version, or avoid exposing your instance to the public Internet to minimize risk. Further, please always understand that providing a person with administrative access to your Ygégé configuration is a risk, as administrators can perform many destructive or damaging actions, regardless of any potential security issues.

As such, this security policy only applies to the most recent stable version of Ygégé. Flaws present in old stable versions which are not present in the current stable version **will not** be fixed.

## Vulnerability Triage

We ask you to please review the following details before reporting an issue.

* We are aware that many configuration and administrative operations may have security implications. Due to the internal details of how Ygégé works, many of these are unavoidable, and we expect users to understand that providing administrative access to configuration files is very sensitive as mentioned above. We consider any vulnerabilities that **exclusively require administrative privileges or file system access** to be a low priority, and those should be disclosed in normal GitHub Issues that can be fixed like any other bug.

* We have a public list of known vulnerabilities in our [Security Advisories](https://github.com/UwUDev/ygege/security/advisories). If your vulnerability is **already disclosed there**, please do not duplicate effort by re-reporting it to our security team.

* Vulnerabilities that can not be **exploited remotely** are considered low- to medium-priority bugs instead. For example, anything that requires shell access to the Ygégé server, manual manipulation of configuration files, etc.

* Vulnerability reports about our project infrastructure (our website, servers, CI/CD, etc.) are welcome, but please tag those separately with `[Ygégé Infrastructure]` in the advisory title. Please also be aware that our infrastructure team does follow the news and has a standard patch policy, so duplicating publicly-known reports here is not usually necessary.

## Reporting a Vulnerability

Once self-triaged and found to be a new and relevant vulnerability, please reach out for responsible disclosure through [GitHub Security Advisories](https://github.com/UwUDev/ygege/security/advisories/new).

When providing a report, please ensure that you:

1. Begin your advisory title with `[Ygégé Security]`. This helps ensure proper visibility and prioritization.

2. Start with an "overview" section, **written for public view**, that describes at a high level what is affected, and the possible consequences. Ideally, we will use this verbatim as the description of the GHSA.

3. Continue on with a "details" section outlining any code or API investigation you have done and, if possible, any suggested fixes. Please provide as much context and detail as you can, including, ideally, a process for reliably triggering the vulnerability so we may test fixes with it.

4. Please provide your GitHub username so we may invite you into the GHSA and provide proper credit.

Once a report is received, it will be reviewed and, if applicable, we will create a GHSA and invite the reporter as well as the relevant team(s) to discuss the issue and work towards a fix.

## Post-Disclosure Process

As a pure volunteer project, **we recognize that we may sometimes be slow to handle vulnerabilities**; we greatly **appreciate patience and the absence of arbitrary timelines for disclosures**, especially for complex vulnerabilities. This includes initial responses. You are welcome to follow up on your GitHub advisory if things go too long without a reply, though please remain polite and patient.

Generally speaking, unless we are very close to a new major release, we will create a point release for any fixes for major vulnerabilities as soon as possible. When close to a new major release, we may wish to defer the fix to the major release instead to avoid duplicated work.

Once a new version is released, **we will wait at least 7 days (1 week) to publish our GHSA**. We believe this time is a fair balance between quick disclosure and the time needed by the majority of users to update their instances. We ask that all 3rd party disclosures (blog posts, etc.) occur **after** our GHSA is published.

CVEs will be requested by us through the GitHub Security interface and published along with the disclosure.

## Security Best Practices

When deploying Ygégé:

- Always use the latest stable version
- Keep your Docker images updated regularly
- Use strong authentication credentials for Telegram API
- Consider running behind a reverse proxy with proper SSL/TLS
- Regularly review your configuration for exposed API keys and credentials
- Monitor logs for suspicious activity
- Restrict network access to trusted sources when possible
- Never commit sensitive credentials to version control

## Acknowledgments

We are grateful to all security researchers who responsibly disclose vulnerabilities and help keep the Ygégé community safe.
