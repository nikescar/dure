package main

import (
	"crypto/ed25519"
	"encoding/json"
	"fmt"
	"sync"

	"github.com/go-webauthn/webauthn/protocol"
	"github.com/go-webauthn/webauthn/webauthn"
	"github.com/google/uuid"
	"golang.org/x/crypto/chacha20poly1305"
)

type WebAuthnImpl struct {
	webauthn *webauthn.WebAuthn
	users    *UserStore
	sessions *SessionStore
}

type UserStore struct {
	mu          sync.RWMutex
	nameToID    map[string]string
	users       map[string]*WebAuthnUser
	credentials map[string][]webauthn.Credential
}

type SessionStore struct {
	mu           sync.RWMutex
	regSessions  map[string]*RegistrationSession
	authSessions map[string]*AuthenticationSession
}

// RegistrationSession stores state for ongoing registration
type RegistrationSession struct {
	UserID      string
	Username    string
	SessionData *webauthn.SessionData
}

// AuthenticationSession stores state for ongoing authentication
type AuthenticationSession struct {
	UserID      string
	SessionData *webauthn.SessionData
}

type WebAuthnUser struct {
	id          []byte
	name        string
	displayName string
}

func (u *WebAuthnUser) WebAuthnID() []byte {
	return u.id
}

func (u *WebAuthnUser) WebAuthnName() string {
	return u.name
}

func (u *WebAuthnUser) WebAuthnDisplayName() string {
	return u.displayName
}

func (u *WebAuthnUser) WebAuthnIcon() string {
	return ""
}

func (u *WebAuthnUser) WebAuthnCredentials() []webauthn.Credential {
	return nil
}

func NewWebAuthnBridge(rpID, rpOrigin, rpName string) (*WebAuthnImpl, error) {
	wconfig := &webauthn.Config{
		RPDisplayName: rpName,
		RPID:          rpID,
		RPOrigins:     []string{rpOrigin},
	}

	w, err := webauthn.New(wconfig)
	if err != nil {
		return nil, fmt.Errorf("failed to create WebAuthn: %w", err)
	}

	return &WebAuthnImpl{
		webauthn: w,
		users: &UserStore{
			nameToID:    make(map[string]string),
			users:       make(map[string]*WebAuthnUser),
			credentials: make(map[string][]webauthn.Credential),
		},
		sessions: &SessionStore{
			regSessions:  make(map[string]*RegistrationSession),
			authSessions: make(map[string]*AuthenticationSession),
		},
	}, nil
}

