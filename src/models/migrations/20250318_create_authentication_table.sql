-- Migration: Create Authentication Table
-- Description: Adds a table to store authentication-related information
-- separate from the user's basic information.

-- Create the authentication table
CREATE TABLE IF NOT EXISTS authentications (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    password_hash VARCHAR(255) NOT NULL,
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    email_verification_token VARCHAR(100),
    email_verification_sent_at TIMESTAMP,
    
    -- Password reset functionality
    password_reset_token VARCHAR(100),
    password_reset_expires_at TIMESTAMP,
    password_reset_sent_at TIMESTAMP,
    
    -- Token revocation tracking
    refresh_token VARCHAR(255),
    refresh_token_expires_at TIMESTAMP,
    access_token_jti VARCHAR(100), -- JWT ID for the latest access token
    
    -- Login tracking
    last_login_at TIMESTAMP,
    last_login_ip VARCHAR(45), -- Supports IPv6 addresses
    last_failed_login_at TIMESTAMP,
    failed_login_attempts INTEGER DEFAULT 0,
    account_locked_until TIMESTAMP,
    
    -- Metadata
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Constraint to ensure one auth record per user
    CONSTRAINT uq_auth_user_id UNIQUE (user_id)
);

-- Create indexes for performance
CREATE INDEX idx_auth_user_id ON authentications(user_id);
CREATE INDEX idx_auth_email_verification_token ON authentications(email_verification_token) 
    WHERE email_verification_token IS NOT NULL;
CREATE INDEX idx_auth_password_reset_token ON authentications(password_reset_token) 
    WHERE password_reset_token IS NOT NULL;
CREATE INDEX idx_auth_refresh_token ON authentications(refresh_token) 
    WHERE refresh_token IS NOT NULL;

-- Add trigger to update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_authentication_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_authentication_timestamp
BEFORE UPDATE ON authentications
FOR EACH ROW
EXECUTE FUNCTION update_authentication_timestamp();

-- Add comment documentation
COMMENT ON TABLE authentications IS 'Stores user authentication information including password hashes, tokens, and login history';
COMMENT ON COLUMN authentications.password_hash IS 'Securely hashed user password';
COMMENT ON COLUMN authentications.email_verified IS 'Flag indicating if the user has verified their email address';
COMMENT ON COLUMN authentications.email_verification_token IS 'Token sent to users to verify their email address';
COMMENT ON COLUMN authentications.password_reset_token IS 'Token used for password reset functionality';
COMMENT ON COLUMN authentications.password_reset_expires_at IS 'Expiration time for the password reset token';
COMMENT ON COLUMN authentications.refresh_token IS 'JWT refresh token for generating new access tokens';
COMMENT ON COLUMN authentications.access_token_jti IS 'ID of the most recently issued access token for revocation checking';
COMMENT ON COLUMN authentications.last_login_at IS 'Timestamp of the user''s last successful login';
COMMENT ON COLUMN authentications.failed_login_attempts IS 'Count of failed login attempts since last successful login';
COMMENT ON COLUMN authentications.account_locked_until IS 'Timestamp until which the account is locked due to failed login attempts';

