# Rest API with Bevy

This is a template repository to implement a REST API on Bevy.

The main consideration is resource sharing between Bevy and the REST API thread.

The thread is created using `bevy-tokio-tasks` where this tool allows us to create a thread that is out of the Bevy main thread but yet being able to return some values back to the Bevy thread.

One way to allow safe passing and sharing of variables is using `arc` and `mutex` to allow different threads to have access to the same variable using different pointers.

# Examples to call REST API

```
curl -X POST -d '{"name":"apple","quantity":3}' -H "Content-type: application/json" http://127.0.0.1:3030/v1/groceries
curl -X POST -d '{"name":"pear","quantity":5}' -H "Content-type: application/json" http://127.0.0.1:3030/v1/groceries
curl -X POST -d '{"name":"orange","quantity":8}' -H "Content-type: application/json" http://127.0.0.1:3030/v1/groceries
curl http://127.0.0.1:3030/v1/groceries
```
