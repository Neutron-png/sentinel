use crate::methodology::{
    MethodologyActivity, MethodologyDefinition, MethodologySection, MethodologyTask,
};

pub fn owasp_definition() -> MethodologyDefinition {
    MethodologyDefinition {
        name: "OWASP".into(),
        version: "4.2".into(),
        description: "OWASP Web Security Testing Guide (WSTG) - The premier cybersecurity testing resource for web application developers and security professionals.".into(),
        sections: vec![
            info_gathering(),
            config_testing(),
            identity_testing(),
            authn_testing(),
            authz_testing(),
            session_testing(),
            input_validation(),
            error_handling(),
            cryptography(),
            business_logic(),
            client_side(),
            api_testing(),
        ],
    }
}

fn tasks(ts: &[(&str, &str, &str)]) -> Vec<MethodologyTask> {
    ts.iter()
        .map(|(t, d, r)| MethodologyTask {
            title: t.to_string(),
            description: d.to_string(),
            reference: r.to_string(),
        })
        .collect()
}

fn activity(title: &str, description: &str, ts: Vec<MethodologyTask>) -> MethodologyActivity {
    MethodologyActivity {
        title: title.into(),
        description: description.into(),
        tasks: ts,
    }
}

fn section(title: &str, acts: Vec<MethodologyActivity>) -> MethodologySection {
    MethodologySection {
        title: title.into(),
        activities: acts,
    }
}

// ── WSTG-INFO: Information Gathering ──

