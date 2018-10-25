var express = require('express');
var app = express();

app.use(function(req, res, next) {
      res.header("Access-Control-Allow-Origin", "*");
      res.header("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept");
      next();
});

app.get('/', function (req, res) {
      res.send(
            [
                  {
                        "tank": { "id": 1, "name": "The Mothership" },
                        "temp": { "f": 82.18, "c": 27.88 },
                        "ph": { "val": 8.11, "mv": 500.15 }
                  },
                  {
                        "tank": { "id": 2, "name": "The Pond" },
                        "temp": { "f": 83.1, "c": 28.39 },
                        "ph": { "val": 7.98, "mv": 488.33 }
                  }
            ]);
});

app.listen(3000, function () {
      console.log('Example app listening on port 3000!');
});
