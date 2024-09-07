# Rest API with Bevy

This is a template repository to implement a REST API on Bevy.

The main consideration is resource sharing between Bevy and the REST API thread.

The thread is created using `bevy-tokio-tasks` where this tool allows us to create a thread that is out of the Bevy main thread but yet being able to return some values back to the Bevy thread.

One way to allow safe passing and sharing of variables is using `arc` and `mutex` to allow different threads to have access to the same variable using different pointers.