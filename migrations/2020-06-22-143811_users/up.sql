CREATE TYPE trust_type AS ENUM ('public', 'authenticated', 'vouched', 'ndaed', 'staff');

CREATE TABLE profiles (
    uuid UUID PRIMARY KEY,
    user_id VARCHAR UNIQUE UNIQUE NOT NULL,
    primary_email VARCHAR UNIQUE NOT NULL,
    primary_username VARCHAR UNIQUE NOT NULL,
    active BOOLEAN NOT NULL,
    trust trust_type NOT NULL,
    version INTEGER NOT NULL,
    profile JSONB NOT NULL
);
