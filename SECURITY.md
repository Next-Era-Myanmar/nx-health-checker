# Security Policy

## Supported Versions

We release patches for security vulnerabilities in the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security vulnerability, please report it to us as described below.

### How to Report

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to our security team:

**Email**: [nxera.solutions@gmail.com](mailto:nxera.solutions@gmail.com)

**Subject**: `[SECURITY] NX Health Checker Vulnerability Report`

### What to Include

Please include the following information in your report:

1. **Description**: A clear description of the vulnerability
2. **Steps to Reproduce**: Detailed steps to reproduce the issue
3. **Impact**: Potential impact of the vulnerability
4. **Environment**: Affected versions and environment details
5. **Proof of Concept**: If applicable, include a proof of concept
6. **Suggested Fix**: If you have suggestions for fixing the issue

### Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Resolution**: As quickly as possible, typically within 30 days

### What to Expect

1. **Acknowledgment**: We will acknowledge receipt of your report
2. **Investigation**: We will investigate the reported vulnerability
3. **Updates**: We will provide regular updates on our progress
4. **Resolution**: We will work to resolve the issue and release a fix
5. **Credit**: We will credit you in our security advisories (if desired)

## Security Best Practices

### For Users

1. **Keep Updated**: Always use the latest version of NX Health Checker
2. **Secure Configuration**: Use strong passwords and secure environment variables
3. **Network Security**: Deploy behind firewalls and use HTTPS in production
4. **Regular Backups**: Maintain regular backups of your database
5. **Monitor Logs**: Regularly review application logs for suspicious activity

### For Developers

1. **Input Validation**: Always validate and sanitize user input
2. **Authentication**: Implement proper authentication and authorization
3. **Error Handling**: Don't expose sensitive information in error messages
4. **Dependencies**: Keep dependencies updated and scan for vulnerabilities
5. **Code Review**: Conduct thorough code reviews for security issues

## Security Features

NX Health Checker includes several security features:

- **Session-based Authentication**: Secure session management
- **Input Validation**: Comprehensive input validation and sanitization
- **SQL Injection Protection**: Parameterized queries with sqlx
- **CORS Protection**: Configurable CORS policies
- **Non-root Execution**: Docker containers run as non-root user
- **Secure Headers**: Security headers for web responses
- **Environment Variables**: Secure configuration management

## Vulnerability Disclosure

When we discover or receive reports of security vulnerabilities, we will:

1. **Assess**: Evaluate the severity and impact
2. **Fix**: Develop and test a fix
3. **Disclose**: Publish a security advisory
4. **Release**: Release a patched version
5. **Document**: Update security documentation

## Security Advisories

Security advisories will be published in:

- **GitHub Security Advisories**: [Security Advisories](https://github.com/next-era/nx-health-checker/security/advisories)
- **Release Notes**: Included in version release notes
- **Email**: Sent to registered users (if applicable)

## Contact

**Security Team**
- **Email**: [nxera.solutions@gmail.com](mailto:nxera.solutions@gmail.com)
- **Company**: Next Era
- **Website**: [nxera.top](https://nxera.top)

---

**Next Era** - Committed to security and reliability.

*Last updated: September 4, 2025*
