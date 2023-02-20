# User Registry
Purpose of this project to play around Rust and experiment telemetry and tokio lib. The project uses DDD.

Data layer uses in-memory storage. However domain layer uses trait so it can easily be extended into different type of storage.

You can configure the HTTP server port in `configuration.yaml`. Default port:8000

## API Endpoints

User Model:

```json
{"name":"{username-name}, "id":{UserID}}
```

### Add a User
You can use the below endpoint to add a user. Random generated UUID is being assigned during a user creation.

`POST http://{URL}/user`

**Request with curl:**

```shell
curl -X POST http://localhost:8000/user -H 'Content-Type: application/json' -d '{"name":"test"}'
```

**Response:**
```shell
{"name":"test","id":"bdbb4cc1-878b-4160-aea3-b5edc947e9f0"}
```

### Get Users

You can fetch the users using below endpoint.

`GET http://{URL}/user`

**Request to fetch users:**
```shell
curl http://localhost:8000/user'
```

**Response:**
```shell
{
   "users":[
      {
         "name":"test",
         "id":"bdbb4cc1-878b-4160-aea3-b5edc947e9f0"
      },
      {
         "name":"test1",
         "id":"f83a7ebd-ab94-404b-8e37-efea0665980c"
      }
   ]
}
```

## Get a user by ID
You can fetch a user by ID using below endpoint.

`GET http://{URL}/user/{userID}`

**Request to fetch a user by ID:**
```shell
curl http://localhost:8000/user/bdbb4cc1-878b-4160-aea3-b5edc947e9f0
```

**Response**
```shell
{"name":"test","id":"bdbb4cc1-878b-4160-aea3-b5edc947e9f0"}
```
## Delete a user by ID
You can delete a user by ID using below endpoint.

`DELETE http://{URL}/user/{userID}`

**Request to delete a user by ID:**
```shell
curl -X DELETE 'http://localhost:8000/user/bdbb4cc1-878b-4160-aea3-b5edc947e9f0'
```