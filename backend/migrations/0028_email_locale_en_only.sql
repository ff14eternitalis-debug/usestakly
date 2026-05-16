UPDATE users
SET email_locale = 'en'
WHERE email_locale = 'fr';

ALTER TABLE users
  DROP CONSTRAINT IF EXISTS users_email_locale_check;

ALTER TABLE users
  ADD CONSTRAINT users_email_locale_check CHECK (email_locale = 'en');