fn info_gathering() -> MethodologySection {
    section(
        "WSTG-INFO: Information Gathering",
        vec![
            activity(
                "Search Engine Discovery",
                "Conduct search engine discovery and reconnaissance for information leakage",
                tasks(&[
                    (
                        "Identify search engine indexed content",
                        "Use search engines to discover indexed pages and sensitive information",
                        "WSTG-INFO-01",
                    ),
                    (
                        "Perform Google dorking",
                        "Use advanced search operators to find exposed data",
                        "WSTG-INFO-01",
                    ),
                ]),
            ),
            activity(
                "Fingerprint Web Server",
                "Identify web server type and version",
                tasks(&[
                    (
                        "Banner grabbing",
                        "Capture server response headers and banner information",
                        "WSTG-INFO-02",
                    ),
                    (
                        "Analyze HTTP response headers",
                        "Extract server version from response headers",
                        "WSTG-INFO-02",
                    ),
                    (
                        "Probe for server-specific behaviors",
                        "Test error pages and malformed requests for version disclosure",
                        "WSTG-INFO-02",
                    ),
                ]),
            ),
            activity(
                "Review Web Server Metafiles",
                "Analyze robots.txt, sitemap.xml and other metadata files",
                tasks(&[
                    (
                        "Analyze robots.txt",
                        "Review disallowed paths for sensitive endpoints",
                        "WSTG-INFO-03",
                    ),
                    (
                        "Analyze sitemap.xml",
                        "Enumerate application structure from sitemap",
                        "WSTG-INFO-03",
                    ),
                    (
                        "Check for exposed .git/config",
                        "Look for version control information disclosure",
                        "WSTG-INFO-03",
                    ),
                ]),
            ),
            activity(
                "Enumerate Applications on Web Server",
                "Discover co-hosted applications on the same server",
                tasks(&[
                    (
                        "Virtual host enumeration",
                        "Identify virtual hosts through DNS and HTTP Host header fuzzing",
                        "WSTG-INFO-04",
                    ),
                    (
                        "Port scanning",
                        "Identify additional services running on target infrastructure",
                        "WSTG-INFO-04",
                    ),
                ]),
            ),
            activity(
                "Review Web Page Content",
                "Identify information leakage from page content",
                tasks(&[
                    (
                        "Source code review for comments",
                        "Inspect HTML/JS source for developer comments and secrets",
                        "WSTG-INFO-05",
                    ),
                    (
                        "Identify client-side data exposure",
                        "Check for embedded credentials, tokens, and API keys",
                        "WSTG-INFO-05",
                    ),
                    (
                        "Analyze HTML metadata tags",
                        "Review meta tags for version disclosure and sensitive info",
                        "WSTG-INFO-05",
                    ),
                ]),
            ),
            activity(
                "Identify Application Entry Points",
                "Map all application entry and exit points",
                tasks(&[
                    (
                        "Endpoint enumeration",
                        "Identify all URL endpoints, API routes, and parameters",
                        "WSTG-INFO-06",
                    ),
                    (
                        "HTTP method discovery",
                        "Enumerate supported HTTP methods per endpoint",
                        "WSTG-INFO-06",
                    ),
                ]),
            ),
            activity(
                "Map Execution Paths",
                "Trace application execution flow through the codebase",
                tasks(&[
                    (
                        "Identify redirect flows",
                        "Map all redirect and forward paths",
                        "WSTG-INFO-07",
                    ),
                    (
                        "Trace multi-step processes",
                        "Document multi-page workflows and state transitions",
                        "WSTG-INFO-07",
                    ),
                ]),
            ),
            activity(
                "Fingerprint Web Application Framework",
                "Identify the framework and technology stack in use",
                tasks(&[
                    (
                        "Analyze cookies for framework hints",
                        "Parse cookie names for framework identification markers",
                        "WSTG-INFO-08",
                    ),
                    (
                        "Check for framework-specific paths",
                        "Look for default framework paths and resources",
                        "WSTG-INFO-08",
                    ),
                    (
                        "Analyze error messages",
                        "Examine stack traces for framework version information",
                        "WSTG-INFO-08",
                    ),
                ]),
            ),
            activity(
                "Fingerprint Web Application",
                "Determine the specific application and its version",
                tasks(&[
                    (
                        "Identify known application patterns",
                        "Look for CMS-specific paths, headers, and behaviors",
                        "WSTG-INFO-09",
                    ),
                    (
                        "Check for admin interfaces",
                        "Enumerate common admin panel locations",
                        "WSTG-INFO-09",
                    ),
                ]),
            ),
            activity(
                "Map Application Architecture",
                "Understand the overall application and network architecture",
                tasks(&[
                    (
                        "Identify CDN and reverse proxy usage",
                        "Detect front-end services modifying requests/responses",
                        "WSTG-INFO-10",
                    ),
                    (
                        "Map load balancer behavior",
                        "Test for session affinity and load balancer configurations",
                        "WSTG-INFO-10",
                    ),
                ]),
            ),
        ],
    )
}

// ── WSTG-CONF: Configuration Testing ──

