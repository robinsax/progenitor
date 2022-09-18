DEFINE TABLE user;

DEFINE SCOPE account SESSION 24h
	SIGNUP ( CREATE user SET email = $email, pass = crypto::argon2::generate($pass) )
	SIGNIN ( SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass, $pass) )
;

CREATE user SET email = 'anyone@anywhere.tld', pass = 'notasecret';

DEFINE TABLE area;
DEFINE TABLE occurrence;
DEFINE TABLE comment;