func (impl *WebAuthnImpl) signup_begin(req SignupBeginRequest) SignupBeginResponse {
	sessionID := uuid.New().String()

	impl.users.mu.Lock()
	userID, exists := impl.users.nameToID[req.username]
	if !exists {
		userID = uuid.New().String()
		impl.users.nameToID[req.username] = userID
	}
	impl.users.mu.Unlock()

	user := &WebAuthnUser{
		id:          []byte(userID),
		name:        req.username,
		displayName: req.display_name,
	}

	// Prepare registration options based on scenario
	var opts []webauthn.RegistrationOption

	// Get existing credentials for exclusion
	impl.users.mu.RLock()
	existingCreds := impl.users.credentials[userID]
	impl.users.mu.RUnlock()

	if len(existingCreds) > 0 {
		// Exclude existing credentials to prevent re-registration
		credDescriptors := make([]protocol.CredentialDescriptor, len(existingCreds))
		for i, cred := range existingCreds {
			credDescriptors[i] = protocol.CredentialDescriptor{
				Type:         protocol.PublicKeyCredentialType,
				CredentialID: cred.ID,
			}
		}
		opts = append(opts, webauthn.WithExclusions(credDescriptors))
	}

	// Configure resident key and user verification based on scenario
	switch req.scenario {
	case "usernameless":
		// Usernameless requires discoverable credentials
		opts = append(opts,
			webauthn.WithResidentKeyRequirement(protocol.ResidentKeyRequirementRequired),
			webauthn.WithAuthenticatorSelection(protocol.AuthenticatorSelection{
				RequireResidentKey: protocol.ResidentKeyRequired(),
				UserVerification:   protocol.VerificationRequired,
			}),
		)
	case "passwordless":
		// Passwordless prefers discoverable credentials with user verification
		opts = append(opts,
			webauthn.WithResidentKeyRequirement(protocol.ResidentKeyRequirementPreferred),
			webauthn.WithAuthenticatorSelection(protocol.AuthenticatorSelection{
				UserVerification: protocol.VerificationRequired,
			}),
		)
	case "mfa":
		// MFA doesn't require resident key or user verification
		opts = append(opts,
			webauthn.WithAuthenticatorSelection(protocol.AuthenticatorSelection{
				UserVerification: protocol.VerificationDiscouraged,
			}),
		)
	default:
		// Default to passwordless-like behavior
		opts = append(opts,
			webauthn.WithResidentKeyRequirement(protocol.ResidentKeyRequirementPreferred),
		)
	}

	// Add credProps extension to check if credential is discoverable
	opts = append(opts, webauthn.WithExtensions(map[string]any{"credProps": true}))

	// Use BeginMediatedRegistration (recommended by doc.go)
	creation, sessionData, err := impl.webauthn.BeginMediatedRegistration(user, protocol.MediationDefault, opts...)
	if err != nil {
		return SignupBeginResponse{
			success: false,
			error:   fmt.Sprintf("failed to begin registration: %v", err),
		}
	}

	impl.sessions.mu.Lock()
	impl.sessions.regSessions[sessionID] = &RegistrationSession{
		UserID:      userID,
		Username:    req.username,
		SessionData: sessionData,
	}
	impl.sessions.mu.Unlock()

	impl.users.mu.Lock()
	impl.users.users[userID] = user
	impl.users.mu.Unlock()

	challengeJSON, err := json.Marshal(creation)
	if err != nil {
		return SignupBeginResponse{
			success: false,
			error:   fmt.Sprintf("failed to marshal challenge: %v", err),
		}
	}

	return SignupBeginResponse{
		session_id:     sessionID,
		challenge_json: string(challengeJSON),
		success:        true,
		error:          "",
	}
}

func (impl *WebAuthnImpl) signup_finish(req SignupFinishRequest) SignupFinishResponse {
	impl.sessions.mu.Lock()
	session, exists := impl.sessions.regSessions[req.session_id]
	if !exists {
		impl.sessions.mu.Unlock()
		return SignupFinishResponse{
			success: false,
			error:   "invalid session",
		}
	}
	// Remove session immediately (one-time use)
	delete(impl.sessions.regSessions, req.session_id)
	impl.sessions.mu.Unlock()

	// Parse the credential from JSON
	var credResponse protocol.CredentialCreationResponse
	err := json.Unmarshal([]byte(req.credential_json), &credResponse)
	if err != nil {
		return SignupFinishResponse{
			success: false,
			error:   fmt.Sprintf("failed to parse credential: %v", err),
		}
	}

	// Parse the credential creation data
	parsedResponse, err := credResponse.Parse()
	if err != nil {
		return SignupFinishResponse{
			success: false,
			error:   fmt.Sprintf("failed to parse credential data: %v", err),
		}
	}

	// Get the user
	impl.users.mu.RLock()
	user, exists := impl.users.users[session.UserID]
	impl.users.mu.RUnlock()

	if !exists {
		return SignupFinishResponse{
			success: false,
			error:   "user not found",
		}
	}

	// Verify and create credential
	credential, err := impl.webauthn.CreateCredential(user, *session.SessionData, parsedResponse)
	if err != nil {
		return SignupFinishResponse{
			success: false,
			error:   fmt.Sprintf("failed to create credential: %v", err),
		}
	}

	// Store credential
	impl.users.mu.Lock()
	impl.users.credentials[session.UserID] = append(
		impl.users.credentials[session.UserID],
		*credential,
	)
	impl.users.mu.Unlock()

	return SignupFinishResponse{
		success: true,
		user_id: session.UserID,
		error:   "",
	}
}

