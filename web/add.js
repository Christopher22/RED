document.addEventListener("DOMContentLoaded", function () {
  if(RED === undefined) throw "RED-Javascript-core is missing!";

  var form = document.getElementById('new_form');
  if(form === null || form.nodeName !== "FORM") throw "Form is invalid!";

  form.onsubmit = function() {
    RED.add(new RED.Download(form.url.value, form.file.value), function(result) {
      if(result.status === 201) {
        var url = document.location.href;
        document.location.href = url.substring(0, url.lastIndexOf("/"));
      }
      else {
        alert("Error during adding!");
      }
    });
    return false;
  };
}, false);
