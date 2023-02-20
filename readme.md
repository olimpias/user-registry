# User Registry
Purpose of this project to play around Rust and experiment telemetry and tokio lib. The project uses DDD.

Data layer uses in-memory storage. However domain layer uses trait so it can easily be extended into different type of storage.

### API Endpoints

`POST http://{URL}/user`

`GET http://{URL}/user`

`GET http://{URL}/user/{userID}`

`DELETE http://{URL}/user/{userID}`