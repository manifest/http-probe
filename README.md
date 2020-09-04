# HTTP prob application

## How to use

```bash
curl -I http://localhost:8080
```
> Send request to increment the counter.

```javascript
const source = new EventSource("http://localhost:8080/sse", {});
source.addEventListener('message', function(e) {
    var data = JSON.parse(e.data);
    console.log(data.id, data.msg);
}, false);
```
> Subscribe to SSE.