func (impl *WebAuthnImpl) signin_begin(req SigninBeginRequest) SigninBeginResponse {
	sessionID := uuid.New().String()

	if req.scenario == "usernameless" {
		assertion, sessionData, err := impl.webauthn.BeginDiscoverableLogin()
		if err != nil {
			return SigninBeginResponse{
				success: false,
				error:   fmt.Sprintf("failed to begin discoverable login: %v", err),
			}
		}

		impl.sessions.mu.Lock()
		impl.sessions.authSessions[sessionID] = &AuthenticationSession{
			UserID:      "", // Will be discovered from credential
			SessionData: sessionData,
		}
		impl.sessions.mu.Unlock()

		challengeJSON, _ := json.Marshal(assertion)
		return SigninBeginResponse{
			session_id:     sessionID,
			challenge_json: string(challengeJSON),
			success:        true,
			error:          "",
		}
	}

	impl.users.mu.RLock()
	userID, exists := impl.users.nameToID[req.username]
	if !exists {
		impl.users.mu.RUnlock()
		return SigninBeginResponse{
			success: false,
			error:   "user not found",
		}
	}

	user := impl.users.users[userID]
	credentials := impl.users.credentials[userID]
	impl.users.mu.RUnlock()

	if len(credentials) == 0 {
		return SigninBeginResponse{
			success: false,
			error:   "user has no credentials",
		}
	}

	// Create a temporary user that returns the stored credentials
	userWithCreds := &userWithCredentials{
		WebAuthnUser: user,
		credentials:  credentials,
	}

	assertion, sessionData, err := impl.webauthn.BeginLogin(userWithCreds)
	if err != nil {
		return SigninBeginResponse{
			success: false,
			error:   fmt.Sprintf("failed to begin login: %v", err),
		}
	}

	impl.sessions.mu.Lock()
	impl.sessions.authSessions[sessionID] = &AuthenticationSession{
		UserID:      userID,
		SessionData: sessionData,
	}
	impl.sessions.mu.Unlock()

	challengeJSON, _ := json.Marshal(assertion)
	return SigninBeginResponse{
		session_id:     sessionID,
		challenge_json: string(challengeJSON),
		success:        true,
		error:          "",
	}
}

func (impl *WebAuthnImpl) signin_finish(req SigninFinishRequest) SigninFinishResponse {
	impl.sessions.mu.Lock()
	session, exists := impl.sessions.authSessions[req.session_id]
	if !exists {
		impl.sessions.mu.Unlock()
		return SigninFinishResponse{
			success: false,
			error:   "invalid session",
		}
	}
	// Remove session immediately (one-time use)
	delete(impl.sessions.authSessions, req.session_id)
	impl.sessions.mu.Unlock()

	// Parse the credential from JSON
	var credResponse protocol.CredentialAssertionResponse
	err := json.Unmarshal([]byte(req.credential_json), &credResponse)
	if err != nil {
		return SigninFinishResponse{
			success: false,
			error:   fmt.Sprintf("failed to parse credential: %v", err),
		}
	}

	// Parse the credential assertion data
	parsedResponse, err := credResponse.Parse()
	if err != nil {
		return SigninFinishResponse{
			success: false,
			error:   fmt.Sprintf("failed to parse credential data: %v", err),
		}
	}

	var userID string
	var credentials []webauthn.Credential

	// For usernameless, discover user from credential
	if session.UserID == "" {
		// Find user by credential ID
		impl.users.mu.RLock()
		found := false
		for uid, creds := range impl.users.credentials {
			for _, cred := range creds {
				if string(cred.ID) == string(parsedResponse.Response.UserHandle) {
					userID = uid
					credentials = creds
					found = true
					break
				}
			}
			if found {
				break
			}
		}
		impl.users.mu.RUnlock()

		if !found {
			return SigninFinishResponse{
				success: false,
				error:   "credential not found",
			}
		}
	} else {
		userID = session.UserID
		impl.users.mu.RLock()
		credentials = impl.users.credentials[userID]
		impl.users.mu.RUnlock()
	}

	// Get user
	impl.users.mu.RLock()
	user := impl.users.users[userID]
	impl.users.mu.RUnlock()

	// Create user with credentials for validation
	userWithCreds := &userWithCredentials{
		WebAuthnUser: user,
		credentials:  credentials,
	}

	// Verify the credential
	credential, err := impl.webauthn.ValidateLogin(userWithCreds, *session.SessionData, parsedResponse)
	if err != nil {
		return SigninFinishResponse{
			success: false,
			error:   fmt.Sprintf("authentication failed: %v", err),
		}
	}

	// Update credential (counter, etc.)
	impl.users.mu.Lock()
	for i, cred := range impl.users.credentials[userID] {
		if string(cred.ID) == string(credential.ID) {
			impl.users.credentials[userID][i] = *credential
			break
		}
	}
	impl.users.mu.Unlock()

	// Generate session token
	sessionToken := uuid.New().String()

	return SigninFinishResponse{
		success:       true,
		user_id:       userID,
		session_token: sessionToken,
		error:         "",
	}
}