fn config_testing() -> MethodologySection {
    section(
        "WSTG-CONF: Configuration Testing",
        vec![
            activity(
                "Test Network Configuration",
                "Verify network infrastructure security configuration",
                tasks(&[
                    (
                        "Check for TLS/SSL configuration issues",
                        "Validate certificate chain, cipher suites, and protocol versions",
                        "WSTG-CONF-01",
                    ),
                    (
                        "Test for HTTP Strict Transport Security",
                        "Verify HSTS header presence and configuration",
                        "WSTG-CONF-01",
                    ),
                ]),
            ),
            activity(
                "Test Application Platform Configuration",
                "Verify application server and platform security",
                tasks(&[
                    (
                        "Check for default credentials",
                        "Test for unchanged default admin credentials",
                        "WSTG-CONF-02",
                    ),
                    (
                        "Review directory listings",
                        "Check if directory listing is enabled on the server",
                        "WSTG-CONF-02",
                    ),
                    (
                        "Test for unnecessary HTTP methods",
                        "Verify that dangerous HTTP methods are disabled",
                        "WSTG-CONF-02",
                    ),
                ]),
            ),
            activity(
                "Test File Extensions Handling",
                "Test how the server handles different file extensions",
                tasks(&[
                    (
                        "Test for backup file exposure",
                        "Check for .bak, .old, .swp file accessibility",
                        "WSTG-CONF-03",
                    ),
                    (
                        "Test for source code disclosure",
                        "Check .phps, .asp source code exposure",
                        "WSTG-CONF-03",
                    ),
                ]),
            ),
            activity(
                "Review Security Headers",
                "Analyze HTTP security headers",
                tasks(&[
                    (
                        "Check Content-Security-Policy",
                        "Verify CSP header presence and effectiveness",
                        "WSTG-CONF-04",
                    ),
                    (
                        "Check X-Frame-Options",
                        "Test for clickjacking protection",
                        "WSTG-CONF-04",
                    ),
                    (
                        "Check X-Content-Type-Options",
                        "Verify MIME type sniffing protection",
                        "WSTG-CONF-04",
                    ),
                    (
                        "Check Referrer-Policy",
                        "Verify referrer information leak protection",
                        "WSTG-CONF-04",
                    ),
                ]),
            ),
            activity(
                "Test CORS Configuration",
                "Validate Cross-Origin Resource Sharing setup",
                tasks(&[
                    (
                        "Test CORS with trusted origins",
                        "Verify CORS headers with valid origin values",
                        "WSTG-CONF-05",
                    ),
                    (
                        "Test CORS with null and wildcard origins",
                        "Check for overly permissive CORS policies",
                        "WSTG-CONF-05",
                    ),
                ]),
            ),
        ],
    )
}

// ── WSTG-IDNT: Identity Management ──

fn identity_testing() -> MethodologySection {
    section(
        "WSTG-IDNT: Identity Management Testing",
        vec![
            activity(
                "Test Role Definitions",
                "Verify that user roles are properly defined and enforced",
                tasks(&[
                    (
                        "Enumerate user roles",
                        "Identify all defined user roles within the application",
                        "WSTG-IDNT-01",
                    ),
                    (
                        "Test vertical privilege boundaries",
                        "Attempt to access higher-privilege functionality",
                        "WSTG-IDNT-01",
                    ),
                ]),
            ),
            activity(
                "Test User Registration",
                "Verify the security of the registration process",
                tasks(&[
                    (
                        "Test for automated registration",
                        "Check CAPTCHA or rate-limiting on registration",
                        "WSTG-IDNT-02",
                    ),
                    (
                        "Test for duplicate user registration",
                        "Verify uniqueness enforcement for user accounts",
                        "WSTG-IDNT-02",
                    ),
                    (
                        "Test weak password policy",
                        "Check minimum password requirements",
                        "WSTG-IDNT-02",
                    ),
                ]),
            ),
            activity(
                "Test Account Provisioning",
                "Review the account creation and provisioning process",
                tasks(&[
                    (
                        "Verify email verification process",
                        "Test account activation flow for bypasses",
                        "WSTG-IDNT-03",
                    ),
                    (
                        "Test for privilege assignment during registration",
                        "Check if roles can be self-assigned during signup",
                        "WSTG-IDNT-03",
                    ),
                ]),
            ),
            activity(
                "Test Account Enumeration",
                "Determine if valid usernames can be identified",
                tasks(&[
                    (
                        "Test login page responses",
                        "Compare responses for valid vs invalid usernames",
                        "WSTG-IDNT-04",
                    ),
                    (
                        "Test password reset enumeration",
                        "Check password reset responses for username disclosure",
                        "WSTG-IDNT-04",
                    ),
                    (
                        "Test registration enumeration",
                        "Check if existing usernames are disclosed during registration",
                        "WSTG-IDNT-04",
                    ),
                ]),
            ),
        ],
    )
}

// ── WSTG-AUTHN: Authentication Testing ──

