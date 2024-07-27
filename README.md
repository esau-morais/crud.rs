# rust_crud

learning rust by building a simple crud

## next steps

- [x] improve types
- [x] create auth middleware
- [ ] integration tests
- [ ] front-end (astro)
- [ ] deploy

## requests

### posts

#### get all posts

```sh
curl -X GET http://localhost:8080/v0/posts | json_pp
```

#### get single post by ID

```sh
curl -X GET http://localhost:8080/v0/posts/{id} | json_pp
```

#### create post

```sh
curl -X POST http://localhost:8080/v0/posts \
-H 'Content-Type: application/json' \
-d '{
    "title": "Any Title",
    "body": "any body",
    "published": false.
    "user_id": 1
}'
```

#### update post

```sh
curl -X PUT http://localhost:8080/v0/posts/{id} \
-H 'Content-Type: application/json' \
-d '{
    "title":"Any Title (Edited)",
    "body": "any body (edited)",
    "published": true
}'
```

### delete post

```sh
curl -X DELETE http://localhost:8080/v0/posts/{id}
```

### auth

#### login

```sh
curl -X POST http://localhost:8080/v0/auth/login \
-H 'Content-Type: application/json' \
-d '{
    "email": "any@example.com",
    "password": "@Any1234"
}'
```

### user

#### create user

```sh
curl -X POST http://localhost:8080/v0/user \
-H 'Content-Type: application/json' \
-d '{
    "email": "any3@gmail.com",
    "name": "Any",
    "password": "@Any1234"
}'
```