// userWithCredentials wraps WebAuthnUser and provides credentials
type userWithCredentials struct {
	*WebAuthnUser
	credentials []webauthn.Credential
}

func (u *userWithCredentials) WebAuthnCredentials() []webauthn.Credential {
	return u.credentials
}

// ============================================================================
// Passkey Login Ceremony (Discoverable Credentials)
// ============================================================================

func (impl *WebAuthnImpl) passkey_login_begin(req PasskeyLoginBeginRequest) PasskeyLoginBeginResponse {
	sessionID := uuid.New().String()

	// Parse mediation mode
	var mediation protocol.CredentialMediationRequirement
	switch req.mediation {
	case "silent":
		mediation = protocol.MediationSilent
	case "optional":
		mediation = protocol.MediationOptional
	case "conditional":
		mediation = protocol.MediationConditional
	case "required":
		mediation = protocol.MediationRequired
	default:
		mediation = protocol.MediationDefault
	}

	// Begin discoverable login (passkey)
	assertion, sessionData, err := impl.webauthn.BeginDiscoverableMediatedLogin(mediation)
	if err != nil {
		return PasskeyLoginBeginResponse{
			success: false,
			error:   fmt.Sprintf("failed to begin passkey login: %v", err),
		}
	}

	// Store session
	impl.sessions.mu.Lock()
	impl.sessions.authSessions[sessionID] = &AuthenticationSession{
		UserID:      "", // Will be discovered from credential
		SessionData: sessionData,
	}
	impl.sessions.mu.Unlock()

	// Marshal challenge to JSON
	challengeJSON, err := json.Marshal(assertion)
	if err != nil {
		return PasskeyLoginBeginResponse{
			success: false,
			error:   fmt.Sprintf("failed to marshal challenge: %v", err),
		}
	}

	return PasskeyLoginBeginResponse{
		session_id:     sessionID,
		challenge_json: string(challengeJSON),
		success:        true,
		error:          "",
	}
}