fn authn_testing() -> MethodologySection {
    section(
        "WSTG-AUTHN: Authentication Testing",
        vec![
            activity(
                "Test Credentials Transport",
                "Verify credentials are transmitted securely",
                tasks(&[
                    (
                        "Verify HTTPS for login",
                        "Confirm login page and POST use HTTPS",
                        "WSTG-AUTHN-01",
                    ),
                    (
                        "Test credential exposure in URL",
                        "Check for credentials in query strings or Referer headers",
                        "WSTG-AUTHN-01",
                    ),
                ]),
            ),
            activity(
                "Test Default Credentials",
                "Check for default or guessable credentials",
                tasks(&[
                    (
                        "Test common default credentials",
                        "Try admin/admin, root/root, and vendor defaults",
                        "WSTG-AUTHN-02",
                    ),
                    (
                        "Test for credential stuffing susceptibility",
                        "Check if rate limiting exists for login attempts",
                        "WSTG-AUTHN-02",
                    ),
                ]),
            ),
            activity(
                "Test Account Lockout",
                "Verify the account lockout mechanism",
                tasks(&[
                    (
                        "Test lockout threshold",
                        "Determine number of failed attempts before lockout",
                        "WSTG-AUTHN-03",
                    ),
                    (
                        "Test lockout bypass techniques",
                        "Try account lockout circumvention methods",
                        "WSTG-AUTHN-03",
                    ),
                ]),
            ),
            activity(
                "Test Authentication Bypass",
                "Attempt to bypass authentication mechanisms",
                tasks(&[
                    (
                        "Test parameter modification",
                        "Modify authentication-related request parameters",
                        "WSTG-AUTHN-04",
                    ),
                    (
                        "Test direct page requests",
                        "Access protected pages directly without authentication",
                        "WSTG-AUTHN-04",
                    ),
                    (
                        "Test SQL injection in login",
                        "Attempt SQL injection in username/password fields",
                        "WSTG-AUTHN-04",
                    ),
                ]),
            ),
            activity(
                "Test Password Reset",
                "Test the password reset/recovery functionality",
                tasks(&[
                    (
                        "Test password reset token strength",
                        "Analyze reset token randomness and expiration",
                        "WSTG-AUTHN-05",
                    ),
                    (
                        "Test reset token leakage",
                        "Check if reset tokens appear in Referer headers or logs",
                        "WSTG-AUTHN-05",
                    ),
                    (
                        "Test for host header injection in reset",
                        "Attempt to poison password reset links",
                        "WSTG-AUTHN-05",
                    ),
                ]),
            ),
            activity(
                "Test Multi-Factor Authentication",
                "Review MFA implementation security",
                tasks(&[
                    (
                        "Test MFA bypass techniques",
                        "Attempt to skip or brute-force MFA verification",
                        "WSTG-AUTHN-06",
                    ),
                    (
                        "Test MFA enrollment process",
                        "Verify MFA setup cannot be hijacked",
                        "WSTG-AUTHN-06",
                    ),
                ]),
            ),
        ],
    )
}

// ── WSTG-AUTHZ: Authorization Testing ──

fn authz_testing() -> MethodologySection {
    section(
        "WSTG-AUTHZ: Authorization Testing",
        vec![
            activity(
                "Test Directory Traversal",
                "Check for path traversal vulnerabilities",
                tasks(&[
                    (
                        "Test relative path traversal",
                        "Attempt ../ sequences to access files outside web root",
                        "WSTG-AUTHZ-01",
                    ),
                    (
                        "Test absolute path traversal",
                        "Attempt absolute path references to system files",
                        "WSTG-AUTHZ-01",
                    ),
                    (
                        "Test encoded traversal sequences",
                        "Try URL-encoded and double-encoded traversal payloads",
                        "WSTG-AUTHZ-01",
                    ),
                ]),
            ),
            activity(
                "Test Privilege Escalation",
                "Attempt to access functionality above assigned privilege level",
                tasks(&[
                    (
                        "Test horizontal privilege escalation",
                        "Access other users' resources by changing identifiers",
                        "WSTG-AUTHZ-02",
                    ),
                    (
                        "Test vertical privilege escalation",
                        "Access admin functionality from a standard user account",
                        "WSTG-AUTHZ-02",
                    ),
                    (
                        "Test for Insecure Direct Object References",
                        "Enumerate and modify object IDs to access unauthorized resources",
                        "WSTG-AUTHZ-02",
                    ),
                ]),
            ),
        ],
    )
}

