ls $(pg_config --includedir-server)
должно выдавать список файлов, если нет то, 
sudo apt install postgresql-server-dev-12

bindgen wrapper.h -o src/wrapper12.rs -- -I /usr/include/postgresql/12/server/
