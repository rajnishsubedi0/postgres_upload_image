# Upload image to postgres and fetch image using actix-web 
* First of all create database named <i>testt</i> in postgres database.
* After that create <i>images</i> named table with following field using this command.
```
CREATE TABLE images (
    id SERIAL PRIMARY KEY,
    data BYTEA NOT NULL
);
```
* Then do ```cargo run``` and enter following command to upload image
```
curl -X POST -F "file=@/home/rkant/Downloads/Logo.png" http://127.0.0.1:8080/upload
```
Now image has been uploaded to the database

# Fetching image from postgres
* To fetch data from database enter following command
```
curl -X GET http://127.0.0.1:8080/upload --output output.jpg
```
or enter directly get image in browser by following command
```
http://127.0.0.1:8080/upload
```