// ── WSTG-SESS: Session Management Testing ──

fn session_testing() -> MethodologySection {
    section(
        "WSTG-SESS: Session Management Testing",
        vec![
            activity(
                "Test Session Management",
                "Verify session token generation and handling",
                tasks(&[
                    (
                        "Analyze session token entropy",
                        "Check session ID randomness and predictability",
                        "WSTG-SESS-01",
                    ),
                    (
                        "Test session fixation",
                        "Attempt to force a known session ID onto a victim",
                        "WSTG-SESS-01",
                    ),
                    (
                        "Test concurrent sessions",
                        "Check if multiple simultaneous sessions are allowed",
                        "WSTG-SESS-01",
                    ),
                ]),
            ),
            activity(
                "Test Cookie Attributes",
                "Verify secure cookie configuration",
                tasks(&[
                    (
                        "Check HttpOnly flag",
                        "Verify session cookies have HttpOnly attribute",
                        "WSTG-SESS-02",
                    ),
                    (
                        "Check Secure flag",
                        "Verify session cookies have Secure attribute",
                        "WSTG-SESS-02",
                    ),
                    (
                        "Check SameSite attribute",
                        "Verify cookies have appropriate SameSite configuration",
                        "WSTG-SESS-02",
                    ),
                    (
                        "Check cookie path and domain",
                        "Verify cookie scope is properly restricted",
                        "WSTG-SESS-02",
                    ),
                ]),
            ),
            activity(
                "Test Logout Functionality",
                "Verify session termination on logout",
                tasks(&[
                    (
                        "Test server-side session invalidation",
                        "Verify session is destroyed on server after logout",
                        "WSTG-SESS-03",
                    ),
                    (
                        "Test session token reuse after logout",
                        "Check if logged-out session tokens remain valid",
                        "WSTG-SESS-03",
                    ),
                    (
                        "Test logout CSRF",
                        "Check if logout requires anti-CSRF protection",
                        "WSTG-SESS-03",
                    ),
                ]),
            ),
            activity(
                "Test Session Timeout",
                "Verify automatic session expiration",
                tasks(&[
                    (
                        "Test idle timeout",
                        "Verify session expires after period of inactivity",
                        "WSTG-SESS-04",
                    ),
                    (
                        "Test absolute timeout",
                        "Verify session expires after maximum lifetime",
                        "WSTG-SESS-04",
                    ),
                ]),
            ),
        ],
    )
}

// ── WSTG-INPV: Input Validation Testing ──

