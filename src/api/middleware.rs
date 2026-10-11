use axum::{
    extract::Request,
    http::{header, HeaderName, HeaderValue},
    middleware::Next,
    response::Response,
};

/// Injects OWASP recommended secure headers into all outgoing HTTP responses:
/// - Content-Security-Policy (CSP)
/// - X-Content-Type-Options: nosniff
/// - X-Frame-Options: DENY
/// - Strict-Transport-Security (HSTS)
/// - Referrer-Policy: strict-origin-when-cross-origin
/// - Permissions-Policy
/// - Cross-Origin-Opener-Policy (COOP)
/// - Cross-Origin-Resource-Policy (CORP)
/// - X-Permitted-Cross-Domain-Policies: none
pub async fn owasp_security_headers(req: Request, next: Next) -> Response {
    let path = req.uri().path().to_string();
    let is_docs_route = path.starts_with("/docs");

    let mut response = next.run(req).await;
    let headers = response.headers_mut();

    // 1. Content-Security-Policy (CSP)
    // Both docs and app now execute scripts strictly from 'self' with ZERO 'unsafe-inline' scripts.
    // The docs route permits 'unsafe-inline' solely for style-src (CSS custom properties and themes injected by Scalar).
    let csp_val = if is_docs_route {
        "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; font-src 'self' data:; img-src 'self' data: https:; connect-src 'self'; frame-ancestors 'self'; form-action 'self'; base-uri 'self';"
    } else {
        "default-src 'self'; script-src 'self'; style-src 'self'; font-src 'self' data:; img-src 'self' data:; connect-src 'self'; frame-ancestors 'none'; form-action 'self'; base-uri 'self';"
    };

    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(csp_val),
    );

    // 2. X-Content-Type-Options: Prevents MIME type sniffing
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );

    // 3. X-Frame-Options: Clickjacking mitigation (defense-in-depth with CSP frame-ancestors)
    headers.insert(
        header::X_FRAME_OPTIONS,
        HeaderValue::from_static("DENY"),
    );

    // 4. Strict-Transport-Security (HSTS): Enforces HTTPS connections
    headers.insert(
        header::STRICT_TRANSPORT_SECURITY,
        HeaderValue::from_static("max-age=31536000; includeSubDomains"),
    );

    // 5. Referrer-Policy: Protects referrer privacy
    headers.insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );

    // 6. Permissions-Policy: Disables unneeded privileged browser APIs
    headers.insert(
        HeaderName::from_static("permissions-policy"),
        HeaderValue::from_static("camera=(), microphone=(), geolocation=(), payment=()"),
    );

    // 7. Cross-Origin-Opener-Policy (COOP): Isolates browsing context against cross-origin attacks
    headers.insert(
        HeaderName::from_static("cross-origin-opener-policy"),
        HeaderValue::from_static("same-origin"),
    );

    // 8. Cross-Origin-Resource-Policy (CORP): Prevents other domains from reading resources
    headers.insert(
        HeaderName::from_static("cross-origin-resource-policy"),
        HeaderValue::from_static("same-origin"),
    );

    // 9. X-Permitted-Cross-Domain-Policies: Restricts Flash/PDF cross-domain policy files
    headers.insert(
        HeaderName::from_static("x-permitted-cross-domain-policies"),
        HeaderValue::from_static("none"),
    );

    response
}