func (impl *WebAuthnImpl) passkey_login_finish(req PasskeyLoginFinishRequest) PasskeyLoginFinishResponse {
	// Retrieve and remove session
	impl.sessions.mu.Lock()
	session, exists := impl.sessions.authSessions[req.session_id]
	if !exists {
		impl.sessions.mu.Unlock()
		return PasskeyLoginFinishResponse{
			success: false,
			error:   "invalid session",
		}
	}
	delete(impl.sessions.authSessions, req.session_id)
	impl.sessions.mu.Unlock()

	// Parse credential response
	var credResponse protocol.CredentialAssertionResponse
	err := json.Unmarshal([]byte(req.credential_json), &credResponse)
	if err != nil {
		return PasskeyLoginFinishResponse{
			success: false,
			error:   fmt.Sprintf("failed to parse credential: %v", err),
		}
	}

	// Parse credential assertion data
	parsedResponse, err := credResponse.Parse()
	if err != nil {
		return PasskeyLoginFinishResponse{
			success: false,
			error:   fmt.Sprintf("failed to parse credential data: %v", err),
		}
	}

	// Loader function to find user by credential
	loadUser := func(rawID []byte, userHandle []byte) (user webauthn.User, err error) {
		impl.users.mu.RLock()
		defer impl.users.mu.RUnlock()

		// Find user by user handle
		for uid, u := range impl.users.users {
			if string(u.id) == string(userHandle) {
				creds := impl.users.credentials[uid]
				return &userWithCredentials{
					WebAuthnUser: u,
					credentials:  creds,
				}, nil
			}
		}

		return nil, fmt.Errorf("user not found for handle")
	}

	// Validate passkey login using ValidatePasskeyLogin (lower-level function)
	validatedUser, validatedCredential, err := impl.webauthn.ValidatePasskeyLogin(loadUser, *session.SessionData, parsedResponse)
	if err != nil {
		return PasskeyLoginFinishResponse{
			success: false,
			error:   fmt.Sprintf("passkey login failed: %v", err),
		}
	}

	// Extract user info
	userID := string(validatedUser.WebAuthnID())
	username := validatedUser.WebAuthnName()

	// Update credential (counter, etc.)
	impl.users.mu.Lock()
	for i, cred := range impl.users.credentials[userID] {
		if string(cred.ID) == string(validatedCredential.ID) {
			impl.users.credentials[userID][i] = *validatedCredential
			break
		}
	}
	impl.users.mu.Unlock()

	// Generate session token
	sessionToken := uuid.New().String()

	return PasskeyLoginFinishResponse{
		success:       true,
		user_id:       userID,
		username:      username,
		session_token: sessionToken,
		error:         "",
	}
}

// ============================================================================
// Multi-Factor Login Ceremony
// ============================================================================

func (impl *WebAuthnImpl) mfa_login_begin(req MfaLoginBeginRequest) MfaLoginBeginResponse {
	sessionID := uuid.New().String()

	// Get user by username
	impl.users.mu.RLock()
	userID, exists := impl.users.nameToID[req.username]
	if !exists {
		impl.users.mu.RUnlock()
		return MfaLoginBeginResponse{
			success: false,
			error:   "user not found",
		}
	}

	user := impl.users.users[userID]
	credentials := impl.users.credentials[userID]
	impl.users.mu.RUnlock()

	if len(credentials) == 0 {
		return MfaLoginBeginResponse{
			success: false,
			error:   "user has no credentials",
		}
	}

	// Create user with credentials
	userWithCreds := &userWithCredentials{
		WebAuthnUser: user,
		credentials:  credentials,
	}

	// Parse mediation mode
	var mediation protocol.CredentialMediationRequirement
	switch req.mediation {
	case "silent":
		mediation = protocol.MediationSilent
	case "optional":
		mediation = protocol.MediationOptional
	case "conditional":
		mediation = protocol.MediationConditional
	case "required":
		mediation = protocol.MediationRequired
	default:
		mediation = protocol.MediationDefault
	}

	// Begin mediated login (multi-factor)
	assertion, sessionData, err := impl.webauthn.BeginMediatedLogin(userWithCreds, mediation)
	if err != nil {
		return MfaLoginBeginResponse{
			success: false,
			error:   fmt.Sprintf("failed to begin MFA login: %v", err),
		}
	}

	// Store session
	impl.sessions.mu.Lock()
	impl.sessions.authSessions[sessionID] = &AuthenticationSession{
		UserID:      userID,
		SessionData: sessionData,
	}
	impl.sessions.mu.Unlock()

	// Marshal challenge to JSON
	challengeJSON, err := json.Marshal(assertion)
	if err != nil {
		return MfaLoginBeginResponse{
			success: false,
			error:   fmt.Sprintf("failed to marshal challenge: %v", err),
		}
	}

	return MfaLoginBeginResponse{
		session_id:     sessionID,
		challenge_json: string(challengeJSON),
		success:        true,
		error:          "",
	}
}