fn input_validation() -> MethodologySection {
    section(
        "WSTG-INPV: Input Validation Testing",
        vec![
            activity(
                "Test for Reflected XSS",
                "Check for cross-site scripting in reflected parameters",
                tasks(&[
                    (
                        "Test basic XSS payloads",
                        "Inject script tags and event handlers in reflected parameters",
                        "WSTG-INPV-01",
                    ),
                    (
                        "Test attribute injection",
                        "Inject into HTML tag attributes",
                        "WSTG-INPV-01",
                    ),
                    (
                        "Test encoded XSS payloads",
                        "Attempt URL-encoded and HTML-entity encoded payloads",
                        "WSTG-INPV-01",
                    ),
                ]),
            ),
            activity(
                "Test for Stored XSS",
                "Check for persistent cross-site scripting",
                tasks(&[
                    (
                        "Test stored payloads in user profiles",
                        "Inject XSS in profile fields visible to other users",
                        "WSTG-INPV-02",
                    ),
                    (
                        "Test stored payloads in comments",
                        "Inject XSS in comment and message fields",
                        "WSTG-INPV-02",
                    ),
                ]),
            ),
            activity(
                "Test for SQL Injection",
                "Check for SQL injection vulnerabilities",
                tasks(&[
                    (
                        "Test error-based SQLi",
                        "Trigger database errors to confirm injection",
                        "WSTG-INPV-05",
                    ),
                    (
                        "Test UNION-based SQLi",
                        "Use UNION SELECT to extract data",
                        "WSTG-INPV-05",
                    ),
                    (
                        "Test blind SQLi",
                        "Use boolean-based and time-based blind injection techniques",
                        "WSTG-INPV-05",
                    ),
                    (
                        "Test for second-order SQLi",
                        "Check for delayed SQL injection in stored data",
                        "WSTG-INPV-05",
                    ),
                ]),
            ),
            activity(
                "Test for Command Injection",
                "Check for OS command injection",
                tasks(&[
                    (
                        "Test command separators",
                        "Attempt command chaining with ; | && ||",
                        "WSTG-INPV-07",
                    ),
                    (
                        "Test blind command injection",
                        "Use time-based techniques to detect blind injection",
                        "WSTG-INPV-07",
                    ),
                ]),
            ),
            activity(
                "Test for XML Injection",
                "Check for XML external entity and injection attacks",
                tasks(&[
                    (
                        "Test for XXE injection",
                        "Attempt to read local files via XML external entities",
                        "WSTG-INPV-08",
                    ),
                    (
                        "Test for XPath injection",
                        "Attempt XPath query injection in XML processing",
                        "WSTG-INPV-08",
                    ),
                ]),
            ),
            activity(
                "Test for SSTI",
                "Check for server-side template injection",
                tasks(&[
                    (
                        "Identify template engine",
                        "Use template-specific syntax to detect engine",
                        "WSTG-INPV-12",
                    ),
                    (
                        "Exploit SSTI for code execution",
                        "Attempt remote code execution through template injection",
                        "WSTG-INPV-12",
                    ),
                ]),
            ),
            activity(
                "Test for HTTP Parameter Pollution",
                "Check for parameter pollution vulnerabilities",
                tasks(&[
                    (
                        "Test duplicate parameter handling",
                        "Send duplicate parameters with different values",
                        "WSTG-INPV-14",
                    ),
                    (
                        "Test HPP in multi-tier applications",
                        "Check how backend systems handle duplicate parameters",
                        "WSTG-INPV-14",
                    ),
                ]),
            ),
        ],
    )
}

// ── WSTG-ERRH: Error Handling ──

fn error_handling() -> MethodologySection {
    section(
        "WSTG-ERRH: Error Handling",
        vec![
            activity(
                "Test Error Handling",
                "Analyze application error responses for information disclosure",
                tasks(&[
                    (
                        "Analyze stack traces",
                        "Trigger errors and examine stack traces for sensitive data",
                        "WSTG-ERRH-01",
                    ),
                    (
                        "Test custom error pages",
                        "Check for consistent error handling across the application",
                        "WSTG-ERRH-01",
                    ),
                    (
                        "Trigger database errors",
                        "Force SQL errors to check for verbose error messages",
                        "WSTG-ERRH-01",
                    ),
                ]),
            ),
            activity(
                "Test for Improper Error Handling",
                "Check for inconsistent error handling patterns",
                tasks(&[
                    (
                        "Test for error code leakage",
                        "Check if internal error codes are exposed to users",
                        "WSTG-ERRH-02",
                    ),
                    (
                        "Test debug mode exposure",
                        "Check if debug or development mode is enabled in production",
                        "WSTG-ERRH-02",
                    ),
                ]),
            ),
        ],
    )
}

