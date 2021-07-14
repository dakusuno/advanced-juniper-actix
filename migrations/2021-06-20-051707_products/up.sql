-- Your SQL goes here
CREATE TABLE products(
   id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
   user_id UUID NOT NULL REFERENCES users (id),
   name VARCHAR(255) NOT NULL,
   price FLOAT8 NOT NULL

)