func (impl *WebAuthnImpl) mfa_login_finish(req MfaLoginFinishRequest) MfaLoginFinishResponse {
	// Retrieve and remove session
	impl.sessions.mu.Lock()
	session, exists := impl.sessions.authSessions[req.session_id]
	if !exists {
		impl.sessions.mu.Unlock()
		return MfaLoginFinishResponse{
			success: false,
			error:   "invalid session",
		}
	}
	delete(impl.sessions.authSessions, req.session_id)
	impl.sessions.mu.Unlock()

	// Parse credential response
	var credResponse protocol.CredentialAssertionResponse
	err := json.Unmarshal([]byte(req.credential_json), &credResponse)
	if err != nil {
		return MfaLoginFinishResponse{
			success: false,
			error:   fmt.Sprintf("failed to parse credential: %v", err),
		}
	}

	// Parse credential assertion data
	parsedResponse, err := credResponse.Parse()
	if err != nil {
		return MfaLoginFinishResponse{
			success: false,
			error:   fmt.Sprintf("failed to parse credential data: %v", err),
		}
	}

	// Get user
	impl.users.mu.RLock()
	user := impl.users.users[session.UserID]
	credentials := impl.users.credentials[session.UserID]
	impl.users.mu.RUnlock()

	// Create user with credentials
	userWithCreds := &userWithCredentials{
		WebAuthnUser: user,
		credentials:  credentials,
	}

	// Validate login (multi-factor) using ValidateLogin (lower-level function)
	validatedCredential, err := impl.webauthn.ValidateLogin(userWithCreds, *session.SessionData, parsedResponse)
	if err != nil {
		return MfaLoginFinishResponse{
			success: false,
			error:   fmt.Sprintf("MFA login failed: %v", err),
		}
	}

	// Update credential (counter, etc.)
	impl.users.mu.Lock()
	for i, cred := range impl.users.credentials[session.UserID] {
		if string(cred.ID) == string(validatedCredential.ID) {
			impl.users.credentials[session.UserID][i] = *validatedCredential
			break
		}
	}
	impl.users.mu.Unlock()

	// Generate session token
	sessionToken := uuid.New().String()

	return MfaLoginFinishResponse{
		success:       true,
		user_id:       session.UserID,
		session_token: sessionToken,
		error:         "",
	}
}

func init() {
	impl, err := NewWebAuthnBridge("localhost", "https://localhost:8443", "Dure")
	if err != nil {
		panic(fmt.Sprintf("failed to initialize WebAuthn: %v", err))
	}
	WebAuthnBridgeImpl = impl

	// Initialize CryptoBridge
	CryptoBridgeImpl = &CryptoImpl{}
}

// ============================================================================
// Crypto Bridge Implementation
// ============================================================================

type CryptoImpl struct{}

// ─── ChaCha20-Poly1305 Implementation ───────────────────────────────────────

func (c *CryptoImpl) chacha20poly1305_encrypt(req *ChaCha20Poly1305EncryptRequest) ChaCha20Poly1305EncryptResponse {
	// Import the chacha20poly1305 package from golang.org/x/crypto
	aead, err := chacha20poly1305.NewX(req.key)
	if err != nil {
		return ChaCha20Poly1305EncryptResponse{
			success: false,
			error:   fmt.Sprintf("failed to create AEAD: %v", err),
		}
	}

	// Verify nonce size
	if len(req.nonce) != aead.NonceSize() {
		return ChaCha20Poly1305EncryptResponse{
			success: false,
			error:   fmt.Sprintf("invalid nonce size: expected %d, got %d", aead.NonceSize(), len(req.nonce)),
		}
	}

	// Encrypt the plaintext
	ciphertext := aead.Seal(nil, req.nonce, req.plaintext, req.additional_data)

	return ChaCha20Poly1305EncryptResponse{
		ciphertext: ciphertext,
		success:    true,
		error:      "",
	}
}