// ── WSTG-CRYP: Cryptography ──

fn cryptography() -> MethodologySection {
    section(
        "WSTG-CRYP: Weak Cryptography",
        vec![
            activity(
                "Test Transport Layer Security",
                "Verify proper TLS/SSL implementation",
                tasks(&[
                    (
                        "Check supported TLS versions",
                        "Verify TLS 1.2+ and disablement of SSL/TLS 1.0/1.1",
                        "WSTG-CRYP-01",
                    ),
                    (
                        "Check cipher suite configuration",
                        "Verify strong cipher suites and disable weak ciphers",
                        "WSTG-CRYP-01",
                    ),
                    (
                        "Test for insecure renegotiation",
                        "Check for TLS renegotiation vulnerabilities",
                        "WSTG-CRYP-01",
                    ),
                ]),
            ),
            activity(
                "Test for Sensitive Data in Transit",
                "Verify data transmission security",
                tasks(&[
                    (
                        "Check for mixed content",
                        "Identify HTTP resources loaded on HTTPS pages",
                        "WSTG-CRYP-02",
                    ),
                    (
                        "Test for sensitive data in URL",
                        "Check if sensitive parameters appear in URL query strings",
                        "WSTG-CRYP-02",
                    ),
                ]),
            ),
            activity(
                "Test for Weak Encryption",
                "Review cryptographic implementation",
                tasks(&[
                    (
                        "Check for hardcoded keys",
                        "Search for hardcoded encryption keys and secrets",
                        "WSTG-CRYP-03",
                    ),
                    (
                        "Check for weak algorithms",
                        "Identify use of MD5, SHA1, RC4, DES in security contexts",
                        "WSTG-CRYP-03",
                    ),
                    (
                        "Test for predictable values",
                        "Check for weak random number generation in tokens",
                        "WSTG-CRYP-03",
                    ),
                ]),
            ),
        ],
    )
}

// ── WSTG-BUSL: Business Logic ──

fn business_logic() -> MethodologySection {
    section(
        "WSTG-BUSL: Business Logic Testing",
        vec![
            activity(
                "Test Business Logic",
                "Identify flaws in business logic workflows",
                tasks(&[
                    (
                        "Test workflow bypass",
                        "Attempt to skip steps in multi-step processes",
                        "WSTG-BUSL-01",
                    ),
                    (
                        "Test negative value inputs",
                        "Submit negative quantities, prices, or values",
                        "WSTG-BUSL-01",
                    ),
                    (
                        "Test race conditions",
                        "Submit concurrent requests to exploit time-of-check time-of-use issues",
                        "WSTG-BUSL-01",
                    ),
                ]),
            ),
            activity(
                "Test Payment Manipulation",
                "Verify integrity of payment processing",
                tasks(&[
                    (
                        "Test price manipulation",
                        "Modify price parameters in requests",
                        "WSTG-BUSL-02",
                    ),
                    (
                        "Test currency manipulation",
                        "Attempt to exploit currency conversion logic",
                        "WSTG-BUSL-02",
                    ),
                    (
                        "Test quantity manipulation",
                        "Manipulate item quantities beyond business rules",
                        "WSTG-BUSL-02",
                    ),
                ]),
            ),
            activity(
                "Test Abuse of Functionality",
                "Identify misuse of legitimate features",
                tasks(&[
                    (
                        "Test for excessive requests",
                        "Send high volumes of requests to test rate limiting",
                        "WSTG-BUSL-03",
                    ),
                    (
                        "Test for resource exhaustion",
                        "Upload large files or request expensive operations repeatedly",
                        "WSTG-BUSL-03",
                    ),
                ]),
            ),
        ],
    )
}

// ── WSTG-CLIENT: Client-Side Testing ──

