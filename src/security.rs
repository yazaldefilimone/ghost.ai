pub const SKIP_SECURITY_PATTERNS: &[&str] = &[
	"login", "signin", "auth", "verify", "otp", "2fa", "token", "secure", "password", "account",
	"admin", "settings", "private", "bank", "wallet", "crypto", "pay", "checkout", "payment",
	"invoice", "billing", "credit", "debit", "cvc", "cvv", "stripe", "paypal", "apple", "google",
	"pix", "transfer", "session", "profile",
];

#[inline(always)]
pub fn contains_skip_security_pattern(text: &str) -> bool {
	SKIP_SECURITY_PATTERNS.iter().any(|pattern| text.to_lowercase().contains(pattern))
}
