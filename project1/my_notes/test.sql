-- sample of Rust code for Postgres store procedure
-- https://github.com/clia/pgxr
-- sample of test SQL:
-- feel free to use your names and path
set rust.file_path = '/home/vk/git/rust_test/data';
CREATE SCHEMA IF NOT EXISTS rust1;
DROP TABLE IF EXISTS rust1.test1;
CREATE TABLE rust1.test1 (id bigserial, version serial, data text, primary key (id, version));
-- easiest way to create SO compiled function is use postgres super user
\c test_db postgres
DROP FUNCTION IF EXISTS rust1.testf();
CREATE OR REPLACE FUNCTION rust1.testf()
RETURNS trigger AS '/home/vk/git/rust_test/postgres/rust.so'
LANGUAGE C strict;
insert into rust1.test1 (data) values ('test-test');
\! cat /home/vk/git/rust_test/data/a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11
-- expected file content:
-- id=1
-- version=1
-- dGVzdC10ZXN0
--