func (c *CryptoImpl) chacha20poly1305_decrypt(req *ChaCha20Poly1305DecryptRequest) ChaCha20Poly1305DecryptResponse {
	// Import the chacha20poly1305 package from golang.org/x/crypto
	aead, err := chacha20poly1305.NewX(req.key)
	if err != nil {
		return ChaCha20Poly1305DecryptResponse{
			success: false,
			error:   fmt.Sprintf("failed to create AEAD: %v", err),
		}
	}

	// Verify nonce size
	if len(req.nonce) != aead.NonceSize() {
		return ChaCha20Poly1305DecryptResponse{
			success: false,
			error:   fmt.Sprintf("invalid nonce size: expected %d, got %d", aead.NonceSize(), len(req.nonce)),
		}
	}

	// Decrypt the ciphertext
	plaintext, err := aead.Open(nil, req.nonce, req.ciphertext, req.additional_data)
	if err != nil {
		return ChaCha20Poly1305DecryptResponse{
			success: false,
			error:   fmt.Sprintf("decryption failed: %v", err),
		}
	}

	return ChaCha20Poly1305DecryptResponse{
		plaintext: plaintext,
		success:   true,
		error:     "",
	}
}

// ─── Ed25519 Implementation ─────────────────────────────────────────────────

// Ed25519 key generation
// Returns a new Ed25519 public/private key pair.
// Public key: 32 bytes, Private key: 64 bytes
func (c *CryptoImpl) ed25519_generate_key(req *Ed25519GenerateKeyRequest) Ed25519GenerateKeyResponse {
	// Use crypto/rand for randomness
	publicKey, privateKey, err := ed25519.GenerateKey(nil)
	if err != nil {
		return Ed25519GenerateKeyResponse{
			success: false,
			error:   fmt.Sprintf("key generation failed: %v", err),
		}
	}

	return Ed25519GenerateKeyResponse{
		public_key:  publicKey,
		private_key: privateKey,
		success:     true,
		error:       "",
	}
}

// Ed25519 message signing
// Signs a message with the given private key.
//
// IMPORTANT: We make a defensive copy of the private key from FFI memory.
// Go 1.25's FIPS cache uses weak pointers to track Ed25519 keys for optimization.
// Weak pointers only work with Go-managed memory, not Rust FFI memory, causing
// "fatal error: getWeakHandle on invalid pointer" crashes.
//
// The copy overhead is minimal (~64 bytes) and ensures reliability.
func (c *CryptoImpl) ed25519_sign(req *Ed25519SignRequest) Ed25519SignResponse {
	// Verify private key size
	if len(req.private_key) != ed25519.PrivateKeySize {
		return Ed25519SignResponse{
			success: false,
			error:   fmt.Sprintf("invalid private key size: expected %d, got %d", ed25519.PrivateKeySize, len(req.private_key)),
		}
	}

	// Make a defensive copy of the private key from FFI memory
	// This avoids potential issues with weak pointers in Go 1.25 FIPS cache
	privateKeyCopy := make(ed25519.PrivateKey, len(req.private_key))
	copy(privateKeyCopy, req.private_key)

	// Sign the message
	signature := ed25519.Sign(privateKeyCopy, req.message)

	return Ed25519SignResponse{
		signature: signature,
		success:   true,
		error:     "",
	}
}

// Ed25519 signature verification
// Verifies an Ed25519 signature for the given message and public key.
func (c *CryptoImpl) ed25519_verify(req *Ed25519VerifyRequest) Ed25519VerifyResponse {
	// Verify public key size
	if len(req.public_key) != ed25519.PublicKeySize {
		return Ed25519VerifyResponse{
			success: false,
			error:   fmt.Sprintf("invalid public key size: expected %d, got %d", ed25519.PublicKeySize, len(req.public_key)),
		}
	}

	// Verify signature size
	if len(req.signature) != ed25519.SignatureSize {
		return Ed25519VerifyResponse{
			success: false,
			error:   fmt.Sprintf("invalid signature size: expected %d, got %d", ed25519.SignatureSize, len(req.signature)),
		}
	}

	// Verify the signature
	valid := ed25519.Verify(req.public_key, req.message, req.signature)

	return Ed25519VerifyResponse{
		valid:   valid,
		success: true,
		error:   "",
	}
}
