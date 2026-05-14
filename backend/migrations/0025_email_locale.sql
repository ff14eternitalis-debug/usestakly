ALTER TABLE users
  ADD COLUMN IF NOT EXISTS email_locale TEXT NOT NULL DEFAULT 'en';

ALTER TABLE users
  DROP CONSTRAINT IF EXISTS users_email_locale_check;

ALTER TABLE users
  ADD CONSTRAINT users_email_locale_check CHECK (email_locale IN ('en', 'fr'));
