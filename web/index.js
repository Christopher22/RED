document.addEventListener("DOMContentLoaded", function () {

  if(RED === undefined) throw "RED-Javascript-core is missing!";

  var update_button = document.getElementById('update_button');
  if(update_button === null || update_button.nodeName !== "BUTTON") throw "Update button is invalid!";

  var idCache = null;
  var table;

  var downloads = new RED.Manager(function(download) {
    idCache = download.id;
    table.add([download.url, download.path, download.status]);
  }, function(download) {
    table.set([download.url, download.path, download.status], table.find(function(row) { return row.download_id === download.id; }).rowIndex);
  }, function(download) {
    table.delete(table.find(function(row) { return row.download_id === download.id; }));
  });

  table = new Table('download_table', function (event) { //On click
      if(confirm("Do you really want to abort and remove the download?")) {
        downloads.remove(event.currentTarget.download_id);
      }
  }, function (row) { //On creation
    row.download_id = idCache;
  }, function(row, activated) { //On change
    if(activated) row.className += "important_font";
    else row.className = "";
  }, null);

  update_button.onClick = function() {
    downloads.update();
  };

  update_button.disabled = false;
  downloads.update();
  downloads.start();
}, false);
