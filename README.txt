Build (Linux):

```
  cargo build --release
```

Build (MacOS):

```
  cargo rustc --release -- -C link-arg=-undefined -C link-arg=dynamic_lookup
```

Postgresql (MacOS):

```
CREATE TABLE table1 (
  id bigserial,
  version serial,
  my_text text,
  my_jsonb jsonb,
  my_blob bytea,
  primary key (id, version)
);
```

replace 'libpgsql_ext.dylib' and 'my_rust_func' with your own:

```
DROP FUNCTION IF EXISTS my_rust_func cascade;

CREATE OR REPLACE FUNCTION my_rust_func() RETURNS trigger
AS '/[....]/my_project/target/release/libpgsql_ext.dylib', 'my_rust_func'
LANGUAGE C strict;

CREATE TRIGGER my_rust_func__trg
  BEFORE INSERT OR UPDATE ON table1
  FOR EACH ROW EXECUTE PROCEDURE my_rust_func();


INSERT INTO table1 (my_text, my_json, my_blob) VALUES ('some text', '{"key1": "value1"}', pg_read_binary_file('/[....]/Pictures/pic1.jpg')::bytea);
```

Logs (MacOS):

```
tail -n 150 /usr/local/var/log/postgresql@11.log
```