fn client_side() -> MethodologySection {
    section("WSTG-CLIENT: Client-Side Testing", vec![
        activity("Test DOM-Based XSS", "Check for client-side cross-site scripting", tasks(&[
            ("Test sources and sinks", "Identify DOM XSS via sources (URL, localStorage) and sinks (innerHTML, eval)", "WSTG-CLIENT-01"),
            ("Test URL fragments", "Check for DOM XSS via hash fragment manipulation", "WSTG-CLIENT-01"),
        ])),
        activity("Test JavaScript Execution", "Review client-side script security", tasks(&[
            ("Test for eval() usage", "Identify use of eval and Function constructor with user input", "WSTG-CLIENT-02"),
            ("Test for postMessage handlers", "Check cross-origin messaging for origin validation", "WSTG-CLIENT-02"),
        ])),
        activity("Test HTML5 Storage", "Verify secure use of client-side storage", tasks(&[
            ("Test localStorage for sensitive data", "Check if tokens or secrets are stored in localStorage", "WSTG-CLIENT-03"),
            ("Test sessionStorage exposure", "Verify session storage does not leak across tabs", "WSTG-CLIENT-03"),
        ])),
        activity("Test WebSockets", "Verify WebSocket communication security", tasks(&[
            ("Test for unauthenticated WebSocket access", "Connect to WebSocket endpoints without authentication", "WSTG-CLIENT-04"),
            ("Test WebSocket message validation", "Inject malicious payloads via WebSocket messages", "WSTG-CLIENT-04"),
        ])),
        activity("Test Web Messaging", "Verify cross-document messaging security", tasks(&[
            ("Test origin validation", "Send postMessage events with spoofed origins", "WSTG-CLIENT-05"),
            ("Test message data validation", "Send malformed messages to event handlers", "WSTG-CLIENT-05"),
        ])),
    ])
}

// ── WSTG-APIT: API Testing ──

fn api_testing() -> MethodologySection {
    section(
        "WSTG-APIT: API Testing",
        vec![
            activity(
                "Test API Authentication",
                "Verify API authentication mechanisms",
                tasks(&[
                    (
                        "Test for missing authentication",
                        "Access API endpoints without credentials",
                        "WSTG-APIT-01",
                    ),
                    (
                        "Test API key exposure",
                        "Look for hardcoded or leaked API keys",
                        "WSTG-APIT-01",
                    ),
                    (
                        "Test JWT vulnerabilities",
                        "Analyze JWT tokens for algorithm confusion and weak signing",
                        "WSTG-APIT-01",
                    ),
                ]),
            ),
            activity(
                "Test API Authorization",
                "Verify API access controls",
                tasks(&[
                    (
                        "Test horizontal access in APIs",
                        "Access other users' data via API by modifying resource IDs",
                        "WSTG-APIT-02",
                    ),
                    (
                        "Test mass assignment",
                        "Add unexpected fields to API requests to overwrite protected attributes",
                        "WSTG-APIT-02",
                    ),
                    (
                        "Test for excessive data exposure",
                        "Check if API responses contain more data than needed",
                        "WSTG-APIT-02",
                    ),
                ]),
            ),
            activity(
                "Test API Rate Limiting",
                "Verify API abuse prevention",
                tasks(&[
                    (
                        "Test for missing rate limiting",
                        "Send rapid requests to check for throttling",
                        "WSTG-APIT-03",
                    ),
                    (
                        "Test rate limit bypass",
                        "Attempt to bypass rate limits via header manipulation",
                        "WSTG-APIT-03",
                    ),
                ]),
            ),
            activity(
                "Test API Input Validation",
                "Check API parameter handling",
                tasks(&[
                    (
                        "Test API parameter fuzzing",
                        "Send unexpected data types and values to API parameters",
                        "WSTG-APIT-04",
                    ),
                    (
                        "Test API content-type handling",
                        "Send requests with unexpected Content-Type headers",
                        "WSTG-APIT-04",
                    ),
                ]),
            ),
        ],
    )
}
