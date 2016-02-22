var RED = RED || {
  Download : function(url, path, status, id) {
    this.id = id;
    this.url = url;
    this.path = path;
    this.status = status;
  },
  Manager: function(onCreation, onChange, onRemoval) {
    var downloads = {};
    var timer = null;

    this.remove = function(id) {
      if(typeof downloads[id] !== 'undefined') {
        var xmlhttp = new XMLHttpRequest();
        xmlhttp.open("DELETE", "downloads/" + id);

        xmlhttp.onreadystatechange = function() {
          if (xmlhttp.readyState === XMLHttpRequest.DONE && xmlhttp.status === 200) {
            onRemoval(downloads[id]);
            delete downloads[id];
          }
        };

        xmlhttp.send('{}');
      }
      else {
        return false;
      }
    };

    this.update = function() {
      var xmlhttp = new XMLHttpRequest();
      xmlhttp.open("GET", "downloads/");

      xmlhttp.onreadystatechange = function() {
        if (xmlhttp.readyState == XMLHttpRequest.DONE && xmlhttp.status == 200) {
          var result = JSON.parse(xmlhttp.responseText);

          for(var id in result) {
            var download = downloads[id];
            if(typeof download === 'undefined') {
              downloads[id] = new RED.Download(result[id].Ressource.url, result[id].Ressource.path, result[id].Status, id);
              onCreation(downloads[id]);
            }
            else if(download.url !== result[id].Ressource.url || download.path !== result[id].Ressource.path || download.status !== result[id].Status) {
              download.url = result[id].Ressource.url;
              download.path = result[id].Ressource.path;
              download.status = result[id].Status;
              onChange(download);
            }
          }
        }
      };

      xmlhttp.send();
    }

    this.start = function() {
      timp = timer || window.setInterval(this.update, 500);
    };

    this.stop = function() {
      if(timer !== null) {
        window.clearInterval(timer);
        timer = null;
      }
    };
  },
  add: function(download, callback) {
    if(!(download instanceof RED.Download)) throw "Invalid download";

    var xmlhttp = new XMLHttpRequest();
    xmlhttp.open("POST", "downloads/");

    xmlhttp.onreadystatechange = function() {
      if (xmlhttp.readyState == XMLHttpRequest.DONE) {
        callback(xmlhttp);
      }
    };

    xmlhttp.send(JSON.stringify({
      url: download.url,
      path: download.path
    }));
  }
};
