function Table(id, callbackOnClick,  callbackOnCreation, callbackOnSelection, callbackOnDelete) {
  var table = document.getElementById(id);
  if(table === null || table.nodeName !== 'TABLE') throw "Invalid table";

  var currentRow = null;

  table.dynamicTable = this;

  var onClick = Object.prototype.toString.call(callbackOnClick) === '[object Function]' ? callbackOnClick : null;
  var onCreation = Object.prototype.toString.call(callbackOnCreation) === '[object Function]' ? callbackOnCreation : null;
  var onSelection = Object.prototype.toString.call(callbackOnSelection) === '[object Function]' ? callbackOnSelection : null;
  var onDelete = Object.prototype.toString.call(callbackOnDelete) === '[object Function]' ? callbackOnDelete : null;
  if(onDelete !== null) document.body.onkeydown = function(event) { if (event.keyCode == 46) table.dynamicTable.delete(); };

  var _deselect = function(row) {
    if (currentRow !== null && (typeof row === "undefined" || row === currentRow)) {
      if(onSelection !== null)
        onSelection(currentRow, false);

      currentRow.onclick = null;
      currentRow = null;
    }
  };

  var _select = function(row) {
    if (currentRow !== null)
      _deselect();

    currentRow = row;
    if(onClick !== null)
      currentRow.onclick = onClick;

    if (onSelection !== null)
      onSelection(row, true);
  };

  this.add = function(values) {
    if(values instanceof Array) {
      var newRow = table.insertRow(-1);
      newRow.onmouseover = function () { _select(this); }
      newRow.onmouseleave = function () { _deselect(this); };

      for (var i = 0, size = values.length; i < size; i++) {
        var newCell = newRow.insertCell(-1);
        newCell.appendChild(document.createTextNode(String(values[i])));
      }

      if(onCreation !== null)
        onCreation(newRow);

      return true;
    }

    return false;
  };

  this.find = function(search) {
    for (row of table.rows) {
      if(search(row) === true)
        return row;
    }

    return null;
  }
  this.set = function(values, row, column) {
    var row = table.rows[row];

    //If the user wants to insert an array...
    if(values instanceof Array && row !== null) {
      for(var i = 0; i < values.length; i++) row.cells[i].textContent = values[i];
      return true;
    }
    else if(typeof column !== "undefined" && row !== null) {
      row.cells[column].textContent = values;
    }

    return false;
  };

  this.get = function(row, column) {
    var row = table.rows[row];
    if(row !== null) {
      //If the user requestes an specific element ...
      if(typeof column !== "undefined") {
        return row.cells[column].textContent;
      }
      else {
        var result = new Array(row.cells.length);
        for (var i = 0; i < row.cells.length; i++) {
          result[i] = row.cells[i].textContent;
        }
        return result;
      }
    }

    return null;
  };

  this.rows = function() {
    return table.rows.length;
  };

  this.delete = function(row) {
    var target = null;

    if (typeof row === "undefined" && currentRow !== null)
      target = currentRow;
    else if(typeof row["rowIndex"] !== "undefined")
      target = row;
    else
      return false;

    _deselect(typeof row === "undefined" ? undefined : target);
    if(onDelete !== null)
      onDelete(target);
    table.deleteRow(target.rowIndex);
    return true;
  };
}
